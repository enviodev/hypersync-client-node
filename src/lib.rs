#[macro_use]
extern crate napi_derive;

use std::sync::Once;

use anyhow::{Context, Result};
use napi::bindgen_prelude::Either3;
use tokio::sync::mpsc;

mod config;
mod decode;
mod decode_call;
pub mod preset_query;
mod query;
mod types;

use config::{ClientConfig, StreamConfig};
use query::Query;
use types::{Block, Event, Log, RollbackGuard, Trace, Transaction};

static LOGGER_INIT: Once = Once::new();

fn init_logger(log_level: Option<&str>) {
    LOGGER_INIT.call_once(|| {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        } else if let Some(filter) = log_level {
            env_logger::Builder::new().parse_filters(filter).init();
        }
    });
}

/// Set the log level for the underlying Rust logger.
///
/// Accepts values like "info", "warn", "debug", "trace", "error",
/// or a full filter directive like "hypersync_client=debug".
/// If RUST_LOG env var is set, it takes precedence.
/// Must be called before creating any HypersyncClient.
/// Only the first call takes effect (logger can only init once per process).
#[napi]
pub fn set_log_level(level: String) {
    init_logger(Some(&level));
}

/// Rate limit information from server response headers.
#[napi(object)]
pub struct RateLimitInfo {
    /// Total request quota for the current window.
    pub limit: Option<i64>,
    /// Remaining budget in the current window.
    pub remaining: Option<i64>,
    /// Seconds until the rate limit window resets.
    pub reset_secs: Option<i64>,
    /// Budget consumed per request.
    pub cost: Option<i64>,
}

impl From<hypersync_client::RateLimitInfo> for RateLimitInfo {
    fn from(info: hypersync_client::RateLimitInfo) -> Self {
        Self {
            limit: info.limit.map(|v| v as i64),
            remaining: info.remaining.map(|v| v as i64),
            reset_secs: info.reset_secs.map(|v| v as i64),
            cost: info.cost.map(|v| v as i64),
        }
    }
}

/// Response from a query that includes rate limit information.
#[napi(object)]
pub struct QueryResponseWithRateLimit {
    /// The query response data.
    pub response: QueryResponse,
    /// Rate limit information from response headers.
    pub rate_limit: RateLimitInfo,
}


/// HyperSync client for querying blockchain data
#[napi]
pub struct HypersyncClient {
    inner: hypersync_client::Client,
    enable_checksum_addresses: bool,
}

#[napi]
impl HypersyncClient {
    /// Create a new client with given config
    #[napi(constructor)]
    pub fn new(cfg: ClientConfig) -> napi::Result<HypersyncClient> {
        Self::new_with_agent(cfg, format!("hscn/{}", env!("CARGO_PKG_VERSION")))
    }

    /// Create a new client with custom user agent
    ///
    /// This method is intended for internal use when you need to customize the user agent string.
    /// Most users should use `new()` instead.
    ///
    /// @internal
    #[doc(hidden)]
    #[napi]
    pub fn new_with_agent(cfg: ClientConfig, user_agent: String) -> napi::Result<HypersyncClient> {
        init_logger(Some("info"));

        let enable_checksum_addresses = cfg.enable_checksum_addresses.unwrap_or_default();

        let inner = hypersync_client::Client::new_with_agent(cfg.into(), user_agent)
            .context("build client")
            .map_err(map_err)?;

        Ok(HypersyncClient {
            inner,
            enable_checksum_addresses,
        })
    }

    /// Get the height of the source hypersync instance
    #[napi]
    pub async fn get_height(&self) -> napi::Result<i64> {
        let height = self.inner.get_height().await.map_err(map_err)?;

        Ok(height.try_into().unwrap())
    }

    /// Get the chain_id of the source hypersync instance
    #[napi]
    pub async fn get_chain_id(&self) -> napi::Result<i64> {
        let chain_id = self.inner.get_chain_id().await.map_err(map_err)?;

        Ok(chain_id.try_into().unwrap())
    }

    /// Collect blockchain data from the given query
    #[napi]
    pub async fn collect(&self, query: Query, config: StreamConfig) -> napi::Result<QueryResponse> {
        let query = query.try_into().context("parse query").map_err(map_err)?;

        let resp = self
            .inner
            .clone()
            .collect(query, config.into())
            .await
            .context("run inner collect")
            .map_err(map_err)?;

        convert_response(resp, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)
    }

