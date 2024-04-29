use crate::{
    error::CommandResult,
    model::{controller::ModelController, device::Device},
};
use tauri::Manager;

#[tauri::command(async)]
pub async fn update_device(
    handle: tauri::AppHandle,
    id: i32,
    name: &str,
    mac_address: &str,
) -> CommandResult<()> {
    let mc = handle.state::<ModelController>();
    mc.patch_device(id, &name, &mac_address).await
}

#[tauri::command(async)]
pub async fn add_device(
    handle: tauri::AppHandle,
    name: &str,
    mac_address: &str,
) -> CommandResult<()> {
    let mc = handle.state::<ModelController>();
    mc.add_device(&name, &mac_address).await
}

#[tauri::command(async)]
pub async fn get_devices(handle: tauri::AppHandle) -> CommandResult<Vec<Device>> {
    let mc = handle.state::<ModelController>();
    mc.get_devices().await
}

#[tauri::command(async)]
pub async fn delete_device(handle: tauri::AppHandle, id: i32) -> CommandResult<()> {
    let mc = handle.state::<ModelController>();
    mc.delete_device(id).await
}

#[tauri::command(async)]
pub async fn wake_device(handle: tauri::AppHandle, id: i32) -> CommandResult<bool> {
    let mc = handle.state::<ModelController>();
    mc.wake_device(id).await
}
