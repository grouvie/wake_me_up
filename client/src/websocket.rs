use std::error::Error;

use futures_util::{future, StreamExt};
use prost::{bytes::Bytes, Message};
use reqwest::header::HeaderMap;
use tokio::pin;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::Request, protocol},
};

use crate::proto::{
    basic_response::BasicResponseProto,
    wake_up::{wake_up_proto::WakeUpMessage, WakeUpProto, WakeUpResponseProto},
};

pub async fn connect_websocket(
    host: &str,
    port: &str,
    headers: HeaderMap,
) -> Result<(), Box<dyn Error>> {
    let mut client_request = Request::builder()
        .uri(format!("ws://{}:{}/api/ws", host, port))
        .body(())?;

    client_request.headers_mut().extend(headers);

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    let (ws_stream, _) = connect_async(client_request).await?;

    println!("Connected to WebSocket Server");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = read.for_each(|message| async {
        let data = message.unwrap().into_data();

        if let Ok(wake_up_proto) = WakeUpProto::decode(Bytes::from(data)) {
            if let Some(WakeUpMessage::WakeUpRequest(wake_up_message)) =
                wake_up_proto.wake_up_message
            {
                println!("Waking up: \n{:#?}", wake_up_message.device);
                let mac_address = wake_up_message.device.mac_address;

                if let Some(mac_address) = parse_mac_address(&mac_address) {
                    println!("Successfully parsed MAC address: {:?}", mac_address);

                    // Create a magic packet (but don't send it yet)
                    let magic_packet = wake_on_lan::MagicPacket::new(&mac_address);

                    // Send the magic packet via UDP to the broadcast address 255.255.255.255:9 from 0.0.0.0:0
                    if let Err(error) = magic_packet.send() {
                        println!("{}", error);
                        return;
                    };

                    println!("Magic packet sent successfully.");
                    let wake_up_proto = WakeUpProto {
                        wake_up_message: Some(WakeUpMessage::WakeUpResponse(WakeUpResponseProto {
                            basic_response: BasicResponseProto { success: true },
                        })),
                    };
                    let payload = wake_up_proto.encode_to_vec();

                    if let Err(error) = stdin_tx.unbounded_send(protocol::Message::binary(payload))
                    {
                        println!("{}", error);
                    };
                } else {
                    println!("Invalid MAC address format");
                    let wake_up_proto = WakeUpProto {
                        wake_up_message: Some(WakeUpMessage::WakeUpResponse(WakeUpResponseProto {
                            basic_response: BasicResponseProto { success: false },
                        })),
                    };
                    let payload = wake_up_proto.encode_to_vec();

                    if let Err(error) = stdin_tx.unbounded_send(protocol::Message::binary(payload))
                    {
                        println!("{}", error);
                    };
                }
            }
        }
    });
    pin!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;

    Ok(())
}

fn parse_mac_address(mac_address_str: &str) -> Option<[u8; 6]> {
    let mut bytes = [0u8; 6];
    let hex_values: Vec<_> = mac_address_str
        .split(':')
        .map(|part| u8::from_str_radix(part, 16))
        .collect();

    if hex_values.len() == 6 {
        for (index, hex_result) in hex_values.into_iter().enumerate() {
            if let Ok(value) = hex_result {
                bytes[index] = value;
            } else {
                return None;
            }
        }
        Some(bytes)
    } else {
        None
    }
}
