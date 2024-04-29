use crate::error::CommandResult;
use crate::model::controller::ModelController;
use crate::proto::add_device_request::AddDeviceRequestProto;
use crate::proto::basic_response::BasicResponseProto;
use crate::proto::device::DevicesProto;
use prost::{bytes::Bytes, Message};
use serde::{Deserialize, Serialize};
use std::io::Read;
use tauri_plugin_http::reqwest;

#[derive(Serialize, Deserialize)]
pub(crate) struct Device {
    id: i32,
    name: String,
    mac_address: String,
}

impl ModelController {
    pub async fn patch_device(&self, id: i32, name: &str, mac_address: &str) -> CommandResult<()> {
        let patch_device_request_proto = AddDeviceRequestProto {
            name: name.to_string(),
            mac_address: mac_address.to_string(),
        };

        let payload = patch_device_request_proto.encode_to_vec();

        let auth_header = self.auth_header.lock().await.clone();

        let request: Vec<u8> = self
            .api_client
            .request(
                &format!("device/{id}"),
                reqwest::Method::PATCH,
                payload,
                auth_header,
            )
            .await?;

        let response: BasicResponseProto = BasicResponseProto::decode(Bytes::from(request))?;

        println!("Success: {}", response.success);

        Ok(())
    }
    pub async fn add_device(&self, name: &str, mac_address: &str) -> CommandResult<()> {
        let patch_device_request_proto = AddDeviceRequestProto {
            name: name.to_string(),
            mac_address: mac_address.to_string(),
        };

        let payload = patch_device_request_proto.encode_to_vec();

        let auth_header = self.auth_header.lock().await.clone();

        let request: Vec<u8> = self
            .api_client
            .request(
                &format!("devices"),
                reqwest::Method::POST,
                payload,
                auth_header,
            )
            .await?;

        let response: BasicResponseProto = BasicResponseProto::decode(Bytes::from(request))?;

        println!("Success: {}", response.success);

        Ok(())
    }
    pub async fn get_devices(&self) -> CommandResult<Vec<Device>> {
        let auth_header = self.auth_header.lock().await.clone();

        let payload = Vec::new();

        let request: Vec<u8> = self
            .api_client
            .request(
                &format!("devices"),
                reqwest::Method::GET,
                payload,
                auth_header,
            )
            .await?;

        let response: DevicesProto = DevicesProto::decode(Bytes::from(request))?;

        let devices = response
            .devices
            .iter()
            .map(|device| Device {
                id: device.device_id.clone(),
                name: device.name.clone(),
                mac_address: device.mac_address.clone(),
            })
            .collect();

        Ok(devices)
    }
    pub async fn delete_device(&self, id: i32) -> CommandResult<()> {
        let payload = Vec::new();

        let auth_header = self.auth_header.lock().await.clone();

        let request: Vec<u8> = self
            .api_client
            .request(
                &format!("device/{id}"),
                reqwest::Method::DELETE,
                payload,
                auth_header,
            )
            .await?;

        let response: BasicResponseProto = BasicResponseProto::decode(Bytes::from(request))?;

        println!("Success: {}", response.success);

        Ok(())
    }
    pub async fn wake_device(&self, id: i32) -> CommandResult<bool> {
        let payload = Vec::new();

        let auth_header = self.auth_header.lock().await.clone();

        let request: Vec<u8> = self
            .api_client
            .request(
                &format!("wake_up/{id}"),
                reqwest::Method::GET,
                payload,
                auth_header,
            )
            .await?;

        let response: BasicResponseProto = BasicResponseProto::decode(Bytes::from(request))?;

        println!("Success: {}", response.success);

        Ok(response.success)
    }
}
