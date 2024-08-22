#[macro_use]
extern crate napi_derive;

use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::sync::mpsc;

mod config;
mod decode;
mod decode_call;
pub mod preset_query;
mod query;
mod types;

use config::{ClientConfig, StreamConfig};
use query::Query;
use types::{Block, Event, Log, RollbackGuard, Trace, Transaction};

#[napi]
pub struct HypersyncClient {
    inner: Arc<hypersync_client::Client>,
    enable_checksum_addresses: bool,
}

#[napi]
impl HypersyncClient {
    /// Create a new client with given config
    #[napi]
    pub fn new(cfg: Option<ClientConfig>) -> napi::Result<HypersyncClient> {
        env_logger::try_init().ok();

        let cfg = cfg.unwrap_or_default();
        let converted_cfg = cfg.try_convert().context("parse config").map_err(map_err)?;

        let inner = hypersync_client::Client::new(converted_cfg)
            .context("build client")
            .map_err(map_err)?;
        let inner = Arc::new(inner);

        Ok(HypersyncClient {
            inner,
            enable_checksum_addresses: cfg.enable_checksum_addresses.unwrap_or_default(),
        })
    }

    /// Get the height of the source hypersync instance
    #[napi]
    pub async fn get_height(&self) -> napi::Result<i64> {
        let height = self.inner.get_height().await.map_err(map_err)?;

        Ok(height.try_into().unwrap())
    }

    /// Get the chain_id of the source hypersync instance
    #[napi]
    pub async fn get_chain_id(&self) -> napi::Result<i64> {
        let chain_id = self.inner.get_chain_id().await.map_err(map_err)?;

        Ok(chain_id.try_into().unwrap())
    }

    #[napi]
    pub async fn collect(&self, query: Query, config: StreamConfig) -> napi::Result<QueryResponse> {
        let query = query
            .try_convert()
            .context("parse query")
            .map_err(map_err)?;
        let config = config
            .try_convert()
            .context("parse stream config")
            .map_err(map_err)?;

        let resp = self
            .inner
            .clone()
            .collect(query, config)
            .await
            .context("run inner collect")
            .map_err(map_err)?;

        convert_response(resp, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)
    }

    #[napi]
    pub async fn collect_events(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<EventResponse> {
        let query = query
            .try_convert()
            .context("parse query")
            .map_err(map_err)?;
        let config = config
            .try_convert()
            .context("parse stream config")
            .map_err(map_err)?;

        let resp = self
            .inner
            .clone()
            .collect_events(query, config)
            .await
            .context("run inner collect")
            .map_err(map_err)?;

        convert_event_response(resp, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)
    }

    #[napi]
    pub async fn collect_parquet(
        &self,
        path: String,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<()> {
        let query = query
            .try_convert()
            .context("parse query")
            .map_err(map_err)?;
        let config = config
            .try_convert()
            .context("parse stream config")
            .map_err(map_err)?;

        self.inner
            .clone()
            .collect_parquet(&path, query, config)
            .await
            .map_err(map_err)
    }

    #[napi]
    pub async fn get(&self, query: Query) -> napi::Result<QueryResponse> {
        let query = query
            .try_convert()
            .context("parse query")
            .map_err(map_err)?;
        let res = self
            .inner
            .get(&query)
            .await
            .context("run inner query")
            .map_err(map_err)?;
        convert_response(res, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)
    }

    #[napi]
    pub async fn get_events(&self, query: Query) -> napi::Result<EventResponse> {
        let query = query
            .try_convert()
            .context("parse query")
            .map_err(map_err)?;
        let res = self
            .inner
            .get_events(query)
            .await
            .context("run inner query")
            .map_err(map_err)?;
        let r = convert_event_response(res, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)?;
        Ok(r)
    }

    #[napi]
    pub async fn stream(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<QueryResponseStream> {
        let query = query
            .try_convert()
            .context("parse query")
            .map_err(map_err)?;
        let config = config
            .try_convert()
            .context("parse stream config")
            .map_err(map_err)?;

        let inner = self
            .inner
            .clone()
            .stream(query, config)
            .await
            .context("start stream")
            .map_err(map_err)?;

        Ok(QueryResponseStream {
            inner: tokio::sync::Mutex::new(inner),
            enable_checksum_addresses: self.enable_checksum_addresses,
        })
    }

    #[napi]
    pub async fn stream_events(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<EventStream> {
        let query = query
            .try_convert()
            .context("parse query")
            .map_err(map_err)?;
        let config = config
            .try_convert()
            .context("parse stream config")
            .map_err(map_err)?;

        let inner = self
            .inner
            .clone()
            .stream_events(query, config)
            .await
            .context("start stream")
            .map_err(map_err)?;

        Ok(EventStream {
            inner: tokio::sync::Mutex::new(inner),
            enable_checksum_addresses: self.enable_checksum_addresses,
        })
    }
}

#[napi]
pub struct QueryResponseStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<hypersync_client::QueryResponse>>>,
    enable_checksum_addresses: bool,
}

#[napi]
impl QueryResponseStream {
    #[napi]
    pub async fn close(&self) {
        self.inner.lock().await.close();
    }

    #[napi]
    pub async fn recv(&self) -> napi::Result<Option<QueryResponse>> {
        let resp = self.inner.lock().await.recv().await;

        resp.map(|r| {
            convert_response(r?, self.enable_checksum_addresses).context("convert response")
        })
        .transpose()
        .map_err(map_err)
    }
}

type HSEventResponse =
    hypersync_client::QueryResponse<Vec<Vec<hypersync_client::simple_types::Event>>>;

#[napi]
pub struct EventStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<HSEventResponse>>>,
    enable_checksum_addresses: bool,
}

#[napi]
impl EventStream {
    #[napi]
    pub async fn close(&self) {
        self.inner.lock().await.close();
    }

    #[napi]
    pub async fn recv(&self) -> napi::Result<Option<EventResponse>> {
        let resp = self.inner.lock().await.recv().await;

        resp.map(|r| {
            convert_event_response(r?, self.enable_checksum_addresses).context("convert response")
        })
        .transpose()
        .map_err(map_err)
    }
}

#[napi(object)]
pub struct QueryResponseData {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub logs: Vec<Log>,
    pub traces: Vec<Trace>,
}

#[napi(object)]
pub struct QueryResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub data: QueryResponseData,
    /// Rollback guard, supposed to be used to detect rollbacks
    pub rollback_guard: Option<RollbackGuard>,
}

#[napi(object)]
pub struct EventResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub data: Vec<Event>,
    /// Rollback guard, supposed to be used to detect rollbacks
    pub rollback_guard: Option<RollbackGuard>,
}

