use anyhow::Context;
use hypersync_client::{
    format::{Address, Hex, LogArgument},
    preset_query,
};

use crate::{map_err, query::Query};

/// Returns a query for all Blocks and Transactions within the block range (from_block, to_block]
/// If to_block is None then query runs to the head of the chain.
#[napi]
pub fn preset_query_blocks_and_transactions(
    from_block: i64,
    to_block: Option<i64>,
) -> napi::Result<Query> {
    let from_block = from_block
        .try_into()
        .context("convert from_block")
        .map_err(map_err)?;
    let to_block = to_block
        .map(|t| t.try_into().context("convert to_block"))
        .transpose()
        .map_err(map_err)?;

    let query: Query = preset_query::blocks_and_transactions(from_block, to_block)
        .try_into()
        .map_err(map_err)?;

    Ok(query)
}

/// Returns a query object for all Blocks and hashes of the Transactions within the block range
/// (from_block, to_block].  Also returns the block_hash and block_number fields on each Transaction
/// so it can be mapped to a block.  If to_block is None then query runs to the head of the chain.
#[napi]
pub fn preset_query_blocks_and_transaction_hashes(
    from_block: i64,
    to_block: Option<i64>,
) -> napi::Result<Query> {
    let from_block = from_block
        .try_into()
        .context("convert from_block")
        .map_err(map_err)?;
    let to_block = to_block
        .map(|t| t.try_into().context("convert to_block"))
        .transpose()
        .map_err(map_err)?;

    let query: Query = preset_query::blocks_and_transaction_hashes(from_block, to_block)
        .try_into()
        .map_err(map_err)?;

    Ok(query)
}

/// Returns a query object for all Logs within the block range from the given address.
/// If to_block is None then query runs to the head of the chain.
#[napi]
pub fn preset_query_logs(
    contract_address: String,
    from_block: i64,
    to_block: Option<i64>,
) -> napi::Result<Query> {
    let address = Address::decode_hex(&contract_address)
        .context("parse address")
        .map_err(map_err)?;

    let from_block = from_block
        .try_into()
        .context("convert from_block")
        .map_err(map_err)?;
    let to_block = to_block
        .map(|t| t.try_into().context("convert to_block"))
        .transpose()
        .map_err(map_err)?;

    preset_query::logs(from_block, to_block, address)
        .try_into()
        .map_err(map_err)
}

/// Returns a query for all Logs within the block range from the given address with a
/// matching topic0 event signature.  Topic0 is the keccak256 hash of the event signature.
/// If to_block is None then query runs to the head of the chain.
#[napi]
pub fn preset_query_logs_of_event(
    contract_address: String,
    topic0: String,
    from_block: i64,
    to_block: Option<i64>,
) -> napi::Result<Query> {
    let address = Address::decode_hex(&contract_address)
        .context("parse address")
        .map_err(map_err)?;
    let topic0 = LogArgument::decode_hex(&topic0)
        .context("parse topic0")
        .map_err(map_err)?;

    let from_block = from_block
        .try_into()
        .context("convert from_block")
        .map_err(map_err)?;
    let to_block = to_block
        .map(|t| t.try_into().context("convert to_block"))
        .transpose()
        .map_err(map_err)?;

    let query = preset_query::logs_of_event(from_block, to_block, topic0, address)
        .try_into()
        .map_err(map_err)?;
    Ok(query)
}
