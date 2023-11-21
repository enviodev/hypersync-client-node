use std::collections::BTreeMap;

use anyhow::{Context, Result};
use arrow2::array::{BinaryArray, BooleanArray, UInt64Array, UInt8Array};
use skar_client::ArrowBatch;

use crate::types::{Block, Log, Transaction};

pub trait FromArrow: Sized {
  type Key: Ord;

  fn from_arrow(batch: &ArrowBatch) -> Result<Vec<(Self::Key, Self)>>;

  fn from_batches(batches: &[ArrowBatch]) -> Result<BTreeMap<Self::Key, Self>> {
    let mut out = BTreeMap::new();

    for batch in batches.iter() {
      let part = Self::from_arrow(batch)?;
      out.extend(part.into_iter());
    }

    Ok(out)
  }
}

impl FromArrow for Log {
  type Key = (i64, i64);

  fn from_arrow(batch: &ArrowBatch) -> Result<Vec<(Self::Key, Self)>> {
    let block_number = batch
      .column::<UInt64Array>("block_number")
      .context("get column")?;
    let log_index = batch
      .column::<UInt64Array>("log_index")
      .context("get column")?;
    let tx_index = batch
      .column::<UInt64Array>("transaction_index")
      .context("get column")?;

    let mut out = Vec::with_capacity(block_number.len());

    for ((&block_number, &log_index), &tx_index) in block_number
      .values_iter()
      .zip(log_index.values_iter())
      .zip(tx_index.values_iter())
    {
      let block_number = block_number.try_into().unwrap();
      let log_index = log_index.try_into().unwrap();
      let transaction_index = tx_index.try_into().unwrap();

      out.push((
        (block_number, log_index),
        Log {
          block_number,
          log_index,
          transaction_index,
          ..Default::default()
        },
      ));
    }

    if let Ok(col) = batch.column::<BooleanArray>("removed") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.removed = val;
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("transaction_hash") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.transaction_hash = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("block_hash") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.block_hash = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("address") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.address = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("data") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.data = val.map(prefix_hex::encode);
      }
    }

    for topic_name in ["topic0", "topic1", "topic2", "topic3"] {
      if let Ok(col) = batch.column::<BinaryArray<i32>>(topic_name) {
        for (target, val) in out.iter_mut().zip(col.iter()) {
          target.1.topics.push(val.map(prefix_hex::encode));
        }
      }
    }

    Ok(out)
  }
}

// pub removed: Option<bool>,
//   pub log_index: Option<i64>,
//   pub transaction_index: Option<i64>,
//   pub transaction_hash: Option<Buffer>,
//   pub block_hash: Option<Buffer>,
//   pub block_number: Option<i64>,
//   pub address: Option<Buffer>,
//   pub data: Option<Buffer>,
//   pub topics: Option<Vec<Buffer>>,

impl FromArrow for Transaction {
  type Key = (i64, i64);

  fn from_arrow(batch: &ArrowBatch) -> Result<Vec<(Self::Key, Self)>> {
    let block_number = batch
      .column::<UInt64Array>("block_number")
      .context("get column")?;
    let tx_index = batch
      .column::<UInt64Array>("transaction_index")
      .context("get column")?;

    let mut out = Vec::with_capacity(block_number.len());

    for (&block_number, &tx_index) in block_number.values_iter().zip(tx_index.values_iter()) {
      let block_number = block_number.try_into().unwrap();
      let transaction_index = tx_index.try_into().unwrap();

      out.push((
        (block_number, transaction_index),
        Transaction {
          block_number,
          transaction_index,
          ..Default::default()
        },
      ));
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("block_hash") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.block_hash = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("from") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.from = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("gas") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.gas = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("gas_price") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.gas_price = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("hash") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.hash = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("input") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.input = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("nonce") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.nonce = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("to") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.to = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("value") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.value = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("v") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.v = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("r") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.r = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("s") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.s = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("max_priority_fee_per_gas") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.max_priority_fee_per_gas = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("max_fee_per_gas") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.max_fee_per_gas = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("chain_id") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.chain_id = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("cumulative_gas_used") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.cumulative_gas_used = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("effective_gas_price") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.effective_gas_price = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("gas_used") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.gas_used = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("contract_address") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.contract_address = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("logs_bloom") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.logs_bloom = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<UInt8Array>("kind") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.kind = val.map(|&v| v.into());
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("root") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.root = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<UInt8Array>("status") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.status = val.map(|&v| v.into());
      }
    }

    Ok(out)
  }
}

impl FromArrow for Block {
  type Key = i64;

  fn from_arrow(batch: &ArrowBatch) -> Result<Vec<(Self::Key, Self)>> {
    let number = batch
      .column::<UInt64Array>("number")
      .context("get column")?;

    let mut out = Vec::with_capacity(number.len());

    for &number in number.values_iter() {
      let number = number.try_into().unwrap();

      out.push((
        number,
        Block {
          number,
          ..Default::default()
        },
      ));
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("hash") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.hash = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("parent_hash") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.parent_hash = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("nonce") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.nonce = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("sha3_uncles") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.sha3_uncles = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("logs_bloom") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.logs_bloom = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("transactions_root") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.transactions_root = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("state_root") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.state_root = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("receipts_root") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.receipts_root = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("miner") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.miner = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("difficulty") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.difficulty = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("total_difficulty") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.total_difficulty = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("extra_data") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.extra_data = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("size") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.size = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("gas_limit") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.gas_limit = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("gas_used") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.gas_used = val.map(prefix_hex::encode);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("timestamp") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.timestamp = val.map(i64_from_bytes);
      }
    }

    if let Ok(col) = batch.column::<BinaryArray<i32>>("base_fee_per_gas") {
      for (target, val) in out.iter_mut().zip(col.iter()) {
        target.1.base_fee_per_gas = val.map(prefix_hex::encode);
      }
    }

    Ok(out)
  }
}

fn i64_from_bytes(v: &[u8]) -> i64 {
  assert!(v.len() <= std::mem::size_of::<i64>());
  let mut buf = [0; std::mem::size_of::<i64>()];
  buf[std::mem::size_of::<i64>() - v.len()..].copy_from_slice(v);
  i64::from_be_bytes(buf)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_i64_from_bytes() {
    let v: Vec<u8> = prefix_hex::decode("0x5bbc001f").unwrap();
    let v = i64_from_bytes(&v);

    assert_eq!(v, 1539047455);
  }
}
