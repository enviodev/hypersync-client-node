use alloy_dyn_abi::DynSolValue;
use alloy_primitives::{hex::FromHex, Signed, U256};
use anyhow::{Context, Result};
use hypersync_client::{
    format::{self, Hex},
    net_types, simple_types,
};
use napi::bindgen_prelude::{BigInt, Either4};

pub trait ToChecksummed: Sized {
    fn to_checksummed(self) -> Result<Self>;
}

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

impl ToChecksummed for Event {
    fn to_checksummed(self) -> Result<Self> {
        Ok(Self {
            transaction: self.transaction.to_checksummed().context("transaction")?,
            block: self.block.to_checksummed().context("block")?,
            log: self.log.to_checksummed().context("log")?,
        })
    }
}

/// Evm log object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Default, Clone)]
pub struct Log {
    pub removed: Option<bool>,
    pub log_index: Option<i64>,
    pub transaction_index: Option<i64>,
    pub transaction_hash: Option<String>,
    pub block_hash: Option<String>,
    pub block_number: Option<i64>,
    pub address: Option<String>,
    pub data: Option<String>,
    pub topics: Vec<Option<String>>,
}

impl ToChecksummed for Log {
    fn to_checksummed(self) -> Result<Self> {
        Ok(Self {
            address: self.address.to_checksummed().context("address")?,
            ..self
        })
    }
}

/// Evm transaction object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Default, Clone)]
pub struct Transaction {
    pub block_hash: Option<String>,
    pub block_number: Option<i64>,
    pub from: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub hash: Option<String>,
    pub input: Option<String>,
    pub nonce: Option<String>,
    pub to: Option<String>,
    pub transaction_index: Option<i64>,
    pub value: Option<String>,
    pub v: Option<String>,
    pub r: Option<String>,
    pub s: Option<String>,
    pub y_parity: Option<String>,
    pub max_priority_fee_per_gas: Option<String>,
    pub max_fee_per_gas: Option<String>,
    pub chain_id: Option<i64>,
    pub access_list: Option<Vec<AccessList>>,
    pub max_fee_per_blob_gas: Option<String>,
    pub blob_versioned_hashes: Option<Vec<String>>,
    pub cumulative_gas_used: Option<String>,
    pub effective_gas_price: Option<String>,
    pub gas_used: Option<String>,
    pub contract_address: Option<String>,
    pub logs_bloom: Option<String>,
    pub kind: Option<i64>,
    pub root: Option<String>,
    pub status: Option<i64>,
    pub l1_fee: Option<String>,
    pub l1_gas_price: Option<String>,
    pub l1_gas_used: Option<String>,
    pub l1_fee_scalar: Option<f64>,
    pub gas_used_for_l1: Option<String>,
}

impl ToChecksummed for Transaction {
    fn to_checksummed(self) -> Result<Self> {
        Ok(Self {
            from: self.from.to_checksummed().context("from")?,
            to: self.to.to_checksummed().context("to")?,
            contract_address: self
                .contract_address
                .to_checksummed()
                .context("contract_address")?,
            ..self
        })
    }
}
/// Evm withdrawal object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Withdrawal {
    pub index: Option<String>,
    pub validator_index: Option<String>,
    pub address: Option<String>,
    pub amount: Option<String>,
}

impl From<&format::Withdrawal> for Withdrawal {
    fn from(w: &format::Withdrawal) -> Self {
        Self {
            index: map_binary(&w.index),
            validator_index: map_binary(&w.validator_index),
            address: map_binary(&w.address),
            amount: map_binary(&w.amount),
        }
    }
}

/// Evm access list object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AccessList {
    pub address: Option<String>,
    pub storage_keys: Option<Vec<String>>,
}

impl From<&format::AccessList> for AccessList {
    fn from(a: &format::AccessList) -> Self {
        Self {
            address: map_binary(&a.address),
            storage_keys: a
                .storage_keys
                .as_ref()
                .map(|arr| arr.iter().map(|x| x.encode_hex()).collect()),
        }
    }
}

/// Evm block header object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Default, Clone)]
pub struct Block {
    pub number: Option<i64>,
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
    pub timestamp: Option<String>,
    pub uncles: Option<Vec<String>>,
    pub base_fee_per_gas: Option<String>,
    pub blob_gas_used: Option<String>,
    pub excess_blob_gas: Option<String>,
    pub parent_beacon_block_root: Option<String>,
    pub withdrawals_root: Option<String>,
    pub withdrawals: Option<Vec<Withdrawal>>,
    pub l1_block_number: Option<i64>,
    pub send_count: Option<String>,
    pub send_root: Option<String>,
    pub mix_hash: Option<String>,
}

impl ToChecksummed for Block {
    fn to_checksummed(self) -> Result<Self> {
        Ok(Block {
            miner: self.miner.to_checksummed().context("miner")?,
            ..self
        })
    }
}

impl<T: ToChecksummed> ToChecksummed for Option<T> {
    fn to_checksummed(self) -> Result<Self> {
        Ok(match self {
            Some(v) => Some(v.to_checksummed()?),
            None => None,
        })
    }
}

