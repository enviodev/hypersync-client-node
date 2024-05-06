#[macro_use]
extern crate napi_derive;

use std::sync::Arc;

use anyhow::{Context, Result};

mod config;
mod decode;
pub mod preset_query;
mod query;
mod types;

use config::{ClientConfig, StreamConfig};
use query::Query;
use tokio::sync::mpsc;
use types::{Block, Event, Log, RollbackGuard, Trace, Transaction};

#[napi]
pub struct HypersyncClient {
    inner: Arc<hypersync_client::Client>,
}

#[napi]
impl HypersyncClient {
    /// Create a new client with given config
    #[napi]
    pub fn new(cfg: Option<ClientConfig>) -> Result<HypersyncClient> {
        env_logger::try_init().ok();

        let cfg = cfg
            .unwrap_or_default()
            .try_convert()
            .context("parse config")?;

        let inner = hypersync_client::Client::new(cfg).context("build client")?;
        let inner = Arc::new(inner);

        Ok(HypersyncClient { inner })
    }

    /// Get the height of the source hypersync instance
    #[napi]
    pub async fn get_height(&self) -> napi::Result<i64> {
        let height = self.inner.get_height().await.map_err(napi::Error::from)?;

        Ok(height.try_into().unwrap())
    }

    #[napi]
    pub async fn collect(&self, query: Query, config: StreamConfig) -> napi::Result<QueryResponse> {
        let query = query.try_convert().context("parse query")?;
        let config = config.try_convert().context("parse stream config")?;

        let resp = self
            .inner
            .clone()
            .collect(query, config)
            .await
            .context("run inner collect")?;

        convert_response(resp)
            .context("convert response")
            .map_err(napi::Error::from)
    }

    #[napi]
    pub async fn collect_events(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<EventResponse> {
        let query = query.try_convert().context("parse query")?;
        let config = config.try_convert().context("parse stream config")?;

        let resp = self
            .inner
            .clone()
            .collect_events(query, config)
            .await
            .context("run inner collect")?;

        convert_event_response(resp)
            .context("convert response")
            .map_err(napi::Error::from)
    }

    #[napi]
    pub async fn collect_parquet(
        &self,
        path: String,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<()> {
        let query = query.try_convert().context("parse query")?;
        let config = config.try_convert().context("parse stream config")?;

        self.inner
            .clone()
            .collect_parquet(&path, query, config)
            .await
            .map_err(napi::Error::from)
    }

    #[napi]
    pub async fn get(&self, query: Query) -> napi::Result<QueryResponse> {
        let query = query.try_convert().context("parse query")?;
        let res = self.inner.get(&query).await.context("run inner query")?;
        convert_response(res)
            .context("convert response")
            .map_err(napi::Error::from)
    }

    #[napi]
    pub async fn get_events(&self, query: Query) -> napi::Result<EventResponse> {
        let query = query.try_convert().context("parse query")?;
        let res = self
            .inner
            .get_events(query)
            .await
            .context("run inner query")?;
        let r = convert_event_response(res).context("convert response")?;
        Ok(r)
    }

    #[napi]
    pub async fn stream(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<QueryResponseStream> {
        let query = query.try_convert().context("parse query")?;
        let config = config.try_convert().context("parse stream config")?;

        let inner = self
            .inner
            .clone()
            .stream(query, config)
            .await
            .context("start stream")?;

        Ok(QueryResponseStream {
            inner: tokio::sync::Mutex::new(inner),
        })
    }

    #[napi]
    pub async fn stream_events(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<EventStream> {
        let mut query = query.try_convert().context("parse query")?;

        query.logs = query
            .logs
            .into_iter()
            .map(|log| {
                let mut log = log;
                let address = log.address.clone();
                if address.into_iter().all(|v| v.as_ref()[0..19] == [0u8; 19]) {
                    log.address = vec![];
                }
                log
            })
            .collect();

        let config = config.try_convert().context("parse stream config")?;

        let inner = self
            .inner
            .clone()
            .stream_events(query, config)
            .await
            .context("start stream")?;

        Ok(EventStream {
            inner: tokio::sync::Mutex::new(inner),
        })
    }
}

#[napi]
pub struct QueryResponseStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<hypersync_client::QueryResponse>>>,
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

        resp.map(|r| convert_response(r?).context("convert response"))
            .transpose()
            .map_err(Into::into)
    }
}

type HSEventResponse =
    hypersync_client::QueryResponse<Vec<Vec<hypersync_client::simple_types::Event>>>;

#[napi]
pub struct EventStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<HSEventResponse>>>,
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

        resp.map(|r| convert_event_response(r?).context("convert response"))
            .transpose()
            .map_err(Into::into)
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

fn convert_response(res: hypersync_client::QueryResponse) -> Result<QueryResponse> {
    let blocks = res
        .data
        .blocks
        .iter()
        .flat_map(|b| b.iter().map(Block::from))
        .collect::<Vec<_>>();

    let transactions = res
        .data
        .transactions
        .iter()
        .flat_map(|b| b.iter().map(Transaction::from))
        .collect::<Vec<_>>();

    let logs = res
        .data
        .logs
        .iter()
        .flat_map(|b| b.iter().map(Log::from))
        .collect::<Vec<_>>();

    let traces = res
        .data
        .traces
        .iter()
        .flat_map(|b| b.iter().map(Trace::from))
        .collect::<Vec<_>>();

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
) -> Result<EventResponse> {
    let mut data = Vec::new();

    for batch in resp.data {
        for event in batch {
            data.push(Event {
                transaction: event.transaction.map(|v| Transaction::from(&*v)),
                block: event.block.map(|v| Block::from(&*v)),
                log: Log::from(&event.log),
            });
        }
    }

    Ok(EventResponse {
        archive_height: resp.archive_height.map(|v| v.try_into().unwrap()),
        next_block: resp.next_block.try_into().unwrap(),
        total_execution_time: resp.total_execution_time.try_into().unwrap(),
        data,
        rollback_guard: resp
            .rollback_guard
            .map(|rg| RollbackGuard::try_convert(rg).context("convert rollback guard"))
            .transpose()?,
    })
}
