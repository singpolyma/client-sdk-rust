use crate::signal_client::signal_stream::SignalStream;
use livekit_protocol as proto;

use parking_lot::RwLock;
use std::fmt::Debug;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Error as WsError;
use tracing::{instrument, Level};

mod signal_stream;

pub(crate) type SignalEmitter = mpsc::Sender<SignalEvent>;
pub(crate) type SignalEvents = mpsc::Receiver<SignalEvent>;
pub(crate) type SignalResult<T> = Result<T, SignalError>;

pub const JOIN_RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Error, Debug)]
pub enum SignalError {
    #[error("ws failure: {0}")]
    WsError(#[from] WsError),
    #[error("failed to parse the url")]
    UrlParse(#[from] url::ParseError),
    #[error("failed to decode messages from server")]
    ProtoParse(#[from] prost::DecodeError),
    #[error("{0}")]
    Timeout(String),
}

/// Events used by the RTCEngine who will handle the reconnection logic
#[derive(Debug)]
pub enum SignalEvent {
    Open,
    Signal(proto::signal_response::Message),
    Close,
}

#[derive(Debug, Clone)]
pub struct SignalOptions {
    pub(crate) reconnect: bool,
    pub(crate) sid: String,
    pub auto_subscribe: bool,
    pub adaptive_stream: bool,
}

impl Default for SignalOptions {
    fn default() -> Self {
        Self {
            reconnect: false,
            auto_subscribe: true,
            sid: "".to_string(),
            adaptive_stream: false,
        }
    }
}

#[derive(Debug)]
pub struct SignalClient {
    stream: RwLock<Option<SignalStream>>,
    emitter: SignalEmitter,
}

impl SignalClient {
    pub fn new() -> (Self, SignalEvents) {
        let (emitter, events) = mpsc::channel(8);
        (
            Self {
                stream: Default::default(),
                emitter,
            },
            events,
        )
    }

    #[instrument(level = Level::DEBUG, skip(url, token, options))]
    pub async fn connect(
        &self,
        url: &str,
        token: &str,
        options: SignalOptions,
    ) -> SignalResult<()> {
        let stream = SignalStream::connect(url, token, options, self.emitter.clone()).await?;
        *self.stream.write() = Some(stream);
        Ok(())
    }

    #[instrument(level = Level::DEBUG)]
    pub async fn close(&self) {
        if let Some(stream) = self.stream.write().take() {
            stream.close().await;
        }
    }

    #[instrument(level = Level::DEBUG)]
    pub async fn send(&self, signal: proto::signal_request::Message) {
        if let Some(stream) = self.stream.read().as_ref() {
            if stream.send(signal).await.is_ok() {
                return;
            }
        }

        // TODO(theomonnom): enqueue message
    }

    /*#[allow(dead_code)]
    pub async fn clear_queue(&self) {
        // TODO(theomonnom): impl
    }*/

    #[instrument(level = Level::DEBUG)]
    pub async fn flush_queue(&self) {
        // TODO(theomonnom): impl
    }
}

pub mod utils {
    use crate::signal_client::{SignalError, SignalEvent, SignalResult, JOIN_RESPONSE_TIMEOUT};
    use livekit_protocol as proto;
    use tokio::time::timeout;
    use tokio_tungstenite::tungstenite::Error as WsError;
    use tracing::{event, instrument, Level};

    use super::SignalEvents;

    #[instrument(level = Level::DEBUG, skip(receiver))]
    pub async fn next_join_response(
        receiver: &mut SignalEvents,
    ) -> SignalResult<proto::JoinResponse> {
        let join = async {
            while let Some(event) = receiver.recv().await {
                match event {
                    SignalEvent::Signal(proto::signal_response::Message::Join(join)) => {
                        return Ok(join)
                    }
                    SignalEvent::Close => break,
                    SignalEvent::Open => continue,
                    _ => {
                        event!(
                            Level::WARN,
                            "received unexpected message while waiting for JoinResponse: {:?}",
                            event
                        );
                        continue;
                    }
                }
            }

            Err(WsError::ConnectionClosed)?
        };

        timeout(JOIN_RESPONSE_TIMEOUT, join)
            .await
            .map_err(|_| SignalError::Timeout("failed to receive JoinResponse".to_string()))?
    }
}