#[napi(object)]
pub struct Events {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub events: Vec<Event>,
    /// Rollback guard, supposed to be used to detect rollbacks
    pub rollback_guard: Option<RollbackGuard>,
}

fn convert_response(
    res: hypersync_client::QueryResponse,
    should_checksum: bool,
) -> Result<QueryResponse> {
    let blocks = res
        .data
        .blocks
        .iter()
        .flat_map(|b| b.iter().map(|b| Block::from_simple(b, should_checksum)))
        .collect::<Result<Vec<_>>>()
        .context("mapping blocks")?;

    let transactions = res
        .data
        .transactions
        .iter()
        .flat_map(|b| {
            b.iter()
                .map(|tx| Transaction::from_simple(tx, should_checksum))
        })
        .collect::<Result<Vec<_>>>()
        .context("mapping transactions")?;

    let logs = res
        .data
        .logs
        .iter()
        .flat_map(|b| b.iter().map(|l| Log::from_simple(l, should_checksum)))
        .collect::<Result<Vec<_>>>()
        .context("mapping logs")?;

    let traces = res
        .data
        .traces
        .iter()
        .flat_map(|b| b.iter().map(|tr| Trace::from_simple(tr, should_checksum)))
        .collect::<Result<Vec<_>>>()
        .context("mapping traces")?;

    Ok(QueryResponse {
        archive_height: res
            .archive_height
            .map(|h| h.try_into())
            .transpose()
            .context("convert height")?,
        next_block: res.next_block.try_into().context("convert next_block")?,
        total_execution_time: res
            .total_execution_time
            .try_into()
            .context("convert total_execution_time")?,
        data: QueryResponseData {
            blocks,
            transactions,
            logs,
            traces,
        },
        rollback_guard: res
            .rollback_guard
            .map(RollbackGuard::try_convert)
            .transpose()
            .context("convert rollback guard")?,
    })
}

fn convert_event_response(
    resp: hypersync_client::QueryResponse<Vec<Vec<hypersync_client::simple_types::Event>>>,
    should_checksum: bool,
) -> Result<EventResponse> {
    let data = resp
        .data
        .into_iter()
        .flat_map(|batch| {
            batch.into_iter().map(|event| {
                Ok(Event {
                    transaction: event
                        .transaction
                        .map(|v| Transaction::from_simple(&v, should_checksum))
                        .transpose()
                        .context("mapping transaction")?,
                    block: event
                        .block
                        .map(|v| Block::from_simple(&v, should_checksum))
                        .transpose()
                        .context("mapping block")?,
                    log: Log::from_simple(&event.log, should_checksum).context("mapping log")?,
                })
            })
        })
        .collect::<Result<Vec<_>>>()
        .context("mapping response data")?;

    Ok(EventResponse {
        archive_height: resp
            .archive_height
            .map(|v| v.try_into())
            .transpose()
            .context("mapping archive_height")?,
        next_block: resp.next_block.try_into().context("mapping next_block")?,
        total_execution_time: resp
            .total_execution_time
            .try_into()
            .context("mapping total_execution_time")?,
        data,
        rollback_guard: resp
            .rollback_guard
            .map(|rg| RollbackGuard::try_convert(rg).context("convert rollback guard"))
            .transpose()?,
    })
}

fn map_err(e: anyhow::Error) -> napi::Error {
    napi::Error::from_reason(format!("{:?}", e))
}
