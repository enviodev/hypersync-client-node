use crate::{column_mapping::ColumnMapping, ArrowChunk};
use anyhow::{anyhow, Context, Result};
use arrow2::datatypes::SchemaRef;
use serde::{Deserialize, Serialize};
use skar_net_types::RollbackGuard;

#[derive(Debug, Clone)]
pub struct QueryResponseData {
    pub blocks: Vec<ArrowBatch>,
    pub transactions: Vec<ArrowBatch>,
    pub logs: Vec<ArrowBatch>,
    pub traces: Vec<ArrowBatch>,
}

#[derive(Debug, Clone)]
pub struct QueryResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<u64>,
    /// Next block to query for, the responses are paginated so
    /// the caller should continue the query from this block if they
    /// didn't get responses up to the to_block they specified in the Query.
    pub next_block: u64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: u64,
    /// Response data
    pub data: QueryResponseData,
    /// Rollback guard
    pub rollback_guard: Option<RollbackGuard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    #[serde(default = "default_batch_size")]
    /// Block range size to use when making individual requests.
    pub batch_size: u64,
    #[serde(default = "default_concurrency")]
    /// Controls the number of concurrent requests made to hypersync server.
    pub concurrency: usize,
    /// Requests are retried forever internally if this param is set to true.
    #[serde(default)]
    pub retry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParquetConfig {
    /// Path to write parquet files to
    pub path: String,
    /// Define type mapping for output columns
    #[serde(default)]
    pub column_mapping: ColumnMapping,
    /// Event signature to parse the logs with. example: Transfer(address indexed from, address indexed to, uint256 amount)
    pub event_signature: Option<String>,
    /// Convert binary output columns to hex
    #[serde(default)]
    pub hex_output: bool,
    #[serde(default = "default_batch_size")]
    /// Block range size to use when making individual requests.
    pub batch_size: u64,
    #[serde(default = "default_concurrency")]
    /// Controls the number of concurrent requests made to hypersync server.
    pub concurrency: usize,
    /// Requests are retried forever internally if this param is set to true.
    #[serde(default)]
    pub retry: bool,
}

fn default_batch_size() -> u64 {
    400
}

fn default_concurrency() -> usize {
    10
}

#[derive(Debug, Clone)]
pub struct ArrowBatch {
    pub chunk: ArrowChunk,
    pub schema: SchemaRef,
}

impl ArrowBatch {
    pub fn column<T: 'static>(&self, name: &str) -> Result<&T> {
        match self
            .schema
            .fields
            .iter()
            .enumerate()
            .find(|(_, f)| f.name == name)
        {
            Some((idx, _)) => {
                let col = self
                    .chunk
                    .columns()
                    .get(idx)
                    .context("get column")?
                    .as_any()
                    .downcast_ref::<T>()
                    .context("cast column type")?;
                Ok(col)
            }
            None => Err(anyhow!("field {} not found in schema", name)),
        }
    }
}
