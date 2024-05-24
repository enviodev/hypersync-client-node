#[macro_use]
extern crate napi_derive;

use std::sync::Arc;

use anyhow::{Context, Result};

mod config;
mod decode;
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
    pub fn new(cfg: ClientConfig) -> Result<HypersyncClient> {
        env_logger::try_init().ok();

        let cfg = cfg.try_convert().context("parse config")?;

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
        let query = query.try_convert().context("parse query")?;
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

    // /// Returns a query for all Blocks and Transactions within the block range (from_block, to_block]
    // /// If to_block is None then query runs to the head of the chain.
    // #[napi]
    // pub fn preset_query_blocks_and_transactions(
    //     &self,
    //     from_block: u32,
    //     to_block: Option<u32>,
    // ) -> napi::Result<Query> {
    //     let query: Query = skar_client::Client::preset_query_blocks_and_transactions(
    //         from_block.into(),
    //         to_block.map(|u| u.into()),
    //     )
    //     .try_into()
    //     .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

    //     Ok(query)
    // }

    // /// Returns a query object for all Blocks and hashes of the Transactions within the block range
    // /// (from_block, to_block].  Also returns the block_hash and block_number fields on each Transaction
    // /// so it can be mapped to a block.  If to_block is None then query runs to the head of the chain.
    // #[napi]
    // pub fn preset_query_blocks_and_transaction_hashes(
    //     &self,
    //     from_block: u32,
    //     to_block: Option<u32>,
    // ) -> napi::Result<Query> {
    //     let query: Query = skar_client::Client::preset_query_blocks_and_transaction_hashes(
    //         from_block.into(),
    //         to_block.map(|u| u.into()),
    //     )
    //     .try_into()
    //     .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

    //     Ok(query)
    // }

    // /// Returns a query object for all Logs within the block range from the given address.
    // /// If to_block is None then query runs to the head of the chain.
    // #[napi]
    // pub fn preset_query_logs(
    //     &self,
    //     contract_address: String,
    //     from_block: u32,
    //     to_block: Option<u32>,
    // ) -> napi::Result<Query> {
    //     // cut the "0x" off the address
    //     let address: &str = if &contract_address[..2] == "0x" {
    //         &contract_address[2..]
    //     } else {
    //         &contract_address
    //     };
    //     let address = hex_str_address_to_byte_array(address)
    //         .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
    //     let query: Query = skar_client::Client::preset_query_logs(
    //         from_block.into(),
    //         to_block.map(|u| u.into()),
    //         address,
    //     )
    //     .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?
    //     .try_into()
    //     .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
    //     Ok(query)
    // }

    // /// Returns a query for all Logs within the block range from the given address with a
    // /// matching topic0 event signature.  Topic0 is the keccak256 hash of the event signature.
    // /// If to_block is None then query runs to the head of the chain.
    // #[napi]
    // pub fn preset_query_logs_of_event(
    //     &self,
    //     contract_address: String,
    //     topic0: String,
    //     from_block: u32,
    //     to_block: Option<u32>,
    // ) -> napi::Result<Query> {
    //     // cut the "0x" off the address
    //     let address: &str = if &contract_address[..2] == "0x" {
    //         &contract_address[2..]
    //     } else {
    //         &contract_address
    //     };
    //     let address = hex_str_address_to_byte_array(address)
    //         .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

    //     // cut the "0x" off the topic0
    //     let topic0: &str = if &topic0[..2] == "0x" {
    //         &topic0[2..]
    //     } else {
    //         &topic0
    //     };
    //     let topic0 = hex_str_topic0_to_byte_array(topic0)
    //         .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

    //     let query: Query = skar_client::Client::preset_query_logs_of_event(
    //         from_block.into(),
    //         to_block.map(|u| u.into()),
    //         topic0,
    //         address,
    //     )
    //     .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?
    //     .try_into()
    //     .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
    //     Ok(query)
    // }
}

// // helper function to decode hex string as address
// fn hex_str_address_to_byte_array(hex_str: &str) -> Result<[u8; 20], String> {
//     if hex_str.len() != 40 {
//         return Err(format!("address must be 40 hex characters"));
//     }

//     let mut dst = [0u8; 20];
//     match faster_hex::hex_decode(hex_str.as_bytes(), &mut dst) {
//         Ok(()) => Ok(dst),
//         Err(e) => Err(format!("Failed to decode hex string: {}", e)),
//     }
// }

// // helper function to decode hex string as topic0
// fn hex_str_topic0_to_byte_array(hex_str: &str) -> Result<[u8; 32], String> {
//     if hex_str.len() != 64 {
//         return Err(format!("topic0 must be 64 hex characters"));
//     }

//     let mut dst = [0u8; 32];
//     match faster_hex::hex_decode(hex_str.as_bytes(), &mut dst) {
//         Ok(()) => Ok(dst),
//         Err(e) => Err(format!("Failed to decode hex string: {}", e)),
//     }
// }

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
