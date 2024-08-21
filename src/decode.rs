use std::sync::Arc;

use anyhow::{Context, Result};
use hypersync_client::format::{Data, Hex, LogArgument};

use crate::{
    map_err,
    types::{DecodedEvent, DecodedSolValue, Event, Log},
};

#[napi]
#[derive(Clone)]
pub struct Decoder {
    inner: Arc<hypersync_client::Decoder>,
    checksummed_addresses: bool,
}

#[napi]
impl Decoder {
    #[napi]
    pub fn from_signatures(signatures: Vec<String>) -> napi::Result<Decoder> {
        let inner = hypersync_client::Decoder::from_signatures(&signatures)
            .context("create inner decoder")
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
    ) -> napi::Result<Decoder> {
        let inner = hypersync_client::Decoder::from_signatures(&signatures)
            .context("create inner decoder")
            .map_err(map_err)?;
        Ok(Self {
            inner: Arc::new(inner),
            checksummed_addresses: checksum,
        })
    }

    #[napi]
    pub fn enable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = true;
    }

    #[napi]
    pub fn disable_checksummed_addresses(&mut self) {
        self.checksummed_addresses = false;
    }

    #[napi]
    pub async fn decode_logs(&self, logs: Vec<Log>) -> Vec<Option<DecodedEvent>> {
        let decoder = self.clone();
        tokio::task::spawn_blocking(move || decoder.decode_logs_sync(logs))
            .await
            .unwrap()
    }

    #[napi]
    pub fn decode_logs_sync(&self, logs: Vec<Log>) -> Vec<Option<DecodedEvent>> {
        logs.iter()
            .map(|log| self.decode_impl(log).ok().flatten())
            .collect::<Vec<_>>()
    }

    #[napi]
    pub async fn decode_events(&self, events: Vec<Event>) -> Vec<Option<DecodedEvent>> {
        let decoder = self.clone();
        tokio::task::spawn_blocking(move || decoder.decode_events_sync(events))
            .await
            .unwrap()
    }

    #[napi]
    pub fn decode_events_sync(&self, events: Vec<Event>) -> Vec<Option<DecodedEvent>> {
        events
            .iter()
            .map(|event| self.decode_impl(&event.log).ok().flatten())
            .collect::<Vec<_>>()
    }

    fn decode_impl(&self, log: &Log) -> Result<Option<DecodedEvent>> {
        let topics = log
            .topics
            .iter()
            .map(|v| {
                v.as_ref()
                    .map(|v| LogArgument::decode_hex(v).context("decode topic"))
                    .transpose()
            })
            .collect::<Result<Vec<_>>>()
            .context("decode topics")?;

        let topic0 = topics
            .first()
            .context("get topic0")?
            .as_ref()
            .context("topic0 is null")?;

        let data = log.data.as_ref().context("get log.data")?;
        let data = Data::decode_hex(data).context("decode data")?;

        let decoded = match self
            .inner
            .decode(topic0.as_slice(), &topics, &data)
            .context("decode log")?
        {
            Some(v) => v,
            None => return Ok(None),
        };

        Ok(Some(DecodedEvent {
            indexed: decoded
                .indexed
                .into_iter()
                .map(|v| DecodedSolValue::new(v, self.checksummed_addresses))
                .collect(),
            body: decoded
                .body
                .into_iter()
                .map(|v| DecodedSolValue::new(v, self.checksummed_addresses))
                .collect(),
        }))
    }
}