impl ToChecksummed for String {
    fn to_checksummed(self) -> Result<Self> {
        Ok(alloy_primitives::Address::from_hex(self)
            .context("creating address from hex string")?
            .to_checksum(None))
    }
}

#[cfg(test)]
mod test {
    use alloy_primitives::hex::FromHex;

    #[test]
    fn alloy_addr() {
        let checksum =
            alloy_primitives::Address::from_hex("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")
                .unwrap()
                .to_checksum(None);
        assert_eq!(checksum, "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    }
}

/// Evm trace object
///
/// See ethereum rpc spec for the meaning of fields
#[napi(object)]
#[derive(Default, Clone)]
pub struct Trace {
    pub from: Option<String>,
    pub to: Option<String>,
    pub call_type: Option<String>,
    pub gas: Option<String>,
    pub input: Option<String>,
    pub init: Option<String>,
    pub value: Option<String>,
    pub author: Option<String>,
    pub reward_type: Option<String>,
    pub block_hash: Option<String>,
    pub block_number: Option<i64>,
    pub address: Option<String>,
    pub code: Option<String>,
    pub gas_used: Option<String>,
    pub output: Option<String>,
    pub subtraces: Option<i64>,
    pub trace_address: Option<Vec<i64>>,
    pub transaction_hash: Option<String>,
    pub transaction_position: Option<i64>,
    pub kind: Option<String>,
    pub error: Option<String>,
}

impl ToChecksummed for Trace {
    fn to_checksummed(self) -> Result<Self> {
        Ok(Self {
            from: self.from.to_checksummed().context("from")?,
            to: self.to.to_checksummed().context("to")?,
            author: self.author.to_checksummed().context("author")?,
            address: self.address.to_checksummed().context("address")?,
            ..self
        })
    }
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
            DynSolValue::Int(v, _) => Either4::B(convert_bigint_signed(v)),
            DynSolValue::Uint(v, _) => Either4::B(convert_bigint_unsigned(v)),
            DynSolValue::FixedBytes(bytes, _) => Either4::C(encode_prefix_hex(bytes.as_slice())),
            DynSolValue::Address(addr) => {
                if !checksummed_addresses {
                    Either4::C(encode_prefix_hex(addr.as_slice()))
                } else {
                    Either4::C(addr.to_checksum(None))
                }
            }
            DynSolValue::Function(bytes) => Either4::C(encode_prefix_hex(bytes.as_slice())),
            DynSolValue::Bytes(bytes) => Either4::C(encode_prefix_hex(bytes.as_slice())),
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

fn encode_prefix_hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "0x".into();
    }

    format!("0x{}", faster_hex::hex_string(bytes))
}

fn map_binary<T: Hex>(v: &Option<T>) -> Option<String> {
    v.as_ref().map(|v| v.encode_hex())
}

impl From<&simple_types::Block> for Block {
    fn from(b: &simple_types::Block) -> Self {
        Self {
            number: b.number.map(|n| n.try_into().unwrap()),
            hash: map_binary(&b.hash),
            parent_hash: map_binary(&b.parent_hash),
            nonce: map_binary(&b.nonce),
            sha3_uncles: map_binary(&b.sha3_uncles),
            logs_bloom: map_binary(&b.logs_bloom),
            transactions_root: map_binary(&b.transactions_root),
            state_root: map_binary(&b.state_root),
            receipts_root: map_binary(&b.receipts_root),
            miner: map_binary(&b.miner),
            difficulty: map_binary(&b.difficulty),
            total_difficulty: map_binary(&b.total_difficulty),
            extra_data: map_binary(&b.extra_data),
            size: map_binary(&b.size),
            gas_limit: map_binary(&b.gas_limit),
            gas_used: map_binary(&b.gas_used),
            timestamp: map_binary(&b.timestamp),
            uncles: b
                .uncles
                .as_ref()
                .map(|arr| arr.iter().map(|u| u.encode_hex()).collect()),
            base_fee_per_gas: map_binary(&b.base_fee_per_gas),
            blob_gas_used: map_binary(&b.blob_gas_used),
            excess_blob_gas: map_binary(&b.excess_blob_gas),
            parent_beacon_block_root: map_binary(&b.parent_beacon_block_root),
            withdrawals_root: map_binary(&b.withdrawals_root),
            withdrawals: b
                .withdrawals
                .as_ref()
                .map(|w| w.iter().map(Withdrawal::from).collect()),
            l1_block_number: b.l1_block_number.map(|n| u64::from(n).try_into().unwrap()),
            send_count: map_binary(&b.transactions_root),
            send_root: map_binary(&b.transactions_root),
            mix_hash: map_binary(&b.transactions_root),
        }
    }
}

impl From<&simple_types::Transaction> for Transaction {
    fn from(t: &simple_types::Transaction) -> Self {
        Self {
            block_hash: map_binary(&t.block_hash),
            block_number: t.block_number.map(|n| u64::from(n).try_into().unwrap()),
            from: map_binary(&t.from),
            gas: map_binary(&t.gas),
            gas_price: map_binary(&t.gas_price),
            hash: map_binary(&t.hash),
            input: map_binary(&t.input),
            nonce: map_binary(&t.nonce),
            to: map_binary(&t.to),
            transaction_index: t
                .transaction_index
                .map(|n| u64::from(n).try_into().unwrap()),
            value: map_binary(&t.value),
            v: map_binary(&t.v),
            r: map_binary(&t.r),
            s: map_binary(&t.s),
            y_parity: map_binary(&t.y_parity),
            max_priority_fee_per_gas: map_binary(&t.max_priority_fee_per_gas),
            max_fee_per_gas: map_binary(&t.max_fee_per_gas),
            chain_id: t
                .chain_id
                .as_ref()
                .map(|n| ruint::aliases::U256::from_be_slice(n).try_into().unwrap()),
            access_list: t
                .access_list
                .as_ref()
                .map(|arr| arr.iter().map(AccessList::from).collect()),
            max_fee_per_blob_gas: map_binary(&t.max_fee_per_blob_gas),
            blob_versioned_hashes: t
                .blob_versioned_hashes
                .as_ref()
                .map(|arr| arr.iter().map(|h| h.encode_hex()).collect()),
            cumulative_gas_used: map_binary(&t.cumulative_gas_used),
            effective_gas_price: map_binary(&t.effective_gas_price),
            gas_used: map_binary(&t.gas_used),
            contract_address: map_binary(&t.contract_address),
            logs_bloom: map_binary(&t.logs_bloom),
            kind: t.kind.map(|v| u8::from(v).into()),
            root: map_binary(&t.root),
            status: t.status.map(|v| v.to_u8().into()),
            l1_fee: map_binary(&t.l1_fee),
            l1_gas_price: map_binary(&t.l1_gas_price),
            l1_gas_used: map_binary(&t.l1_gas_used),
            l1_fee_scalar: t.l1_fee_scalar,
            gas_used_for_l1: map_binary(&t.gas_used_for_l1),
        }
    }
}

impl From<&simple_types::Log> for Log {
    fn from(l: &simple_types::Log) -> Self {
        Self {
            removed: l.removed,
            log_index: l.log_index.map(|n| u64::from(n).try_into().unwrap()),
            transaction_index: l
                .transaction_index
                .map(|n| u64::from(n).try_into().unwrap()),
            transaction_hash: map_binary(&l.transaction_hash),
            block_hash: map_binary(&l.block_hash),
            block_number: l.block_number.map(|n| u64::from(n).try_into().unwrap()),
            address: map_binary(&l.address),
            data: map_binary(&l.data),
            topics: l
                .topics
                .iter()
                .map(|t| t.as_ref().map(|v| v.encode_hex()))
                .collect(),
        }
    }
}

impl From<&simple_types::Trace> for Trace {
    fn from(t: &simple_types::Trace) -> Self {
        Self {
            from: map_binary(&t.from),
            to: map_binary(&t.to),
            call_type: t.call_type.clone(),
            gas: map_binary(&t.gas),
            input: map_binary(&t.input),
            init: map_binary(&t.init),
            value: map_binary(&t.value),
            author: map_binary(&t.author),
            reward_type: t.reward_type.clone(),
            block_hash: map_binary(&t.block_hash),
            block_number: t.block_number.map(|n| n.try_into().unwrap()),
            address: map_binary(&t.address),
            code: map_binary(&t.code),
            gas_used: map_binary(&t.gas_used),
            output: map_binary(&t.output),
            subtraces: t.subtraces.map(|n| n.try_into().unwrap()),
            trace_address: t
                .trace_address
                .as_ref()
                .map(|arr| arr.iter().map(|n| (*n).try_into().unwrap()).collect()),
            transaction_hash: map_binary(&t.transaction_hash),
            transaction_position: t.transaction_position.map(|n| n.try_into().unwrap()),
            kind: t.kind.clone(),
            error: t.error.clone(),
        }
    }
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
    pub fn try_convert(arg: net_types::RollbackGuard) -> Result<Self> {
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

fn convert_bigint_signed(v: Signed<256, 4>) -> BigInt {
    let (sign, abs) = v.into_sign_and_abs();
    BigInt {
        sign_bit: sign.is_negative(),
        words: abs.into_limbs().to_vec(),
    }
}

fn convert_bigint_unsigned(v: U256) -> BigInt {
    BigInt {
        sign_bit: false,
        words: v.into_limbs().to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_convert_signed() {
        for i in (i128::from(i64::MIN)..i128::from(u64::MAX))
            .step_by(usize::try_from(u64::MAX / 31).unwrap())
            .take(1024)
        {
            let v = Signed::<256, 4>::try_from(i).unwrap();
            let out = convert_bigint_signed(v);

            assert_eq!(i128::try_from(v).unwrap(), out.get_i128().0);
        }
    }

    #[test]
    fn test_bigint_convert_unsigned() {
        for i in (u128::from(u64::MIN)..u128::MAX)
            .step_by(usize::try_from(u64::MAX / 31).unwrap())
            .take(1024)
        {
            let v = U256::from(i);
            let out = convert_bigint_unsigned(v);

            assert_eq!(u128::try_from(v).unwrap(), out.get_u128().1);
        }
    }
}
