use std::collections::HashMap;
use std::sync::Arc;

use alloy_json_abi::JsonAbi;
use anyhow::{anyhow, Context, Result};
use skar_format::{Address, Hex, LogArgument};

use crate::types::{DecodedEvent, DecodedSolValue, Event, Log};

#[napi]
#[derive(Clone)]
pub struct Decoder {
    inner: Arc<skar_client::Decoder>,
    checksummed_addresses: bool,
}

#[napi]
impl Decoder {
    #[napi]
    pub fn new(json_abis: HashMap<String, serde_json::Value>) -> napi::Result<Decoder> {
        Self::new_impl(json_abis).map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    fn new_impl(json_abis: HashMap<String, serde_json::Value>) -> Result<Self> {
        let json_abis = json_abis
            .into_iter()
            .map(|(addr, abi)| {
                let json = serde_json::to_string(&abi).context("serialize json")?;
                let abi: JsonAbi = serde_json::from_str(&json).context("parse json abi")?;
                let addr = Address::decode_hex(&addr).context("decode hex address")?;
                Ok((addr, abi))
            })
            .collect::<Result<Vec<_>>>()
            .context("parse json abi list")?;

        let inner = skar_client::Decoder::new(&json_abis).context("build inner decoder")?;

        Ok(Self {
            inner: Arc::new(inner),
            checksummed_addresses: false,
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
    pub async fn decode_logs(&self, logs: Vec<Log>) -> napi::Result<Vec<Option<DecodedEvent>>> {
        let decoder = self.clone();
        tokio::task::spawn_blocking(move || decoder.decode_logs_sync(logs))
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?
    }

    #[napi]
    pub fn decode_logs_sync(&self, logs: Vec<Log>) -> napi::Result<Vec<Option<DecodedEvent>>> {
        logs.iter()
            .map(|log| self.decode_impl(log))
            .collect::<Result<Vec<_>>>()
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    #[napi]
    pub async fn decode_events(
        &self,
        events: Vec<Event>,
    ) -> napi::Result<Vec<Option<DecodedEvent>>> {
        let decoder = self.clone();
        tokio::task::spawn_blocking(move || decoder.decode_events_sync(events))
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?
    }

    #[napi]
    pub fn decode_events_sync(
        &self,
        events: Vec<Event>,
    ) -> napi::Result<Vec<Option<DecodedEvent>>> {
        events
            .iter()
            .map(|event| self.decode_impl(&event.log))
            .collect::<Result<Vec<_>>>()
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    fn decode_impl(&self, log: &Log) -> Result<Option<DecodedEvent>> {
        let address = log.address.as_ref().context("get address")?;
        let address = Address::decode_hex(address).context("decode address")?;

        let mut topics = Vec::new();

        for topic in log.topics.iter() {
            match topic {
                Some(topic) => {
                    let topic = LogArgument::decode_hex(topic).context("decode topic")?;
                    topics.push(Some(topic));
                }
                None => topics.push(None),
            }
        }

        let topics = topics
            .iter()
            .map(|t| t.as_ref().map(|t| t.as_slice()))
            .collect::<Vec<Option<&[u8]>>>();

        let topic0 = topics
            .first()
            .context("get topic0")?
            .context("get topic0")?;

        let data = log.data.as_ref().context("get data field")?;
        let data: Vec<u8> =
            prefix_hex::decode(data).map_err(|e| anyhow!("decode data field: {}", e))?;

        let decoded = match self
            .inner
            .decode(address.as_slice(), topic0, &topics, &data)
            .context("decode log")?
        {
            Some(decoded) => decoded,
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
