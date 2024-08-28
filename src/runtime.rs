use async_tungstenite::tungstenite::{Error, Message};
use async_tungstenite::WebSocketStream;
use futures_util::stream::{SplitSink, SplitStream};

// ******************************
// ASYNC-STD Channels
// *****************************

#[cfg(feature = "use-async-std")]
pub use async_std::channel::{unbounded as channel, Receiver, Sender};
#[cfg(feature = "use-async-std")]
pub use async_std::task::spawn;
#[cfg(feature = "use-async-std")]
use async_std::{AsyncRead, AsyncWrite};

// ******************************
// Tokio Channels
// *****************************
#[cfg(feature = "use-tokio")]
pub use tokio::sync::mpsc::{channel, Receiver, Sender};
#[cfg(feature = "use-tokio")]
pub use tokio::task::spawn;
#[cfg(feature = "use-tokio")]
use tokio::io::{AsyncRead, AsyncWrite};

pub async fn ws_incoming_messages(
    mut stream: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>,
    to_user: Sender<Result<Message, Error>>,
) {
    loop {
        while let Some(message) = stream.next().await {
            let _ = to_user.send(message).await;
        }
    }
}

pub async fn ws_outgoing_messages(
    mut sink: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>,
    mut from_user: Receiver<Message>,
) {
    loop {
        match from_user.recv().await {
            Some(msg) => sink.send(msg).await.expect("Failed to send message"),
            None => todo!(),
        }
    }
}