    /// Collect blockchain events from the given query
    #[napi]
    pub async fn collect_events(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<EventResponse> {
        let query = query.try_into().context("parse query").map_err(map_err)?;
        let config = config.into();

        let resp = self
            .inner
            .clone()
            .collect_events(query, config)
            .await
            .context("run inner collect")
            .map_err(map_err)?;

        convert_event_response(resp, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)
    }

    /// Collect blockchain data and save to parquet format
    #[napi]
    pub async fn collect_parquet(
        &self,
        path: String,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<()> {
        let query = query.try_into().context("parse query").map_err(map_err)?;
        let config = config.into();

        self.inner
            .clone()
            .collect_parquet(&path, query, config)
            .await
            .map_err(map_err)
    }

    /// Get blockchain data for a single query
    #[napi]
    pub async fn get(&self, query: Query) -> napi::Result<QueryResponse> {
        let query = query.try_into().context("parse query").map_err(map_err)?;
        let res = self
            .inner
            .get(&query)
            .await
            .context("run inner query")
            .map_err(map_err)?;
        convert_response(res, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)
    }

    /// Get blockchain events for a single query
    #[napi]
    pub async fn get_events(&self, query: Query) -> napi::Result<EventResponse> {
        let query = query.try_into().context("parse query").map_err(map_err)?;
        let res = self
            .inner
            .get_events(query)
            .await
            .context("run inner query")
            .map_err(map_err)?;
        let r = convert_event_response(res, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)?;
        Ok(r)
    }

    /// Stream chain height events
    #[napi]
    // note: needs to be async for napi to allow a tokio::spawn internally
    pub async fn stream_height(&self) -> HeightStream {
        let inner = self.inner.clone().stream_height();

        HeightStream {
            inner: tokio::sync::Mutex::new(inner),
        }
    }
    /// Stream blockchain data from the given query
    #[napi]
    pub async fn stream(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<QueryResponseStream> {
        let query = query.try_into().context("parse query").map_err(map_err)?;
        let config = config.into();

        let inner = self
            .inner
            .clone()
            .stream(query, config)
            .await
            .context("start stream")
            .map_err(map_err)?;

        Ok(QueryResponseStream {
            inner: tokio::sync::Mutex::new(inner),
            enable_checksum_addresses: self.enable_checksum_addresses,
        })
    }

    /// Get blockchain data for a single query, with rate limit info
    #[napi]
    pub async fn get_with_rate_limit(
        &self,
        query: Query,
    ) -> napi::Result<QueryResponseWithRateLimit> {
        let query = query.try_into().context("parse query").map_err(map_err)?;
        let res = self
            .inner
            .get_with_rate_limit(&query)
            .await
            .context("run inner query")
            .map_err(map_err)?;
        let response = convert_response(res.response, self.enable_checksum_addresses)
            .context("convert response")
            .map_err(map_err)?;
        Ok(QueryResponseWithRateLimit {
            response,
            rate_limit: res.rate_limit.into(),
        })
    }

    /// Get the most recently observed rate limit information.
    /// Returns null if no requests have been made yet.
    #[napi]
    pub fn rate_limit_info(&self) -> Option<RateLimitInfo> {
        self.inner.rate_limit_info().map(|info| info.into())
    }

    /// Wait until the current rate limit window resets.
    /// Returns immediately if no rate limit info observed or quota available.
    #[napi]
    pub async fn wait_for_rate_limit(&self) {
        self.inner.wait_for_rate_limit().await;
    }

    /// Stream blockchain events from the given query
    #[napi]
    pub async fn stream_events(
        &self,
        query: Query,
        config: StreamConfig,
    ) -> napi::Result<EventStream> {
        let query = query.try_into().context("parse query").map_err(map_err)?;
        let config = config.into();

        let inner = self
            .inner
            .clone()
            .stream_events(query, config)
            .await
            .context("start stream")
            .map_err(map_err)?;

        Ok(EventStream {
            inner: tokio::sync::Mutex::new(inner),
            enable_checksum_addresses: self.enable_checksum_addresses,
        })
    }
}

/// Stream for receiving query responses
#[napi]
pub struct QueryResponseStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<hypersync_client::QueryResponse>>>,
    enable_checksum_addresses: bool,
}

#[napi]
impl QueryResponseStream {
    /// Close the response stream
    #[napi]
    pub async fn close(&self) {
        self.inner.lock().await.close();
    }

    /// Receive the next query response from the stream
    #[napi]
    pub async fn recv(&self) -> napi::Result<Option<QueryResponse>> {
        let resp = self.inner.lock().await.recv().await;

        resp.map(|r| {
            convert_response(r?, self.enable_checksum_addresses).context("convert response")
        })
        .transpose()
        .map_err(map_err)
    }
}

type HSEventResponse = hypersync_client::QueryResponse<Vec<hypersync_client::simple_types::Event>>;

/// Stream for receiving event responses
#[napi]
pub struct EventStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<Result<HSEventResponse>>>,
    enable_checksum_addresses: bool,
}

#[napi]
impl EventStream {
    /// Close the event stream
    #[napi]
    pub async fn close(&self) {
        self.inner.lock().await.close();
    }

