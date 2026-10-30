use crate::map_err;
use crate::types::{DecodedSolValue, Trace, Transaction};
use alloy_dyn_abi::JsonAbiExt;
use alloy_json_abi::Function;
use anyhow::{Context, Result};
use hypersync_client::format::{Data, Hex};
use std::collections::HashMap;
use std::sync::Arc;

type Selector = [u8; 4];

/// Decoder for Ethereum function calls
#[napi]
#[derive(Clone)]
pub struct CallDecoder {
    // Decoding is done here rather than with hypersync_client::CallDecoder,
    // which decodes the calldata without stripping the selector (garbage
    // output) and panics on input shorter than 4 bytes.
    functions: Arc<HashMap<Selector, Function>>,
    checksummed_addresses: bool,
}

fn parse_signatures(signatures: &[String]) -> Result<HashMap<Selector, Function>> {
    signatures
        .iter()
        .map(|sig| {
            let function = Function::parse(sig)
                .with_context(|| format!("parse function signature {sig:?}"))?;
            Ok((function.selector().0, function))
        })
        .collect()
}

#[napi]
impl CallDecoder {
    /// Create call decoder from function signatures
    #[napi]
    pub fn from_signatures(signatures: Vec<String>) -> napi::Result<CallDecoder> {
        Self::from_signatures_with_checksum(signatures, false)
    }

    /// Create call decoder from function signatures with checksum option
    #[napi]
    pub fn from_signatures_with_checksum(
        signatures: Vec<String>,
        checksum: bool,
    ) -> napi::Result<CallDecoder> {
        let functions = parse_signatures(&signatures)
            .context("build inner decoder")
            .map_err(map_err)?;

        Ok(Self {
            functions: Arc::new(functions),
            checksummed_addresses: checksum,
        })
    }

    /// Enable checksummed addresses in decoded output
    #[napi]
    pub fn enable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = true;
    }

    /// Disable checksummed addresses in decoded output
    #[napi]
    pub fn disable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = false;
    }

    /// Decode function call inputs asynchronously
    #[napi]
    pub async fn decode_inputs(
        &self,
        inputs: Vec<String>,
    ) -> napi::Result<Vec<Option<Vec<DecodedSolValue>>>> {
        let decoder = self.clone();

        tokio::task::spawn_blocking(move || decoder.decode_inputs_sync(inputs))
            .await
            .context("decode task")
            .map_err(map_err)
    }

    /// Decode transaction inputs asynchronously
    #[napi]
    pub async fn decode_transactions_input(
        &self,
        txs: Vec<Transaction>,
    ) -> napi::Result<Vec<Option<Vec<DecodedSolValue>>>> {
        let decoder = self.clone();

        tokio::task::spawn_blocking(move || decoder.decode_transactions_input_sync(txs))
            .await
            .context("decode task")
            .map_err(map_err)
    }

    /// Decode trace inputs asynchronously
    #[napi]
    pub async fn decode_traces_input(
        &self,
        traces: Vec<Trace>,
    ) -> napi::Result<Vec<Option<Vec<DecodedSolValue>>>> {
        let decoder = self.clone();

        tokio::task::spawn_blocking(move || decoder.decode_traces_input_sync(traces))
            .await
            .context("decode task")
            .map_err(map_err)
    }

    /// Decode function call inputs synchronously
    #[napi]
    pub fn decode_inputs_sync(&self, inputs: Vec<String>) -> Vec<Option<Vec<DecodedSolValue>>> {
        inputs
            .into_iter()
            .map(|input| self.decode_impl(input))
            .collect()
    }

    /// Decode transaction inputs synchronously
    #[napi]
    pub fn decode_transactions_input_sync(
        &self,
        txs: Vec<Transaction>,
    ) -> Vec<Option<Vec<DecodedSolValue>>> {
        txs.into_iter()
            .map(|tx| self.decode_impl(tx.input?))
            .collect()
    }

    /// Decode trace inputs synchronously
    #[napi]
    pub fn decode_traces_input_sync(
        &self,
        traces: Vec<Trace>,
    ) -> Vec<Option<Vec<DecodedSolValue>>> {
        traces
            .into_iter()
            .map(|trace| self.decode_impl(trace.input?))
            .collect()
    }

    /// Decode a single input string
    ///
    /// Returns null if the input is not valid hex, doesn't match a known
    /// function selector, or doesn't decode as that function's arguments.
    #[napi]
    pub fn decode_impl(&self, input: String) -> Option<Vec<DecodedSolValue>> {
        let input = Data::decode_hex(input.as_str()).ok()?;
        let selector: Selector = input.get(..4)?.try_into().ok()?;
        let function = self.functions.get(&selector)?;
        let decoded = function.abi_decode_input(&input[4..]).ok()?;
        Some(
            decoded
                .into_iter()
                .map(|value| DecodedSolValue::new(value, self.checksummed_addresses))
                .collect(),
        )
    }
}
