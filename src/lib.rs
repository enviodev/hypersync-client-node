#[macro_use]
extern crate napi_derive;

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use from_arrow::FromArrow;

mod config;
mod decode;
mod from_arrow;
mod query;
mod types;

use config::{Config, ParquetConfig, StreamConfig};
use query::Query;
use skar_format::Hex;
use tokio::sync::mpsc;
use types::{Block, Event, Log, Transaction};

#[napi]
pub struct HypersyncClient {
    inner: skar_client::Client,
}

#[napi]
impl HypersyncClient {
    /// Create a new client with given config
    #[napi]
    pub fn new(cfg: Config) -> napi::Result<HypersyncClient> {
        env_logger::try_init().ok();

        Self::new_impl(cfg).map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    fn new_impl(cfg: Config) -> Result<HypersyncClient> {
        let cfg = cfg.try_convert().context("parse config")?;

        let inner = skar_client::Client::new(cfg).context("build client")?;

        Ok(HypersyncClient { inner })
    }

    /// Get the height of the source hypersync instance
    #[napi]
    pub async fn get_height(&self) -> napi::Result<i64> {
        let height = self
            .inner
            .get_height()
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

        Ok(height.try_into().unwrap())
    }

    /// Stream data from hypersync server concurrently using the given query
    ///
    /// This parallelizes the hypersync queries so will have higher performance compared to
    ///  regular .send methods.
    ///
    /// If query.to_block is not specified, this stream will stop at the block height of the source
    ///  hypersync node. It is not continuous.
    #[napi]
    pub async fn stream(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<QueryResponseStream> {
        self.stream_impl(query, config)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn stream_impl(&self, query: Query, config: StreamConfig) -> Result<QueryResponseStream> {
        let query = query.try_convert().context("parse query")?;
        let config = config.try_convert().context("parse config")?;

        let rx = self
            .inner
            .stream::<skar_client::ArrowIpc>(query, config)
            .await
            .context("start stream")?;

        Ok(QueryResponseStream {
            inner: tokio::sync::Mutex::new(rx),
        })
    }

    /// Stream events data from hypersync server concurrently using the given query
    ///
    /// This parallelizes the hypersync queries so will have higher performance compared to
    ///  regular .send methods.
    ///
    /// If query.to_block is not specified, this stream will stop at the block height of the source
    ///  hypersync node. It is not continuous.
    #[napi]
    pub async fn stream_events(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<EventsStream> {
        self.stream_events_impl(query, config)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn stream_events_impl(&self, query: Query, config: StreamConfig) -> Result<EventsStream> {
        let query = query.try_convert().context("parse query")?;
        let config = config.try_convert().context("parse config")?;

        let rx = self
            .inner
            .stream::<skar_client::ArrowIpc>(query, config)
            .await
            .context("start stream")?;

        Ok(EventsStream {
            inner: tokio::sync::Mutex::new(rx),
        })
    }

    /// Create a parquet file by executing a query.
    ///
    /// Path should point to a folder that will contain the parquet files in the end.
    #[napi]
    pub async fn create_parquet_folder(
        &self,
        query: Query,
        config: ParquetConfig,
    ) -> napi::Result<()> {
        self.create_parquet_folder_impl(query, config)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn create_parquet_folder_impl(&self, query: Query, config: ParquetConfig) -> Result<()> {
        let query = query.try_convert().context("parse query")?;
        let config = config.try_convert().context("parse parquet config")?;

        self.inner
            .create_parquet_folder(query, config)
            .await
            .context("create parquet folder")?;

        Ok(())
    }

    /// Send a query request to the source hypersync instance.
    ///
    /// Returns a query response which contains block, tx and log data.
    #[napi]
    pub async fn send_req(&self, query: Query) -> napi::Result<QueryResponse> {
        self.send_req_impl(query)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn send_req_impl(&self, query: Query) -> Result<QueryResponse> {
        let query = query.try_convert().context("parse query")?;

        let res = self
            .inner
            .send::<skar_client::ArrowIpc>(&query)
            .await
            .context("execute query")?;

        let res =
            convert_response_to_query_response(res).context("convert response to js format")?;

        Ok(res)
    }

    /// Send a event query request to the source hypersync instance.
    ///
    /// This executes the same query as send_req function on the source side but
    /// it groups data for each event(log) so it is easier to process it.
    #[napi]
    pub async fn send_events_req(&self, query: Query) -> napi::Result<Events> {
        self.send_events_req_impl(query)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn send_events_req_impl(&self, query: Query) -> Result<Events> {
        let mut query = query.try_convert().context("parse query")?;

        if !query.field_selection.block.is_empty() {
            for field in BLOCK_JOIN_FIELDS.iter() {
                query.field_selection.block.insert(field.to_string());
            }
        }

        if !query.field_selection.transaction.is_empty() {
            for field in TX_JOIN_FIELDS.iter() {
                query.field_selection.transaction.insert(field.to_string());
            }
        }

        if !query.field_selection.log.is_empty() {
            for field in LOG_JOIN_FIELDS.iter() {
                query.field_selection.log.insert(field.to_string());
            }
        }

        let res = self
            .inner
            .send::<skar_client::ArrowIpc>(&query)
            .await
            .context("execute query")?;
        let res = convert_response_to_events(res).context("convert response to js format")?;

        Ok(res)
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
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<skar_client::QueryResponse>>>,
}

#[napi]
impl QueryResponseStream {
    #[napi]
    pub async fn recv(&self) -> Option<napi::Result<QueryResponse>> {
        self.recv_impl()
            .await
            .map(|v| v.map_err(|e| napi::Error::from_reason(format!("{:?}", e))))
    }

    async fn recv_impl(&self) -> Option<Result<QueryResponse>> {
        self.inner
            .lock()
            .await
            .recv()
            .await
            .map(|resp| convert_response_to_query_response(resp?).context("convert response"))
    }
}

#[napi]
pub struct EventsStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<skar_client::QueryResponse>>>,
}

#[napi]
impl EventsStream {
    #[napi]
    pub async fn recv(&self) -> Option<napi::Result<Events>> {
        self.recv_impl()
            .await
            .map(|v| v.map_err(|e| napi::Error::from_reason(format!("{:?}", e))))
    }

    async fn recv_impl(&self) -> Option<Result<Events>> {
        self.inner
            .lock()
            .await
            .recv()
            .await
            .map(|resp| convert_response_to_events(resp?).context("convert response"))
    }
}

#[napi(object)]
pub struct QueryResponseData {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub logs: Vec<Log>,
}

#[napi(object)]
pub struct RollbackGuard {
    /// Block number of the last scanned block
    pub block_number: i64,
    /// Block timestamp of the last scanned block
    pub timestamp: i64,
    /// Block hash of the last scanned block
    pub hash: String,
    /// Block number of the first scanned block in memory.
    ///
    /// This might not be the first scanned block. It only includes blocks that are in memory (possible to be rolled back).
    pub first_block_number: i64,
    /// Parent hash of the first scanned block in memory.
    ///
    /// This might not be the first scanned block. It only includes blocks that are in memory (possible to be rolled back).
    pub first_parent_hash: String,
}

impl RollbackGuard {
    fn try_convert(arg: skar_net_types::RollbackGuard) -> Result<Self> {
        Ok(Self {
            block_number: arg
                .block_number
                .try_into()
                .context("convert block_number")?,
            timestamp: arg.timestamp,
            hash: arg.hash.encode_hex(),
            first_block_number: arg
                .first_block_number
                .try_into()
                .context("convert first_block_number")?,
            first_parent_hash: arg.first_parent_hash.encode_hex(),
        })
    }
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

const BLOCK_JOIN_FIELDS: &[&str] = &["number"];
const TX_JOIN_FIELDS: &[&str] = &["block_number", "transaction_index"];
const LOG_JOIN_FIELDS: &[&str] = &["log_index", "transaction_index", "block_number"];

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

fn convert_response_to_events(res: skar_client::QueryResponse) -> Result<Events> {
    let mut blocks = BTreeMap::new();

    for batch in res.data.blocks.iter() {
        let data = Block::from_arrow(batch).context("map blocks from arrow")?;

        for block in data {
            blocks.insert(block.number, block);
        }
    }

    let mut txs = BTreeMap::new();

    for batch in res.data.transactions.iter() {
        let data = Transaction::from_arrow(batch).context("map transactions from arrow")?;

        for tx in data {
            txs.insert((tx.block_number, tx.transaction_index), tx);
        }
    }

    let logs = res
        .data
        .logs
        .iter()
        .map(Log::from_arrow)
        .collect::<Result<Vec<_>>>()
        .context("map logs from arrow")?
        .concat();

    let mut events = Vec::with_capacity(logs.len());

    for log in logs.into_iter() {
        let transaction = txs.get(&(log.block_number, log.transaction_index)).cloned();
        let block = blocks.get(&log.block_number).cloned();

        events.push(Event {
            log,
            block,
            transaction,
        })
    }

    Ok(Events {
        archive_height: res.archive_height.map(|h| h.try_into().unwrap()),
        next_block: res.next_block.try_into().unwrap(),
        total_execution_time: res.total_execution_time.try_into().unwrap(),
        events,
        rollback_guard: res
            .rollback_guard
            .map(RollbackGuard::try_convert)
            .transpose()
            .context("convert rollback guard")?,
    })
}

fn convert_response_to_query_response(res: skar_client::QueryResponse) -> Result<QueryResponse> {
    let blocks = res
        .data
        .blocks
        .iter()
        .map(Block::from_arrow)
        .collect::<Result<Vec<_>>>()
        .context("map blocks from arrow")?
        .concat();

    let transactions = res
        .data
        .transactions
        .iter()
        .map(Transaction::from_arrow)
        .collect::<Result<Vec<_>>>()
        .context("map transactions from arrow")?
        .concat();

    let logs = res
        .data
        .logs
        .iter()
        .map(Log::from_arrow)
        .collect::<Result<Vec<_>>>()
        .context("map logs from arrow")?
        .concat();

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
        },
        rollback_guard: res
            .rollback_guard
            .map(RollbackGuard::try_convert)
            .transpose()
            .context("convert rollback guard")?,
    })
}
