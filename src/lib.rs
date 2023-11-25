#[macro_use]
extern crate napi_derive;

use anyhow::{Context, Result};
use from_arrow::FromArrow;

mod config;
mod from_arrow;
mod query;
mod types;

use config::Config;
use query::Query;
use types::{Block, Event, Log, Transaction};

#[napi]
pub struct SkarClient {
    inner: skar_client::Client,
}

#[napi]
impl SkarClient {
    #[napi]
    pub fn new(cfg: Config) -> napi::Result<SkarClient> {
        let cfg = cfg
            .try_convert()
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

        Ok(SkarClient {
            inner: skar_client::Client::new(cfg),
        })
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

    /// Create a parquet file by executing a query.
    ///
    /// If the query can't be finished in a single request, this function will
    ///  keep on making requests using the pagination mechanism (next_block) until
    ///  it reaches the end. It will stream data into the parquet file as it comes from
    ///. the server.
    ///
    /// Path should point to a folder that will contain the parquet files in the end.
    #[napi]
    pub async fn create_parquet_folder(&self, query: Query, path: String) -> napi::Result<()> {
        self.create_parquet_folder_impl(query, path)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn create_parquet_folder_impl(&self, query: Query, path: String) -> Result<()> {
        let query = query.try_convert().context("parse query")?;

        self.inner
            .create_parquet_folder(query, path)
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
}

#[napi(object)]
pub struct QueryResponseData {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub logs: Vec<Log>,
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
}

fn convert_response_to_events(res: skar_client::QueryResponse) -> Result<Events> {
    let blocks = Block::from_batches(&res.data.blocks).context("map blocks from arrow")?;

    let txs =
        Transaction::from_batches(&res.data.transactions).context("map transactions from arrow")?;

    let logs = Log::from_batches(&res.data.logs).context("map logs from arrow")?;

    let mut events = Vec::with_capacity(logs.len());

    for (_, log) in logs.into_iter() {
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
    })
}

fn convert_response_to_query_response(res: skar_client::QueryResponse) -> Result<QueryResponse> {
    let blocks = Block::from_batches(&res.data.blocks)
        .context("map blocks from arrow")?
        .into_values()
        .collect();

    let transactions = Transaction::from_batches(&res.data.transactions)
        .context("map transactions from arrow")?
        .into_values()
        .collect();

    let logs = Log::from_batches(&res.data.logs)
        .context("map logs from arrow")?
        .into_values()
        .collect();

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
    })
}
