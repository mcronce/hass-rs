use crate::types::{Command, Response, Subscribe, Unsubscribe, WSEvent};
use crate::{connect_async, task, HassError, HassResult, WebSocket};

use async_tungstenite::tungstenite::Message as TungsteniteMessage;
//use futures_channel::mpsc::{channel, Receiver, Sender};
use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};
//use log::info;
use std::collections::HashMap;

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use url;

pub struct WsConn {
    //message sequence required by the Websocket server, I may need this field on recconect
    //last_sequence: Arc<AtomicU64>,

    //Client --> Gateway (send "Commands" msg to the Gateway)
    pub(crate) to_gateway: Sender<Command>,

    //Gateway --> Client (receive "Response" msg from the Gateway)
    pub(crate) from_gateway: Receiver<HassResult<Response>>,

    //Register all the events and their callback
    //Should I modify the callback signature ? -- like Box<dyn Fn(WSEvent) -> BoxFuture<'static, EventResult>
    pub(crate) event_listeners: Arc<Mutex<HashMap<u64, Box<dyn Fn(WSEvent) + Send>>>>,
    //Should I create a hashmap for Commands?, not clear if it's useful.
}

impl WsConn {
    pub(crate) async fn connect(url: url::Url) -> HassResult<WsConn> {
        let wsclient = connect_async(url).await.expect("Can't connect to gateway");
        let (sink, stream) = wsclient.split();

        //Channels to recieve the Client Command and send it over to the Websocket server
        let (to_gateway, from_client) = channel::<Command>(20);

        //Channels to receive the Response from the Websocket server and send it over to the Client
        let (to_client, from_gateway) = channel::<HassResult<Response>>(20);

        let last_sequence = Arc::new(AtomicU64::new(1));
        let last_sequence_clone_sender = Arc::clone(&last_sequence);
        //let last_sequence_clone_receiver = Arc::clone(&last_sequence);

        let event_listeners = Arc::new(Mutex::new(HashMap::new()));
        let event_listeners_clone_receiver = Arc::clone(&event_listeners);

        // Client --> Gateway
        if let Err(e) = sender_loop(last_sequence_clone_sender, sink, from_client).await {
            //to_client.send(Err(HassError::from(e))).await?
            return Err(e);
        }

        //Gateway --> Client
        if let Err(e) = receiver_loop(stream, to_client, event_listeners_clone_receiver).await {
            return Err(e);
        };

        Ok(WsConn {
            //last_sequence,
            to_gateway,
            from_gateway,
            event_listeners,
        })
    }
}

fn get_last_seq(last_sequence: &Arc<AtomicU64>) -> Option<u64> {
    // Increase the last sequence and use the previous value in the request
    match last_sequence.fetch_add(1, Ordering::Relaxed) {
        0 => None,
        v => Some(v),
    }
}

//listen for client commands and transform those to TungsteniteMessage and send to gateway
async fn sender_loop(
    last_sequence: Arc<AtomicU64>,
    mut sink: SplitSink<WebSocket, TungsteniteMessage>,
    mut from_client: Receiver<Command>,
) -> HassResult<()> {
    task::spawn(async move {
        //Fuse the stream such that poll_next will never again be called once it has finished.
        //let mut fused = from_client.fuse();
        loop {
            match from_client.recv().await {
                Some(item) => match item {
                    Command::Close => {
                        return sink
                            .send(TungsteniteMessage::Close(None))
                            .await
                            .map_err(|_| HassError::ConnectionClosed);
                    }
                    Command::AuthInit(auth) => {
                        // Transform command to TungsteniteMessage
                        let cmd = Command::AuthInit(auth).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::Ping(mut ping) => {
                        ping.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::Ping(ping).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::SubscribeEvent(mut subscribe) => {
                        subscribe.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::SubscribeEvent(subscribe).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::Unsubscribe(mut unsubscribe) => {
                        unsubscribe.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::Unsubscribe(unsubscribe).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::GetConfig(mut getconfig) => {
                        getconfig.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::GetConfig(getconfig).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::GetStates(mut getstates) => {
                        getstates.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::GetStates(getstates).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::GetServices(mut getservices) => {
                        getservices.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::GetServices(getservices).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::GetPanels(mut getpanels) => {
                        getpanels.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::GetServices(getpanels).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                    Command::CallService(mut callservice) => {
                        callservice.id = get_last_seq(&last_sequence);

                        // Transform command to TungsteniteMessage
                        let cmd = Command::CallService(callservice).to_tungstenite_message();

                        // Send the message to gateway
                        if let Err(e) = sink.send(cmd).await {
                            return Err(HassError::from(e));
                        }
                    }
                },
                None => {}
            }
        }
    });

    Ok(())
}

//listen for gateway responses and either send to client the response or execute the defined closure for Event subscribtion
async fn receiver_loop(
    //    last_sequence: Arc<AtomicU64>,
    mut stream: SplitStream<WebSocket>,
    to_client: Sender<HassResult<Response>>,
    event_listeners: Arc<Mutex<HashMap<u64, Box<dyn Fn(WSEvent) + Send>>>>,
) -> HassResult<()> {
    task::spawn(async move {
        loop {
            match stream.next().await {
                Some(Ok(item)) => match item {
                    TungsteniteMessage::Text(data) => {
                        // info!("{}", data);

                        //Serde: The tag identifying which variant we are dealing with is now inside of the content,
                        // next to any other fields of the variant
                        let payload: Result<Response, HassError> = serde_json::from_str(&data)
                            .map_err(|_| HassError::UnknownPayloadReceived);

                        //Match on payload, and act accordingly, like execute the client defined closure if any Event received
                        match payload {
                            Ok(value) => match value {
                                Response::Event(event) => {
                                    let mut table = event_listeners.lock().await;

                                    match table.get_mut(&event.id) {
                                        Some(client_func) => {
                                            //execute client closure
                                            client_func(event);
                                        }
                                        None => todo!("send unsubscribe request"),
                                    }
                                }
                                _ => to_client.send(Ok(value)).await.unwrap(),
                            },
                            Err(error) => to_client.send(Err(error)).await.unwrap(),
                        };
                    }
                    _ => {}
                },

                Some(Err(error)) => match to_client.send(Err(HassError::from(&error))).await {
                    //send the error to client ("unexpected message format, like a new error")
                    Ok(_r) => {}
                    Err(_e) => {}
                },
                None => {}
            }
        }
    });
    Ok(())
}
