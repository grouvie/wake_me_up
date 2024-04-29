use super::data::models::AddDeviceModel;
use crate::model::{controller::ModelController, data::models::DeviceModel};
use crate::proto::add_device_request::AddDeviceRequestProto;
use crate::{
    database::data::client::Client,
    error::{MyError, MyResult},
};

impl ModelController {
    pub(crate) async fn add_device(
        &self,
        user_id: i32,
        device: AddDeviceRequestProto,
    ) -> MyResult<()> {
        let mut client = Client::new(self.client.pool.clone()).await;

        client
            .add_device(user_id, AddDeviceModel::from(&device))
            .await
    }
    pub(crate) async fn get_devices(&self, user_id: i32) -> MyResult<Vec<DeviceModel>> {
        let mut client = Client::new(self.client.pool.clone()).await;

        let devices = client.get_devices(user_id).await?;

        let devices = devices.iter().map(DeviceModel::from).collect();

        Ok(devices)
    }
    pub(crate) async fn does_user_own_device(&self, user_id: i32, device_id: i32) -> MyResult<()> {
        let client = Client::new(self.client.pool.clone()).await;

        let owner = client.does_user_own_device(user_id, device_id).await?;

        if owner {
            Ok(())
        } else {
            Err(MyError::UserDoesNotOwnDevice { user_id, device_id })
        }
    }
    pub(crate) async fn delete_device(&self, device_id: i32) -> MyResult<()> {
        let client = Client::new(self.client.pool.clone()).await;

        client.delete_device(device_id).await
    }
    pub(crate) async fn patch_device(
        &self,
        device_id: i32,
        device: AddDeviceRequestProto,
    ) -> MyResult<()> {
        let client = Client::new(self.client.pool.clone()).await;

        client
            .patch_device(device_id, AddDeviceModel::from(&device))
            .await
    }
}
