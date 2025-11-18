use std::collections::HashMap;

#[napi(object)]
#[derive(Default, Clone)]
pub struct StreamConfig {
    pub column_mapping: Option<ColumnMapping>,
    pub event_signature: Option<String>,
    pub hex_output: Option<HexOutput>,
    pub batch_size: Option<i64>,
    pub max_batch_size: Option<i64>,
    pub min_batch_size: Option<i64>,
    pub concurrency: Option<i64>,
    pub max_num_blocks: Option<i64>,
    pub max_num_transactions: Option<i64>,
    pub max_num_logs: Option<i64>,
    pub max_num_traces: Option<i64>,
    pub response_bytes_ceiling: Option<i64>,
    pub response_bytes_floor: Option<i64>,
    pub reverse: Option<bool>,
}

#[napi(string_enum)]
#[derive(Default, Debug, Clone, Copy)]
pub enum HexOutput {
    #[default]
    NoEncode,
    Prefixed,
    NonPrefixed,
}

#[napi(string_enum)]
#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Float64,
    Float32,
    UInt64,
    UInt32,
    Int64,
    Int32,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct ColumnMapping {
    pub block: Option<HashMap<String, DataType>>,
    pub transaction: Option<HashMap<String, DataType>>,
    pub log: Option<HashMap<String, DataType>>,
    pub trace: Option<HashMap<String, DataType>>,
    pub decoded_log: Option<HashMap<String, DataType>>,
}

impl From<HexOutput> for hypersync_client::HexOutput {
    fn from(hex_output: HexOutput) -> Self {
        match hex_output {
            HexOutput::NoEncode => hypersync_client::HexOutput::NoEncode,
            HexOutput::Prefixed => hypersync_client::HexOutput::Prefixed,
            HexOutput::NonPrefixed => hypersync_client::HexOutput::NonPrefixed,
        }
    }
}

impl From<DataType> for hypersync_client::DataType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Float64 => hypersync_client::DataType::Float64,
            DataType::Float32 => hypersync_client::DataType::Float32,
            DataType::UInt64 => hypersync_client::DataType::UInt64,
            DataType::UInt32 => hypersync_client::DataType::UInt32,
            DataType::Int64 => hypersync_client::DataType::Int64,
            DataType::Int32 => hypersync_client::DataType::Int32,
        }
    }
}

impl From<ColumnMapping> for hypersync_client::ColumnMapping {
    fn from(mapping: ColumnMapping) -> Self {
        use std::collections::BTreeMap;

        fn to_btreemap(
            hm: Option<HashMap<String, DataType>>,
        ) -> BTreeMap<String, hypersync_client::DataType> {
            hm.unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()
        }

        hypersync_client::ColumnMapping {
            block: to_btreemap(mapping.block),
            transaction: to_btreemap(mapping.transaction),
            log: to_btreemap(mapping.log),
            trace: to_btreemap(mapping.trace),
            decoded_log: to_btreemap(mapping.decoded_log),
        }
    }
}

impl From<StreamConfig> for hypersync_client::StreamConfig {
    fn from(config: StreamConfig) -> Self {
        use hypersync_client::StreamConfig as Cfg;

        hypersync_client::StreamConfig {
            column_mapping: config.column_mapping.map(Into::into),
            event_signature: config.event_signature,
            hex_output: config.hex_output.map(Into::into).unwrap_or_default(),
            batch_size: config
                .batch_size
                .map_or(Cfg::default_batch_size(), |v| v as u64),
            max_batch_size: config
                .max_batch_size
                .map_or(Cfg::default_max_batch_size(), |v| v as u64),
            min_batch_size: config
                .min_batch_size
                .map_or(Cfg::default_min_batch_size(), |v| v as u64),
            concurrency: config
                .concurrency
                .map_or(Cfg::default_concurrency(), |v| v as usize),
            max_num_blocks: config.max_num_blocks.map(|v| v as usize),
            max_num_transactions: config.max_num_transactions.map(|v| v as usize),
            max_num_logs: config.max_num_logs.map(|v| v as usize),
            max_num_traces: config.max_num_traces.map(|v| v as usize),
            response_bytes_ceiling: config
                .response_bytes_ceiling
                .map_or(Cfg::default_response_bytes_ceiling(), |v| v as u64),
            response_bytes_floor: config
                .response_bytes_floor
                .map_or(Cfg::default_response_bytes_floor(), |v| v as u64),
            reverse: config.reverse.unwrap_or_default(),
        }
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
#[derive(Default, Clone)]
pub enum SerializationFormat {
    #[default]
    Json,
    CapnProto,
}
