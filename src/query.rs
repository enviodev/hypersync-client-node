use anyhow::{Context, Result};
use serde::Serialize;

#[napi(object)]
#[derive(Default, Clone, Serialize)]
pub struct LogSelection {
    pub address: Vec<String>,
    pub topics: Vec<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Clone, Serialize)]
pub struct TransactionSelection {
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub sighash: Vec<String>,
    pub status: Option<i64>,
}

#[napi(object)]
#[derive(Default, Clone, Serialize)]
pub struct FieldSelection {
    pub block: Vec<String>,
    pub transaction: Vec<String>,
    pub log: Vec<String>,
}

#[napi(object)]
#[derive(Default, Clone, Serialize)]
pub struct Query {
    pub from_block: i64,
    pub to_block: Option<i64>,
    pub logs: Vec<LogSelection>,
    pub transactions: Vec<TransactionSelection>,
    pub include_all_blocks: bool,
    pub field_selection: FieldSelection,
    pub max_num_blocks: Option<i64>,
    pub max_num_transactions: Option<i64>,
    pub max_num_logs: Option<i64>,
}

impl Query {
    pub fn try_convert(&self) -> Result<skar_net_types::Query> {
        let json = serde_json::to_vec(self).context("serialize to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}
