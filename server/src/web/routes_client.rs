use axum::{
    extract::{
        ws::{self, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::{headers, TypedHeader};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use prost::Message;
use std::{net::SocketAddr, ops::ControlFlow};
use tokio::sync::mpsc::{self, Receiver};
use tracing::{debug, error, info, trace};

use crate::{
    ctx::Ctx,
    error::MyResult,
    model::controller::ModelController,
    proto::wake_up::{wake_up_proto::WakeUpMessage, WakeUpProto},
};

pub fn routes(mc: ModelController) -> Router {
    Router::new().route("/ws", get(ws_handler)).with_state(mc)
}

async fn ws_handler(
    State(mc): State<ModelController>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ctx: Ctx,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> MyResult<impl IntoResponse> {
    debug!(">>> {:<12} - ws_handler", "HANDLER");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    let user_id = ctx.user_id();

    let (sender, receiver) = mpsc::channel(1000);

    // We get the connected_clients from our state and insert the newly connected client id
    // with the sender to send messages to this connection into the hashmap
    let mc_into = mc.clone();
    let mut connected_clients = mc.connected_clients.lock().await;
    connected_clients.insert(user_id, sender);

    info!("`{user_agent}` at {addr} with id {user_id} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, addr, receiver, mc_into, user_id)))
}

// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    mut receiver: Receiver<WakeUpProto>,
    mc: ModelController,
    user_id: i32,
) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(ws::Message::Ping(vec![1, 2, 3])).await.is_ok() {
        //println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    let mc_into = mc.clone();

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who, user_id, mc.clone())
                .await
                .is_break()
            {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }

    // By splitting socket we can send and receive at the same time.
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // This task will receive WakeUpProto messages from the api receiver and forward them
    // over the websocket connection
    let mut recv_task = tokio::spawn(async move {
        while let Some(wake_up_proto) = receiver.recv().await {
            let proto_bytes = wake_up_proto.encode_to_vec();
            if let Err(e) = ws_sender.send(ws::Message::Binary(proto_bytes)).await {
                error!(
                    "Failed to send WakeUpProto over Websocket connection: {}",
                    e
                );
            };
        }
    });

    // This second task will receive messages from the client and process them
    // Answering the client who made the api request is probably going to be implemented
    // at a later stage
    let mut ws_recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            // process message and break if instructed to do so
            if process_message(msg, who, user_id, mc.clone())
                .await
                .is_break()
            {
                break;
            }
        }
        who
    });

    tokio::select! {
        recv_a = (&mut recv_task) => {
            match recv_a {
                Ok(_) => debug!("First task is disconnecting"),
                Err(b) => error!("Error receiving messages {b:?}"),
            }
            ws_recv_task.abort();
        },
        recv_b = (&mut ws_recv_task) => {
            match recv_b {
                Ok(who) => debug!("{} is disconnecting", who),
                Err(b) => error!("Error receiving messages {b:?}"),
            }
            recv_task.abort();
        },
    }

    // We get the connected_clients from our state and remove the disconnected client id
    // from the hashmap
    let mut connected_clients = mc_into.connected_clients.lock().await;
    connected_clients.remove(&user_id);

    // returning from the handler closes the websocket connection
    info!("Websocket context {who} destroyed");
}

// helper to print contents of messages to stdout. Has special treatment for Close.
async fn process_message(
    msg: ws::Message,
    who: SocketAddr,
    user_id: i32,
    mc: ModelController,
) -> ControlFlow<(), ()> {
    match msg {
        ws::Message::Text(t) => {
            info!(">>> {who} sent str: {t:?}");
        }
        ws::Message::Binary(d) => {
            match WakeUpProto::decode(Bytes::from(d.clone())) {
                Ok(wake_up_proto) => match wake_up_proto.wake_up_message {
                    Some(wake_up_message) => match wake_up_message {
                        WakeUpMessage::WakeUpResponse(wake_up_response) => {
                            let wake_up_sender = mc.wake_up_sender.lock().await;
                            if let Some(sender) = wake_up_sender.get(&user_id) {
                                if let Err(e) =
                                    sender.send(wake_up_response.basic_response.success).await
                                {
                                    error!(
                                        "Failed to send WakeUpProto over Websocket connection: {}",
                                        e
                                    );
                                }
                            } else {
                                error!("Sender not found for user_id: {}", user_id);
                            }
                            debug!(">>> {} sent {:?}", who, wake_up_response.basic_response);
                        }
                        _ => {
                            debug!("Unsupported message type received")
                        }
                    },
                    None => {
                        debug!("Received empty wake_up_message")
                    }
                },
                Err(e) => {
                    debug!("Decoding WakeUpProto failed with {e}");
                }
            };
            trace!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        ws::Message::Close(c) => {
            if let Some(cf) = c {
                debug!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                debug!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        ws::Message::Pong(v) => {
            trace!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        ws::Message::Ping(v) => {
            trace!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
