use alloy_dyn_abi::DynSolValue;
use napi::bindgen_prelude::{BigInt, Either4};

/// Data relating to a single event (log)
#[napi(object)]
#[derive(Default, Clone)]
pub struct Event {
    /// Transaction that triggered this event
    pub transaction: Option<Transaction>,
    /// Block that this event happened in
    pub block: Option<Block>,
    /// Evm log data
    pub log: Log,
}

/// Evm log object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Default, Clone)]
pub struct Log {
    pub removed: Option<bool>,
    pub log_index: i64,
    pub transaction_index: i64,
    pub transaction_hash: Option<String>,
    pub block_hash: Option<String>,
    pub block_number: i64,
    pub address: Option<String>,
    pub data: Option<String>,
    pub topics: Vec<Option<String>>,
}

/// Evm transaction object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Default, Clone)]
pub struct Transaction {
    pub block_hash: Option<String>,
    pub block_number: i64,
    pub from: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub hash: Option<String>,
    pub input: Option<String>,
    pub nonce: Option<String>,
    pub to: Option<String>,
    pub transaction_index: i64,
    pub value: Option<String>,
    pub v: Option<String>,
    pub r: Option<String>,
    pub s: Option<String>,
    pub max_priority_fee_per_gas: Option<String>,
    pub max_fee_per_gas: Option<String>,
    pub chain_id: Option<String>,
    pub cumulative_gas_used: Option<String>,
    pub effective_gas_price: Option<String>,
    pub gas_used: Option<String>,
    pub contract_address: Option<String>,
    pub logs_bloom: Option<String>,
    pub kind: Option<u32>,
    pub root: Option<String>,
    pub status: Option<u32>,
}

/// Evm block header object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Default, Clone)]
pub struct Block {
    pub number: i64,
    pub hash: Option<String>,
    pub parent_hash: Option<String>,
    pub nonce: Option<String>,
    pub sha3_uncles: Option<String>,
    pub logs_bloom: Option<String>,
    pub transactions_root: Option<String>,
    pub state_root: Option<String>,
    pub receipts_root: Option<String>,
    pub miner: Option<String>,
    pub difficulty: Option<String>,
    pub total_difficulty: Option<String>,
    pub extra_data: Option<String>,
    pub size: Option<String>,
    pub gas_limit: Option<String>,
    pub gas_used: Option<String>,
    pub timestamp: Option<i64>,
    pub base_fee_per_gas: Option<String>,
}

/// Decoded EVM log
#[napi(object)]
#[derive(Default)]
pub struct DecodedEvent {
    pub indexed: Vec<DecodedSolValue>,
    pub body: Vec<DecodedSolValue>,
}

#[napi(object)]
#[derive(Clone)]
pub struct DecodedSolValue {
    pub val: Either4<bool, BigInt, String, Vec<DecodedSolValue>>,
}

impl DecodedSolValue {
    pub fn new(val: DynSolValue, checksummed_addresses: bool) -> Self {
        let val = match val {
            DynSolValue::Bool(b) => Either4::A(b),
            DynSolValue::Int(v, _) => Either4::B(BigInt {
                sign_bit: v.is_negative(),
                words: v.into_limbs().to_vec(),
            }),
            DynSolValue::Uint(v, _) => Either4::B(BigInt {
                sign_bit: false,
                words: v.into_limbs().to_vec(),
            }),
            DynSolValue::FixedBytes(bytes, _) => Either4::C(prefix_hex::encode(bytes.as_slice())),
            DynSolValue::Address(addr) => {
                if !checksummed_addresses {
                    Either4::C(prefix_hex::encode(addr.as_slice()))
                } else {
                    Either4::C(addr.to_checksum(None))
                }
            }
            DynSolValue::Function(bytes) => Either4::C(prefix_hex::encode(bytes.as_slice())),
            DynSolValue::Bytes(bytes) => Either4::C(prefix_hex::encode(bytes.as_slice())),
            DynSolValue::String(s) => Either4::C(s),
            DynSolValue::Array(vals) => Either4::D(
                vals.into_iter()
                    .map(|v| DecodedSolValue::new(v, checksummed_addresses))
                    .collect(),
            ),
            DynSolValue::FixedArray(vals) => Either4::D(
                vals.into_iter()
                    .map(|v| DecodedSolValue::new(v, checksummed_addresses))
                    .collect(),
            ),
            DynSolValue::Tuple(vals) => Either4::D(
                vals.into_iter()
                    .map(|v| DecodedSolValue::new(v, checksummed_addresses))
                    .collect(),
            ),
        };

        Self { val }
    }
}
