use crate::map_err;
use crate::types::{DecodedSolValue, Trace, Transaction};
use anyhow::Context;
use hypersync_client::format::{Data, Hex};
use std::sync::Arc;

#[napi]
#[derive(Clone)]
pub struct CallDecoder {
    inner: Arc<hypersync_client::CallDecoder>,
    checksummed_addresses: bool,
}

#[napi]
impl CallDecoder {
    #[napi]
    pub fn from_signatures(signatures: Vec<String>) -> napi::Result<CallDecoder> {
        let inner = hypersync_client::CallDecoder::from_signatures(&signatures)
            .context("build inner decoder")
            .map_err(map_err)?;

        Ok(Self {
            inner: Arc::new(inner),
            checksummed_addresses: false,
        })
    }

    #[napi]
    pub fn from_signatures_with_checksum(
        signatures: Vec<String>,
        checksum: bool,
    ) -> napi::Result<CallDecoder> {
        let inner = hypersync_client::CallDecoder::from_signatures(&signatures)
            .context("build inner decoder")
            .map_err(map_err)?;

        Ok(Self {
            inner: Arc::new(inner),
            checksummed_addresses: checksum,
        })
    }

    pub fn enable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = true;
    }

    pub fn disable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = false;
    }

    #[napi]
    pub async fn decode_inputs(&self, inputs: Vec<String>) -> Vec<Option<Vec<DecodedSolValue>>> {
        let decoder = self.clone();

        tokio::task::spawn_blocking(move || decoder.decode_inputs_sync(inputs))
            .await
            .unwrap()
    }

    #[napi]

    pub async fn decode_transactions_input(
        &self,
        txs: Vec<Transaction>,
    ) -> Vec<Option<Vec<DecodedSolValue>>> {
        let decoder = self.clone();

        tokio::task::spawn_blocking(move || decoder.decode_transactions_input_sync(txs))
            .await
            .unwrap()
    }

    #[napi]
    pub async fn decode_traces_input(
        &self,
        traces: Vec<Trace>,
    ) -> Vec<Option<Vec<DecodedSolValue>>> {
        let decoder = self.clone();

        tokio::task::spawn_blocking(move || decoder.decode_traces_input_sync(traces))
            .await
            .unwrap()
    }

    #[napi]
    pub fn decode_inputs_sync(&self, inputs: Vec<String>) -> Vec<Option<Vec<DecodedSolValue>>> {
        inputs
            .into_iter()
            .map(|input| self.decode_impl(input))
            .collect()
    }

    #[napi]
    pub fn decode_transactions_input_sync(
        &self,
        txs: Vec<Transaction>,
    ) -> Vec<Option<Vec<DecodedSolValue>>> {
        txs.into_iter()
            .map(|tx| self.decode_impl(tx.input?))
            .collect()
    }

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

    #[napi]
    pub fn decode_impl(&self, input: String) -> Option<Vec<DecodedSolValue>> {
        let input = Data::decode_hex(input.as_str())
            .context("decode input")
            .unwrap();
        let decoded_input = self
            .inner
            .decode_input(&input)
            .context("decode log")
            .unwrap();
        decoded_input.map(|decoded_input| {
            decoded_input
                .into_iter()
                .map(|value| DecodedSolValue::new(value, self.checksummed_addresses))
                .collect()
        })
    }
}
