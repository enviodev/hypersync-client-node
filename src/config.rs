use std::collections::HashMap;

use anyhow::{Context, Result};
use serde::Serialize;

#[napi(object)]
#[derive(Default, Clone, Serialize)]
pub struct StreamConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_mapping: Option<ColumnMapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_output: Option<HexOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_batch_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_blocks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_transactions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_logs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_traces: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_bytes_ceiling: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_bytes_floor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
}

#[napi(string_enum)]
#[derive(Default, Debug, Serialize, Clone, Copy)]
pub enum HexOutput {
    #[default]
    NoEncode,
    Prefixed,
    NonPrefixed,
}

#[napi(string_enum)]
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Float64,
    Float32,
    UInt64,
    UInt32,
    Int64,
    Int32,
}

#[napi(object)]
#[derive(Default, Clone, Serialize)]
pub struct ColumnMapping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<HashMap<String, DataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<HashMap<String, DataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<HashMap<String, DataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<HashMap<String, DataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_log: Option<HashMap<String, DataType>>,
}

impl StreamConfig {
    pub fn try_convert(&self) -> Result<hypersync_client::StreamConfig> {
        let json = serde_json::to_vec(self).context("serialize to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct ClientConfig {
    pub url: String,
    pub api_token: String,
    pub http_req_timeout_millis: Option<i64>,
    pub max_num_retries: Option<i64>,
    pub retry_backoff_ms: Option<i64>,
    pub retry_base_ms: Option<i64>,
    pub retry_ceiling_ms: Option<i64>,
    pub enable_checksum_addresses: Option<bool>,
    pub serialization_format: Option<SerializationFormat>,
    pub enable_query_caching: Option<bool>,
}

impl From<ClientConfig> for hypersync_client::ClientConfig {
    fn from(config: ClientConfig) -> Self {
        use hypersync_client::ClientConfig as Cfg;
        let serialization_format = match config.serialization_format.unwrap_or_default() {
            SerializationFormat::Json => hypersync_client::SerializationFormat::Json,
            SerializationFormat::CapnProto => {
                let should_cache_queries = config.enable_query_caching.unwrap_or_default();
                hypersync_client::SerializationFormat::CapnProto {
                    should_cache_queries,
                }
            }
        };
        Self {
            url: config.url,
            api_token: config.api_token,
            http_req_timeout_millis: config
                .http_req_timeout_millis
                .map_or(Cfg::default_http_req_timeout_millis(), |v| v as u64),
            max_num_retries: config
                .max_num_retries
                .map_or(Cfg::default_max_num_retries(), |v| v as usize),
            retry_backoff_ms: config
                .retry_backoff_ms
                .map_or(Cfg::default_retry_backoff_ms(), |v| v as u64),
            retry_base_ms: config
                .retry_base_ms
                .map_or(Cfg::default_retry_base_ms(), |v| v as u64),
            retry_ceiling_ms: config
                .retry_ceiling_ms
                .map_or(Cfg::default_retry_ceiling_ms(), |v| v as u64),
            serialization_format,
        }
    }
}

#[napi(string_enum)]
#[derive(Default, Clone, Serialize)]
pub enum SerializationFormat {
    #[default]
    Json,
    CapnProto,
}