    /// Receive the next event response from the stream
    #[napi]
    pub async fn recv(&self) -> napi::Result<Option<EventResponse>> {
        let resp = self.inner.lock().await.recv().await;

        resp.map(|r| {
            convert_event_response(r?, self.enable_checksum_addresses).context("convert response")
        })
        .transpose()
        .map_err(map_err)
    }
}

#[napi(string_enum)]
pub enum HeightTag {
    Height,
}
#[napi(object)]
pub struct HeightStreamHeightEvent {
    #[napi(js_name = "type")]
    pub type_: HeightTag,
    pub height: i64,
}

#[napi(string_enum)]
pub enum ConnectedTag {
    Connected,
}

#[napi(object)]
pub struct HeightStreamConnectedEvent {
    #[napi(js_name = "type")]
    pub type_: ConnectedTag,
}

#[napi(string_enum)]
pub enum ReconnectingTag {
    Reconnecting,
}

#[napi(object)]
pub struct HeightStreamReconnectingEvent {
    #[napi(js_name = "type")]
    pub type_: ReconnectingTag,
    pub delay_millis: i64,
    pub error_msg: String,
}

#[napi]
/// Height stream event, switch on 'event.type' to get different payload options
///
/// switch (event.type) {
///   case "Height":
///     console.log("Height:", event.height);
///     break;
///   case "Connected":
///     console.log("Connected to stream");
///     break;
///   case "Reconnecting":
///     console.log("Reconnecting in", event.delayMillis, "ms", "due to error:", event.errorMsg);
///     break;
/// }
pub type HeightStreamEvent =
    Either3<HeightStreamHeightEvent, HeightStreamConnectedEvent, HeightStreamReconnectingEvent>;

fn try_into_height_stream_event(
    e: hypersync_client::HeightStreamEvent,
) -> Result<HeightStreamEvent> {
    let event = match e {
        hypersync_client::HeightStreamEvent::Height(h) => Either3::A(HeightStreamHeightEvent {
            type_: HeightTag::Height,
            height: i64::try_from(h).context("convert height to i64")?,
        }),
        hypersync_client::HeightStreamEvent::Connected => Either3::B(HeightStreamConnectedEvent {
            type_: ConnectedTag::Connected,
        }),
        hypersync_client::HeightStreamEvent::Reconnecting { delay, error_msg } => {
            Either3::C(HeightStreamReconnectingEvent {
                type_: ReconnectingTag::Reconnecting,
                delay_millis: i64::try_from(delay.as_millis())
                    .context("convert reconnect delay millis to i64")?,
                error_msg,
            })
        }
    };
    Ok(event)
}

/// Stream for receiving height stream events
/// yields the immediate height of the chain and then
/// continues to yield height updates as they are received
#[napi]
pub struct HeightStream {
    inner: tokio::sync::Mutex<mpsc::Receiver<hypersync_client::HeightStreamEvent>>,
}

#[napi]
impl HeightStream {
    /// Close the height stream
    #[napi]
    pub async fn close(&self) {
        self.inner.lock().await.close();
    }

