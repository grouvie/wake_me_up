use tokio::sync::mpsc::Sender;
use tracing::error;

use crate::{
    database::data::client::Client,
    error::{MyError, MyResult},
    model::controller::ModelController,
    proto::{
        device::DeviceProto,
        wake_up::{wake_up_proto::WakeUpMessage, WakeUpProto, WakeUpRequestProto},
    },
};

impl ModelController {
    pub async fn wake_up_device(
        &self,
        user_id: i32,
        device_id: i32,
        sender: Sender<bool>,
    ) -> MyResult<()> {
        let client = Client::new(self.client.pool.clone()).await;

        let device = client.get_device(device_id).await?;

        let wake_up_proto = WakeUpProto {
            wake_up_message: Some(WakeUpMessage::WakeUpRequest(WakeUpRequestProto {
                device: DeviceProto {
                    device_id,
                    name: device.name,
                    mac_address: device.mac_address,
                },
            })),
        };

        let connected_clients = self.connected_clients.lock().await;

        if let Some(sender) = connected_clients.get(&user_id) {
            if let Err(e) = sender.send(wake_up_proto).await {
                error!(
                    "Failed to send WakeUpProto over Websocket connection: {}",
                    e
                );
                return Err(MyError::FailedToSendWakeUpProto {
                    error: e.to_string(),
                });
            }
        } else {
            error!("Sender not found for user_id: {}", user_id);
            return Err(MyError::NoSenderFoundForUser { user_id });
        }

        let mut wake_up_sender = self.wake_up_sender.lock().await;

        wake_up_sender.insert(user_id, sender);

        Ok(())
    }
}
