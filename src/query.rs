use anyhow::{Context, Result};
use hypersync_client::net_types;
use napi::bindgen_prelude::Either;

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct LogFilter {
    /// Address of the contract, any logs that has any of these addresses will be returned.
    /// Empty means match all.
    pub address: Option<Vec<String>>,
    /// Topics to match, each member of the top level array is another array, if the nth topic matches any
    ///  topic specified in topics[n] the log will be returned. Empty means match all.
    pub topics: Option<Vec<Vec<String>>>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct LogSelection {
    pub include: LogFilter,
    pub exclude: Option<LogFilter>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct TransactionFilter {
    /// Address the transaction should originate from. If transaction.from matches any of these, the transaction
    ///  will be returned. Keep in mind that this has an and relationship with to filter, so each transaction should
    ///  match both of them. Empty means match all.
    pub from: Option<Vec<String>>,
    /// Address the transaction should go to. If transaction.to matches any of these, the transaction will
    ///  be returned. Keep in mind that this has an and relationship with from filter, so each transaction should
    ///  match both of them. Empty means match all.
    pub to: Option<Vec<String>>,
    /// If first 4 bytes of transaction input matches any of these, transaction will be returned. Empty means match all.
    pub sighash: Option<Vec<String>>,
    /// If tx.status matches this it will be returned.
    pub status: Option<i64>,
    /// If transaction.type matches any of these values, the transaction will be returned
    #[napi(js_name = "type")]
    pub type_: Option<Vec<u8>>,
    // If transaction.contract_address matches any of these values, the transaction will be returned.
    pub contract_address: Option<Vec<String>>,
    /// If transaction.hash matches any of these values, the transaction will be returned.
    /// Empty means match all.
    pub hash: Option<Vec<String>>,

    /// If transaction.authorization_list matches any of these values, the transaction will be returned.
    pub authorization_list: Option<Vec<AuthorizationSelection>>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct TransactionSelection {
    pub include: TransactionFilter,
    pub exclude: Option<TransactionFilter>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct AuthorizationSelection {
    /// List of chain ids to match in the transaction authorizationList
    pub chain_id: Option<Vec<i64>>,
    /// List of addresses to match in the transaction authorizationList
    pub address: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct FieldSelection {
    pub block: Option<Vec<BlockField>>,
    pub transaction: Option<Vec<TransactionField>>,
    pub log: Option<Vec<LogField>>,
    pub trace: Option<Vec<TraceField>>,
}

#[napi(string_enum)]
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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
    Type,
    Root,
    Status,
    L1Fee,
    L1GasPrice,
    L1GasUsed,
    L1FeeScalar,
    GasUsedForL1,
}

#[napi(string_enum)]
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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
    Type,
    Error,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct TraceFilter {
    pub from: Option<Vec<String>>,
    pub to: Option<Vec<String>>,
    pub address: Option<Vec<String>>,
    pub call_type: Option<Vec<String>>,
    pub reward_type: Option<Vec<String>>,
    #[napi(js_name = "type")]
    pub type_: Option<Vec<String>>,
    pub sighash: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct TraceSelection {
    pub include: TraceFilter,
    pub exclude: Option<TraceFilter>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct BlockFilter {
    /// Hash of a block, any blocks that have one of these hashes will be returned.
    /// Empty means match all.
    pub hash: Option<Vec<String>>,
    /// Miner address of a block, any blocks that have one of these miners will be returned.
    /// Empty means match all.
    pub miner: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Clone, Debug)]
pub struct BlockSelection {
    pub include: BlockFilter,
    pub exclude: Option<BlockFilter>,
}

#[napi]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum JoinMode {
    #[default]
    Default,
    JoinAll,
    JoinNothing,
}

#[napi(object)]
#[derive(Default, Clone)]
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
    pub to_block: Option<i64>,
    /// List of log selections, these have an or relationship between them, so the query will return logs
    /// that match any of these selections.
    pub logs: Option<Vec<Either<LogFilter, LogSelection>>>,
    /// List of transaction selections, the query will return transactions that match any of these selections and
    ///  it will return transactions that are related to the returned logs.
    pub transactions: Option<Vec<Either<TransactionFilter, TransactionSelection>>>,
    /// List of trace selections, the query will return traces that match any of these selections and
    ///  it will re turn traces that are related to the returned logs.
    pub traces: Option<Vec<Either<TraceFilter, TraceSelection>>>,
    /// List of block selections, the query will return blocks that match any of these selections
    pub blocks: Option<Vec<Either<BlockFilter, BlockSelection>>>,
    /// Weather to include all blocks regardless of if they are related to a returned transaction or log. Normally
    ///  the server will return only the blocks that are related to the transaction or logs in the response. But if this
    ///  is set to true, the server will return data for all blocks in the requested range [from_block, to_block).
    pub include_all_blocks: Option<bool>,
    /// Field selection. The user can select which fields they are interested in, requesting less fields will improve
    ///  query execution time and reduce the payload size so the user should always use a minimal number of fields.
    pub field_selection: FieldSelection,
    /// Maximum number of blocks that should be returned, the server might return more blocks than this number but
    ///  it won't overshoot by too much.
    pub max_num_blocks: Option<i64>,
    /// Maximum number of transactions that should be returned, the server might return more transactions than this number but
    ///  it won't overshoot by too much.
    pub max_num_transactions: Option<i64>,
    /// Maximum number of logs that should be returned, the server might return more logs than this number but
    ///  it won't overshoot by too much.
    pub max_num_logs: Option<i64>,
    /// Maximum number of traces that should be returned, the server might return more traces than this number but
    ///  it won't overshoot by too much.
    pub max_num_traces: Option<i64>,
    /// Selects join mode for the query,
    /// Default: join in this order logs -> transactions -> traces -> blocks
    /// JoinAll: join everything to everything. For example if logSelection matches log0, we get the
    /// associated transaction of log0 and then we get associated logs of that transaction as well. Applites similarly
    /// to blocks, traces.
    /// JoinNothing: join nothing.
    pub join_mode: Option<JoinMode>,
}

impl Query {
    pub fn try_convert(&self) -> Result<net_types::Query> {
        todo!()
        // let json = serde_json::to_vec(self).context("serialize to json")?;
        // serde_json::from_slice(&json).context("parse json")
    }
}

impl TryFrom<net_types::Query> for Query {
    type Error = anyhow::Error;

    fn try_from(_skar_query: net_types::Query) -> Result<Self> {
        todo!()
        // let json = serde_json::to_vec(&skar_query).context("serialize query to json")?;
        // serde_json::from_slice(&json).context("parse json")
    }
}

impl TryFrom<LogFilter> for net_types::LogFilter {
    type Error = anyhow::Error;

    fn try_from(filter: LogFilter) -> Result<net_types::LogFilter> {
        use arrayvec::ArrayVec;
        use hypersync_client::format::{Address, LogArgument};

        let address = if let Some(addresses) = filter.address {
            addresses
                .into_iter()
                .map(|addr_str| Address::try_from(addr_str.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| anyhow::anyhow!("Failed to parse address: {}", e))?
        } else {
            Vec::new()
        };

        let mut topics = ArrayVec::new();
        if let Some(topic_vecs) = filter.topics {
            for (i, topic_vec) in topic_vecs.into_iter().enumerate() {
                if i >= 4 {
                    anyhow::bail!("Log filter has more than 4 topics");
                }
                let parsed_topics = topic_vec
                    .into_iter()
                    .map(|topic_str| LogArgument::try_from(topic_str.as_str()))
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .context("Failed to parse topic")?;
                topics.push(parsed_topics);
            }
        }

        Ok(net_types::LogFilter {
            address,
            address_filter: None,
            topics,
        })
    }
}

impl TryFrom<LogSelection> for net_types::LogSelection {
    type Error = anyhow::Error;

    fn try_from(selection: LogSelection) -> Result<net_types::LogSelection> {
        let include = net_types::LogFilter::try_from(selection.include)?;
        let exclude = selection
            .exclude
            .map(net_types::LogFilter::try_from)
            .transpose()?;

        Ok(net_types::LogSelection { include, exclude })
    }
}

impl TryFrom<AuthorizationSelection> for net_types::AuthorizationSelection {
    type Error = anyhow::Error;

    fn try_from(selection: AuthorizationSelection) -> Result<net_types::AuthorizationSelection> {
        use hypersync_client::format::Address;

        let chain_id = selection.chain_id.unwrap_or_default().into_iter().map(|id| id as u64).collect();
        
        let address = if let Some(addresses) = selection.address {
            addresses
                .into_iter()
                .map(|addr_str| Address::try_from(addr_str.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("Failed to parse authorization address")?
        } else {
            Vec::new()
        };

        Ok(net_types::AuthorizationSelection {
            chain_id,
            address,
        })
    }
}

impl TryFrom<TransactionFilter> for net_types::TransactionFilter {
    type Error = anyhow::Error;

    fn try_from(filter: TransactionFilter) -> Result<net_types::TransactionFilter> {
        use hypersync_client::format::{Address, Hash};
        use hypersync_client::net_types::Sighash;

        let from = if let Some(addresses) = filter.from {
            addresses
                .into_iter()
                .map(|addr_str| Address::try_from(addr_str.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("Failed to parse from address")?
        } else {
            Vec::new()
        };

        let to = if let Some(addresses) = filter.to {
            addresses
                .into_iter()
                .map(|addr_str| Address::try_from(addr_str.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("Failed to parse to address")?
        } else {
            Vec::new()
        };

        let sighash = if let Some(sighashes) = filter.sighash {
            sighashes
                .into_iter()
                .map(|sig_str| Sighash::try_from(sig_str.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("Failed to parse sighash")?
        } else {
            Vec::new()
        };

        let status = filter.status.map(|s| s as u8);

        let type_ = filter.type_.unwrap_or_default();

        let contract_address = if let Some(addresses) = filter.contract_address {
            addresses
                .into_iter()
                .map(|addr_str| Address::try_from(addr_str.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("Failed to parse contract address")?
        } else {
            Vec::new()
        };

        let hash = if let Some(hashes) = filter.hash {
            hashes
                .into_iter()
                .map(|hash_str| Hash::try_from(hash_str.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("Failed to parse hash")?
        } else {
            Vec::new()
        };

        let authorization_list = if let Some(auth_list) = filter.authorization_list {
            auth_list
                .into_iter()
                .map(net_types::AuthorizationSelection::try_from)
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("Failed to convert authorization list")?
        } else {
            Vec::new()
        };

        Ok(net_types::TransactionFilter {
            from,
            from_filter: None,
            to,
            to_filter: None,
            sighash,
            status,
            type_,
            contract_address,
            contract_address_filter: None,
            hash,
            authorization_list,
        })
    }
}

impl TryFrom<TransactionSelection> for net_types::TransactionSelection {
    type Error = anyhow::Error;

    fn try_from(selection: TransactionSelection) -> Result<net_types::TransactionSelection> {
        let include = net_types::TransactionFilter::try_from(selection.include)?;
        let exclude = selection
            .exclude
            .map(net_types::TransactionFilter::try_from)
            .transpose()?;

        Ok(net_types::TransactionSelection { include, exclude })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_filter_conversion() {
        let filter = LogFilter {
            address: Some(vec![
                "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()
            ]),
            topics: Some(vec![vec![
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),
            ]]),
        };

        let converted = net_types::LogFilter::try_from(filter).expect("conversion should succeed");

        assert_eq!(converted.address.len(), 1);
        assert_eq!(converted.topics.len(), 1);
        assert_eq!(converted.topics[0].len(), 1);
        assert!(converted.address_filter.is_none());
    }

    #[test]
    fn test_log_selection_conversion() {
        let selection = LogSelection {
            include: LogFilter {
                address: Some(vec![
                    "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()
                ]),
                topics: None,
            },
            exclude: Some(LogFilter {
                address: Some(vec![
                    "0x0000000000000000000000000000000000000000".to_string()
                ]),
                topics: None,
            }),
        };

        let converted =
            net_types::LogSelection::try_from(selection).expect("conversion should succeed");

        assert_eq!(converted.include.address.len(), 1);
        assert!(converted.exclude.is_some());
        assert_eq!(converted.exclude.unwrap().address.len(), 1);
    }

    #[test]
    fn test_log_filter_empty_collections() {
        let filter = LogFilter {
            address: None,
            topics: None,
        };

        let converted = net_types::LogFilter::try_from(filter).expect("conversion should succeed");

        assert!(converted.address.is_empty());
        assert!(converted.topics.is_empty());
    }

    #[test]
    fn test_transaction_filter_conversion() {
        let filter = TransactionFilter {
            from: Some(vec!["0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()]),
            to: Some(vec!["0xa0b86a33e6c11c8c0c5c0b5e6adee30d1a234567".to_string()]),
            sighash: Some(vec!["0xa9059cbb".to_string()]),
            status: Some(1),
            type_: Some(vec![2]),
            contract_address: Some(vec!["0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".to_string()]),
            hash: Some(vec!["0x40d008f2a1653f09b7b028d30c7fd1ba7c84900fcfb032040b3eb3d16f84d294".to_string()]),
            authorization_list: None,
        };

        let converted = net_types::TransactionFilter::try_from(filter).expect("conversion should succeed");

        assert_eq!(converted.from.len(), 1);
        assert_eq!(converted.to.len(), 1);
        assert_eq!(converted.sighash.len(), 1);
        assert_eq!(converted.status, Some(1));
        assert_eq!(converted.type_.len(), 1);
        assert_eq!(converted.type_[0], 2);
        assert_eq!(converted.contract_address.len(), 1);
        assert_eq!(converted.hash.len(), 1);
        assert!(converted.authorization_list.is_empty());
        assert!(converted.from_filter.is_none());
        assert!(converted.to_filter.is_none());
    }

    #[test]
    fn test_transaction_selection_conversion() {
        let selection = TransactionSelection {
            include: TransactionFilter {
                from: Some(vec!["0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()]),
                to: None,
                sighash: None,
                status: None,
                type_: None,
                contract_address: None,
                hash: None,
                authorization_list: None,
            },
            exclude: Some(TransactionFilter {
                status: Some(0),
                from: None,
                to: None,
                sighash: None,
                type_: None,
                contract_address: None,
                hash: None,
                authorization_list: None,
            }),
        };

        let converted = net_types::TransactionSelection::try_from(selection).expect("conversion should succeed");

        assert_eq!(converted.include.from.len(), 1);
        assert!(converted.exclude.is_some());
        let exclude = converted.exclude.unwrap();
        assert_eq!(exclude.status, Some(0));
    }

    #[test]
    fn test_authorization_selection_conversion() {
        let auth_selection = AuthorizationSelection {
            chain_id: Some(vec![1, 137, 42161]),
            address: Some(vec!["0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()]),
        };

        let converted = net_types::AuthorizationSelection::try_from(auth_selection).expect("conversion should succeed");

        assert_eq!(converted.chain_id.len(), 3);
        assert_eq!(converted.chain_id, vec![1, 137, 42161]);
        assert_eq!(converted.address.len(), 1);
    }

    #[test]
    fn test_transaction_filter_empty_collections() {
        let filter = TransactionFilter {
            from: None,
            to: None,
            sighash: None,
            status: None,
            type_: None,
            contract_address: None,
            hash: None,
            authorization_list: None,
        };

        let converted = net_types::TransactionFilter::try_from(filter).expect("conversion should succeed");

        assert!(converted.from.is_empty());
        assert!(converted.to.is_empty());
        assert!(converted.sighash.is_empty());
        assert!(converted.status.is_none());
        assert!(converted.type_.is_empty());
        assert!(converted.contract_address.is_empty());
        assert!(converted.hash.is_empty());
        assert!(converted.authorization_list.is_empty());
    }

    #[test]
    fn test_transaction_filter_hash_field() {
        let filter = TransactionFilter {
            from: None,
            to: None,
            sighash: None,
            status: None,
            type_: None,
            contract_address: None,
            hash: Some(vec![
                "0x40d008f2a1653f09b7b028d30c7fd1ba7c84900fcfb032040b3eb3d16f84d294".to_string(),
                "0x88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6".to_string(),
            ]),
            authorization_list: None,
        };

        let converted = net_types::TransactionFilter::try_from(filter).expect("conversion should succeed");

        assert_eq!(converted.hash.len(), 2);
        assert!(converted.from.is_empty());
        assert!(converted.to.is_empty());
        assert!(converted.sighash.is_empty());
        assert!(converted.contract_address.is_empty());
        assert!(converted.authorization_list.is_empty());
    }
}
