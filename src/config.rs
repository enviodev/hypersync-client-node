use std::num::NonZeroU64;

use anyhow::{Context, Result};

#[napi(object)]
#[derive(Default, Clone)]
pub struct Config {
  pub url: String,
  pub bearer_token: Option<String>,
  pub http_req_timeout_millis: Option<i64>,
}

impl Config {
  pub fn try_convert(&self) -> Result<skar_client::Config> {
    Ok(skar_client::Config {
      url: self.url.parse().context("parse url")?,
      bearer_token: self.bearer_token.clone(),
      http_req_timeout_millis: match self.http_req_timeout_millis {
        Some(c) => {
          NonZeroU64::new(c.try_into().context("parse timeout")?).context("parse timeout")?
        }
        None => NonZeroU64::new(30000).unwrap(),
      },
    })
  }
}
