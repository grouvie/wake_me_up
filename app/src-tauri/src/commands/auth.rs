use crate::{error::CommandResult, model::controller::ModelController};
use tauri::Manager;

#[tauri::command(async)]
pub async fn login(handle: tauri::AppHandle, username: &str, password: &str) -> CommandResult<()> {
    let mc = handle.state::<ModelController>();
    mc.login(username, password).await
}
