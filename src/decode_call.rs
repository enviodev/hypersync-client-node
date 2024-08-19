use crate::map_err;
use crate::types::DecodedSolValue;
use anyhow::Context;
use hypersync_client::format::{Data, Hex};
use hypersync_client::CallDecoder;
use std::sync::Arc;

#[napi]
#[derive(Clone)]
pub struct Decoder {
    inner: Arc<CallDecoder>,
    checksummed_addresses: bool,
}

#[napi]
impl Decoder {
    #[napi]
    pub fn from_signatures(signatures: Vec<String>) -> napi::Result<Decoder> {
        let inner = hypersync_client::CallDecoder::from_signatures(&signatures)
            .context("build inner decoder")
            .map_err(map_err)?;

        Ok(Self {
            inner: Arc::new(inner),
            checksummed_addresses: false,
        })
    }

    pub fn enable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = true;
    }

    pub fn disable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = false;
    }

    #[napi]
    pub async fn decode_input(&self, input: String) -> Option<Vec<DecodedSolValue>> {
        let decoder = self.clone();

        tokio::task::spawn_blocking(move || decoder.decode_input_sync(input))
            .await
            .unwrap()
    }

    #[napi]
    pub fn decode_input_sync(&self, input: String) -> Option<Vec<DecodedSolValue>> {
        let input = Data::decode_hex(input.as_str())
            .context("decode input")
            .unwrap();

        self.inner
            .decode_input(&input)
            .context("decode log")
            .unwrap()
            .map(|v| {
                v.into_iter()
                    .map(|value| DecodedSolValue::new(value, self.checksummed_addresses))
                    .collect()
            })
    }
}
