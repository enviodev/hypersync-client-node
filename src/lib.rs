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

  #[napi]
  pub async fn get_height(&self) -> napi::Result<i64> {
    let height = self
      .inner
      .get_height()
      .await
      .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

    Ok(height.try_into().unwrap())
  }

  #[napi]
  pub async fn send_events_req(&self, query: Query) -> napi::Result<Events> {
    self
      .send_events_req_impl(query)
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

const BLOCK_JOIN_FIELDS: &[&str] = &["number"];
const TX_JOIN_FIELDS: &[&str] = &["block_number", "transaction_index"];
const LOG_JOIN_FIELDS: &[&str] = &["log_index", "transaction_index", "block_number"];

#[napi(object)]
pub struct Events {
  pub archive_height: Option<i64>,
  pub next_block: i64,
  pub total_execution_time: i64,
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
