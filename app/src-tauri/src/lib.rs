#![feature(async_closure)]
use model::controller::ModelController;
use std::env;
use tauri::App;
use tauri::AppHandle;
use tauri::Manager;
mod client;
mod commands;
mod error;
mod model;

use commands::*;

// Include the proto modules, which are generated from protos/*.proto.
// It is important to maintain the same structure as in the proto.
pub mod proto {
    pub mod login_request {
        include!(concat!(env!("OUT_DIR"), "/proto.login_request.rs"));
    }
    pub mod login_response {
        include!(concat!(env!("OUT_DIR"), "/proto.login_response.rs"));
    }
    pub mod basic_response {
        include!(concat!(env!("OUT_DIR"), "/proto.basic_response.rs"));
    }
    pub mod add_device_request {
        include!(concat!(env!("OUT_DIR"), "/proto.add_device_request.rs"));
    }
    pub mod device {
        include!(concat!(env!("OUT_DIR"), "/proto.device.rs"));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    let base_url = env::var("BASE_URL").unwrap_or("http://127.0.0.1:8080/api".to_string());

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let builder = tauri::Builder::default().plugin(tauri_plugin_http::init());

            let mc = ModelController::new(&base_url).await;

            builder
                .manage(mc)
                .plugin(tauri_plugin_shell::init())
                .plugin(tauri_plugin_http::init())
                .invoke_handler(tauri::generate_handler![
                    auth::login,
                    device::update_device,
                    device::add_device,
                    device::get_devices,
                    device::delete_device,
                    device::wake_device,
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        });
}
