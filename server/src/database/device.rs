use super::data::{client::Client, models::DeviceDB};
use crate::error::{MyError, MyResult};
use crate::model::data::models::AddDeviceModel;

impl Client {
    pub(crate) async fn add_device(
        &mut self,
        user_id: i32,
        add_device: AddDeviceModel,
    ) -> MyResult<()> {
        sqlx::query("INSERT into device (user_id, name, mac_address) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind(add_device.name)
            .bind(add_device.mac_address)
            .execute(&self.pool)
            .await
            .map_err(|e| MyError::Database {
                error: e.to_string(),
            })?;

        Ok(())
    }
    pub(crate) async fn get_devices(&mut self, user_id: i32) -> MyResult<Vec<DeviceDB>> {
        let devices = sqlx::query_as::<_, DeviceDB>("SELECT * FROM device WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| MyError::Database {
                error: e.to_string(),
            })?;

        Ok(devices)
    }
    pub(crate) async fn does_user_own_device(
        &self,
        user_id: i32,
        device_id: i32,
    ) -> MyResult<bool> {
        let result: bool = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM device WHERE user_id = $1 AND id = $2",
        )
        .bind(user_id)
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MyError::Database {
            error: e.to_string(),
        })?
        .map(|count| count > 0)
        .unwrap_or(false);

        Ok(result)
    }
    pub(crate) async fn delete_device(&self, device_id: i32) -> MyResult<()> {
        sqlx::query("DELETE FROM device WHERE id = $1")
            .bind(device_id)
            .execute(&self.pool)
            .await
            .map_err(|e| MyError::Database {
                error: e.to_string(),
            })?;
        Ok(())
    }
    pub(crate) async fn patch_device(
        &self,
        device_id: i32,
        device: AddDeviceModel,
    ) -> MyResult<()> {
        sqlx::query("UPDATE device SET name = $1, mac_address = $2 WHERE id = $3")
            .bind(&device.name)
            .bind(&device.mac_address)
            .bind(device_id)
            .execute(&self.pool)
            .await
            .map_err(|e| MyError::Database {
                error: e.to_string(),
            })?;

        Ok(())
    }
    pub(crate) async fn get_device(&self, device_id: i32) -> MyResult<DeviceDB> {
        let device = sqlx::query_as::<_, DeviceDB>("SELECT * FROM device WHERE id = $1")
            .bind(device_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| MyError::Database {
                error: e.to_string(),
            })?;

        Ok(device)
    }
}