    /// Receive the next height stream event from the stream
    #[napi]
    pub async fn recv(&self) -> napi::Result<Option<HeightStreamEvent>> {
        let resp = self.inner.lock().await.recv().await;
        resp.map(|hs_height_event| try_into_height_stream_event(hs_height_event).map_err(map_err))
            .transpose()
    }
}

/// Data returned from a query response
#[napi(object)]
pub struct QueryResponseData {
    /// Blocks returned by the query
    pub blocks: Vec<Block>,
    /// Transactions returned by the query
    pub transactions: Vec<Transaction>,
    /// Logs returned by the query
    pub logs: Vec<Log>,
    /// Traces returned by the query
    pub traces: Vec<Trace>,
}

/// Response from a blockchain query
#[napi(object)]
pub struct QueryResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub data: QueryResponseData,
    /// Rollback guard, supposed to be used to detect rollbacks
    pub rollback_guard: Option<RollbackGuard>,
}

/// Response from an event query
#[napi(object)]
pub struct EventResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub data: Vec<Event>,
    /// Rollback guard, supposed to be used to detect rollbacks
    pub rollback_guard: Option<RollbackGuard>,
}

/// Collection of events from a blockchain query
#[napi(object)]
pub struct Events {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub events: Vec<Event>,
    /// Rollback guard, supposed to be used to detect rollbacks
    pub rollback_guard: Option<RollbackGuard>,
}

fn convert_response(
    res: hypersync_client::QueryResponse,
    should_checksum: bool,
) -> Result<QueryResponse> {
    let blocks = res
        .data
        .blocks
        .iter()
        .flat_map(|b| b.iter().map(|b| Block::from_simple(b, should_checksum)))
        .collect::<Result<Vec<_>>>()
        .context("mapping blocks")?;

    let transactions = res
        .data
        .transactions
        .iter()
        .flat_map(|b| {
            b.iter()
                .map(|tx| Transaction::from_simple(tx, should_checksum))
        })
        .collect::<Result<Vec<_>>>()
        .context("mapping transactions")?;

    let logs = res
        .data
        .logs
        .iter()
        .flat_map(|b| b.iter().map(|l| Log::from_simple(l, should_checksum)))
        .collect::<Result<Vec<_>>>()
        .context("mapping logs")?;

    let traces = res
        .data
        .traces
        .iter()
        .flat_map(|b| b.iter().map(|tr| Trace::from_simple(tr, should_checksum)))
        .collect::<Result<Vec<_>>>()
        .context("mapping traces")?;

    Ok(QueryResponse {
        archive_height: res
            .archive_height
            .map(|h| h.try_into())
            .transpose()
            .context("convert height")?,
        next_block: res.next_block.try_into().context("convert next_block")?,
        total_execution_time: res
            .total_execution_time
            .try_into()
            .context("convert total_execution_time")?,
        data: QueryResponseData {
            blocks,
            transactions,
            logs,
            traces,
        },
        rollback_guard: res
            .rollback_guard
            .map(RollbackGuard::try_from)
            .transpose()
            .context("convert rollback guard")?,
    })
}

fn convert_event_response(
    resp: hypersync_client::QueryResponse<Vec<hypersync_client::simple_types::Event>>,
    should_checksum: bool,
) -> Result<EventResponse> {
    let data = resp
        .data
        .into_iter()
        .map(|event| {
            Ok(Event {
                transaction: event
                    .transaction
                    .map(|v| Transaction::from_simple(&v, should_checksum))
                    .transpose()
                    .context("mapping transaction")?,
                block: event
                    .block
                    .map(|v| Block::from_simple(&v, should_checksum))
                    .transpose()
                    .context("mapping block")?,
                log: Log::from_simple(&event.log, should_checksum).context("mapping log")?,
            })
        })
        .collect::<Result<Vec<_>>>()
        .context("mapping response data")?;

    Ok(EventResponse {
        archive_height: resp
            .archive_height
            .map(|v| v.try_into())
            .transpose()
            .context("mapping archive_height")?,
        next_block: resp.next_block.try_into().context("mapping next_block")?,
        total_execution_time: resp
            .total_execution_time
            .try_into()
            .context("mapping total_execution_time")?,
        data,
        rollback_guard: resp
            .rollback_guard
            .map(|rg| RollbackGuard::try_from(rg).context("convert rollback guard"))
            .transpose()?,
    })
}

fn map_err(e: anyhow::Error) -> napi::Error {
    napi::Error::from_reason(format!("{:?}", e))
}
