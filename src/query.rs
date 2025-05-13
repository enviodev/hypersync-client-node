use anyhow::{Context, Result};
use hypersync_client::net_types;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LogSelection {
    /// Address of the contract, any logs that has any of these addresses will be returned.
    /// Empty means match all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    /// Topics to match, each member of the top level array is another array, if the nth topic matches any
    ///  topic specified in topics[n] the log will be returned. Empty means match all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Vec<String>>>,
}

#[napi(object)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TransactionSelection {
    /// Address the transaction should originate from. If transaction.from matches any of these, the transaction
    ///  will be returned. Keep in mind that this has an and relationship with to filter, so each transaction should
    ///  match both of them. Empty means match all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    /// Address the transaction should go to. If transaction.to matches any of these, the transaction will
    ///  be returned. Keep in mind that this has an and relationship with from filter, so each transaction should
    ///  match both of them. Empty means match all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    /// If first 4 bytes of transaction input matches any of these, transaction will be returned. Empty means match all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sighash: Option<Vec<String>>,
    /// If tx.status matches this it will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// If transaction.type matches any of these values, the transaction will be returned
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Vec<u8>>,
    // If transaction.contract_address matches any of these values, the transaction will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<Vec<String>>,

    /// If transaction.authorization_list matches any of these values, the transaction will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_list: Option<Vec<AuthorizationSelection>>,
}

#[napi(object)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct AuthorizationSelection {
    /// List of chain ids to match in the transaction authorizationList
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<Vec<i64>>,
    /// List of addresses to match in the transaction authorizationList
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FieldSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<Vec<BlockField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Vec<TransactionField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<LogField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Vec<TraceField>>,
}

#[napi(string_enum)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockField {
    Number,
    Hash,
    ParentHash,
    Nonce,
    Sha3Uncles,
    LogsBloom,
    TransactionsRoot,
    StateRoot,
    ReceiptsRoot,
    Miner,
    Difficulty,
    TotalDifficulty,
    ExtraData,
    Size,
    GasLimit,
    GasUsed,
    Timestamp,
    Uncles,
    BaseFeePerGas,
    BlobGasUsed,
    ExcessBlobGas,
    ParentBeaconBlockRoot,
    WithdrawalsRoot,
    Withdrawals,
    L1BlockNumber,
    SendCount,
    SendRoot,
    MixHash,
}

#[napi(string_enum)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionField {
    BlockHash,
    BlockNumber,
    From,
    Gas,
    GasPrice,
    Hash,
    Input,
    Nonce,
    To,
    TransactionIndex,
    Value,
    V,
    R,
    S,
    YParity,
    MaxPriorityFeePerGas,
    MaxFeePerGas,
    ChainId,
    AccessList,
    AuthorizationList,
    MaxFeePerBlobGas,
    BlobVersionedHashes,
    CumulativeGasUsed,
    EffectiveGasPrice,
    GasUsed,
    ContractAddress,
    LogsBloom,
    #[serde(rename = "type")]
    Kind,
    Root,
    Status,
    L1Fee,
    L1GasPrice,
    L1GasUsed,
    L1FeeScalar,
    GasUsedForL1,
}

#[napi(string_enum)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogField {
    Removed,
    LogIndex,
    TransactionIndex,
    TransactionHash,
    BlockHash,
    BlockNumber,
    Address,
    Data,
    Topic0,
    Topic1,
    Topic2,
    Topic3,
}

#[napi(string_enum)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceField {
    From,
    To,
    CallType,
    Gas,
    Input,
    Init,
    Value,
    Author,
    RewardType,
    BlockHash,
    BlockNumber,
    Address,
    Code,
    GasUsed,
    Output,
    Subtraces,
    TraceAddress,
    TransactionHash,
    TransactionPosition,
    Kind,
    Error,
}

#[napi(object)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TraceSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sighash: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BlockSelection {
    /// Hash of a block, any blocks that have one of these hashes will be returned.
    /// Empty means match all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<Vec<String>>,
    /// Miner address of a block, any blocks that have one of these miners will be returned.
    /// Empty means match all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miner: Option<Vec<String>>,
}

#[napi]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum JoinMode {
    Default,
    JoinAll,
    JoinNothing,
}

impl Default for JoinMode {
    fn default() -> Self {
        Self::Default
    }
}

#[napi(object)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Query {
    /// The block to start the query from
    pub from_block: i64,
    /// The block to end the query at. If not specified, the query will go until the
    ///  end of data. Exclusive, the returned range will be [from_block..to_block).
    ///
    /// The query will return before it reaches this target block if it hits the time limit
    ///  configured on the server. The user should continue their query by putting the
    ///  next_block field in the response into from_block field of their next query. This implements
    ///  pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<i64>,
    /// List of log selections, these have an or relationship between them, so the query will return logs
    /// that match any of these selections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<LogSelection>>,
    /// List of transaction selections, the query will return transactions that match any of these selections and
    ///  it will return transactions that are related to the returned logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<TransactionSelection>>,
    /// List of trace selections, the query will return traces that match any of these selections and
    ///  it will re turn traces that are related to the returned logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<TraceSelection>>,
    /// List of block selections, the query will return blocks that match any of these selections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<BlockSelection>>,
    /// Weather to include all blocks regardless of if they are related to a returned transaction or log. Normally
    ///  the server will return only the blocks that are related to the transaction or logs in the response. But if this
    ///  is set to true, the server will return data for all blocks in the requested range [from_block, to_block).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_blocks: Option<bool>,
    /// Field selection. The user can select which fields they are interested in, requesting less fields will improve
    ///  query execution time and reduce the payload size so the user should always use a minimal number of fields.
    pub field_selection: FieldSelection,
    /// Maximum number of blocks that should be returned, the server might return more blocks than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_blocks: Option<i64>,
    /// Maximum number of transactions that should be returned, the server might return more transactions than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_transactions: Option<i64>,
    /// Maximum number of logs that should be returned, the server might return more logs than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_logs: Option<i64>,
    /// Maximum number of traces that should be returned, the server might return more traces than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_traces: Option<i64>,
    /// Selects join mode for the query,
    /// Default: join in this order logs -> transactions -> traces -> blocks
    /// JoinAll: join everything to everything. For example if logSelection matches log0, we get the
    /// associated transaction of log0 and then we get associated logs of that transaction as well. Applites similarly
    /// to blocks, traces.
    /// JoinNothing: join nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_mode: Option<JoinMode>,
}

impl Query {
    pub fn try_convert(&self) -> Result<net_types::Query> {
        let json = serde_json::to_vec(self).context("serialize to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}

impl TryFrom<net_types::Query> for Query {
    type Error = anyhow::Error;

    fn try_from(skar_query: net_types::Query) -> Result<Self> {
        let json = serde_json::to_vec(&skar_query).context("serialize query to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}
