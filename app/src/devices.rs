use gloo_timers::future::TimeoutFuture;
use icondata::FiEdit;
use leptonic::components::prelude::*;
use leptos::{
    error::{Error, Result},
    logging::log,
    *,
};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::{f32::consts::E, fmt::Write};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name="invoke", js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_add_update_delete_device(cmd: &str, args: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name="invoke", js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_wake_get_devices(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

fn edit(
    devices: ReadSignal<Vec<Device>>,
    id: i32,
    set_show_edit_device_modal: WriteSignal<bool>,
    set_edit_id: WriteSignal<i32>,
    set_edit_name: WriteSignal<String>,
    set_edit_mac_address: WriteSignal<String>,
) {
    let index = devices
        .get()
        .iter()
        .position(|device| device.id == id)
        .unwrap();

    let device = devices.get().get(index).unwrap().clone();

    set_edit_id.set(device.id);
    set_edit_name.set(device.name);
    set_edit_mac_address.set(device.mac_address);
    set_show_edit_device_modal.set(true);
}

fn wake(
    devices: ReadSignal<Vec<Device>>,
    id: i32,
    set_show_wake_device_modal: WriteSignal<bool>,
    set_wake_device_id: WriteSignal<i32>,
    set_wake_device_name: WriteSignal<String>,
) {
    let index = devices
        .get()
        .iter()
        .position(|device| device.id == id)
        .unwrap();

    let device = devices.get().get(index).unwrap().clone();

    set_wake_device_id.set(id);
    set_wake_device_name.set(device.name);
    set_show_wake_device_modal.set(true);
}

#[derive(Clone, Serialize, Deserialize)]
struct Device {
    id: i32,
    name: String,
    mac_address: String,
}

#[derive(Serialize, Deserialize)]
struct GetDevicesArgs {}

#[derive(Serialize, Deserialize)]
struct WakeDeleteDeviceArgs {
    id: i32,
}

#[derive(Serialize, Deserialize)]
struct AddDeviceArgs<'a> {
    name: &'a str,
    #[allow(non_snake_case)]
    macAddress: &'a str,
}

#[derive(Serialize, Deserialize)]
struct UpdateDeviceArgs<'a> {
    id: i32,
    name: &'a str,
    #[allow(non_snake_case)]
    macAddress: &'a str,
}

#[component]
pub fn Devices() -> impl IntoView {
    // -- UPDATE DEVICE
    let (show_update_device_modal, set_show_update_device_modal) = create_signal(false);
    let (update_id, set_update_id) = create_signal(0);
    let (update_name, set_update_name) = create_signal("".to_string());
    let (update_mac_address, set_update_mac_address) = create_signal("".to_string());

    // -- ADD DEVICE
    let (show_add_device_modal, set_show_add_device_modal) = create_signal(false);
    let (add_name, set_add_name) = create_signal("".to_string());
    let (add_mac_address, set_add_mac_address) = create_signal("".to_string());

    // -- WAKE DEVICE
    let (show_wake_device_modal, set_show_wake_device_modal) = create_signal(false);
    let (wake_device_id, set_wake_device_id) = create_signal(0);
    let (wake_device_name, set_wake_device_name) = create_signal("".to_string());

    // -- DEVICES
    let (devices, set_devices) = create_signal(Vec::new());

    // -- ALERT
    let (show_error_alert, set_show_error_alert) = create_signal(false);
    let (error_alert_message, set_error_alert_message) = create_signal("".to_string());

    let show_error_alert_banner = move |error| {
        spawn_local(async move {
            set_show_error_alert.set(true);
            set_error_alert_message.set(error);
            TimeoutFuture::new(2_500).await;
            set_show_error_alert.set(false);
            TimeoutFuture::new(1_000).await;
            set_error_alert_message.set("".to_string());
        });
    };

    let (show_success_alert, set_show_success_alert) = create_signal(false);
    let (success_alert_message, set_success_alert_message) = create_signal("".to_string());

    let show_success_alert_banner = move |message| {
        spawn_local(async move {
            set_show_success_alert.set(true);
            set_success_alert_message.set(message);
            TimeoutFuture::new(2_500).await;
            set_show_success_alert.set(false);
            TimeoutFuture::new(1_000).await;
            set_success_alert_message.set("".to_string());
        });
    };

    let get_devices = move || {
        spawn_local(async move {
            let args = to_value(&GetDevicesArgs {}).unwrap();
            let result = invoke_wake_get_devices("get_devices", args).await;
            match result {
                Ok(devices) => {
                    let result: Result<Vec<Device>, _> = from_value(devices);
                    match result {
                        Ok(devices) => set_devices.set(devices),
                        Err(error) => {
                            set_devices.set(Vec::new());
                            let error = error.to_string();
                            log!("{error}");
                            show_error_alert_banner(error);
                        }
                    }
                }
                Err(error) => {
                    set_devices.set(Vec::new());
                    let error = error.as_string().unwrap_or_default();
                    log!("{error}");
                    show_error_alert_banner(error);
                }
            }
        });
    };

    let update_device = move || {
        spawn_local(async move {
            let device_id: i32 = update_id.get_untracked();
            if device_id.is_negative() {
                return;
            }
            let device_name = update_name.get_untracked();
            if device_name.is_empty() {
                return;
            }
            let device_mac_address = update_mac_address.get_untracked();
            if device_mac_address.is_empty() {
                return;
            }

            let args = to_value(&UpdateDeviceArgs {
                id: device_id,
                name: &device_name,
                macAddress: &device_mac_address,
            })
            .unwrap();
            let result = invoke_add_update_delete_device("update_device", args).await;
            match result {
                Ok(_) => {
                    log! {"Success!"};
                    get_devices();
                }
                Err(error) => {
                    let error = error.as_string().unwrap_or_default();
                    log!("{error}");
                    show_error_alert_banner(error);
                }
            }
            set_show_update_device_modal.set(false);
        });
    };

    let delete_device = move || {
        spawn_local(async move {
            let device_id: i32 = update_id.get_untracked();
            if device_id.is_negative() {
                return;
            }

            let args = to_value(&WakeDeleteDeviceArgs { id: device_id }).unwrap();
            let result = invoke_add_update_delete_device("delete_device", args).await;
            match result {
                Ok(_) => {
                    log! {"Success!"};
                    get_devices();
                }
                Err(error) => {
                    let error = error.as_string().unwrap_or_default();
                    log!("{error}");
                    show_error_alert_banner(error);
                }
            }
            set_show_update_device_modal.set(false);
        });
    };

    let add_device = move || {
        spawn_local(async move {
            let device_name = add_name.get_untracked();
            if device_name.is_empty() {
                return;
            }
            let device_mac_address = add_mac_address.get_untracked();
            if device_mac_address.is_empty() {
                return;
            }

            let args = to_value(&AddDeviceArgs {
                name: &device_name,
                macAddress: &device_mac_address,
            })
            .unwrap();

            set_add_name.set("".to_string());
            set_add_mac_address.set("".to_string());

            let result = invoke_add_update_delete_device("add_device", args).await;
            match result {
                Ok(_) => {
                    log! {"Success!"};
                    get_devices();
                }
                Err(error) => {
                    let error = error.as_string().unwrap_or_default();
                    log!("{error}");
                    show_error_alert_banner(error);
                }
            }
            set_show_add_device_modal.set(false);
        });
    };

    let wake_device = move || {
        spawn_local(async move {
            let device_id: i32 = wake_device_id.get_untracked();
            if device_id.is_negative() {
                return;
            }

            let args = to_value(&WakeDeleteDeviceArgs { id: device_id }).unwrap();
            let result = invoke_wake_get_devices("wake_device", args).await;
            match result {
                Ok(value) => {
                    if let Some(success) = value.as_bool() {
                        if success {
                            log! {"Success!"};
                            get_devices();
                            show_success_alert_banner(
                                "Magic packet to wake up device sent successfully".to_string(),
                            );
                        } else {
                            show_error_alert_banner("The device was not woken up".to_string());
                        }
                    };
                }
                Err(error) => {
                    let error = error.as_string().unwrap_or_default();
                    log!("{error}");
                    show_error_alert_banner(error);
                }
            }
            set_show_wake_device_modal.set(false);
        });
    };

    let cancel = move || {
        spawn_local(async move {
            // -- Set update signals to empty
            set_update_id.set(0);
            set_update_name.set("".to_string());
            set_update_mac_address.set("".to_string());

            // -- Set add signals to empty
            set_add_name.set("".to_string());
            set_add_mac_address.set("".to_string());

            // -- Hide Modals
            set_show_update_device_modal.set(false);
            set_show_add_device_modal.set(false);
            set_show_wake_device_modal.set(false);
        });
    };

    get_devices();

    view! {
            <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100vh; min-width: 100vw; background: white;">
                {move || devices.get().into_iter().map(|device| view!{
                    <div class="device-card">
                        <div class="header">
                            <span class="device-id">#{device.id}</span>
                        </div>
                        <div class="content">
                            <div class="info">
                                <h3 class="name">{device.name.clone()}</h3>
                                <h3 class="mac-address">{device.mac_address}</h3>
                            </div>
                            <div class="icon-button" on:click=move|_|edit(devices, device.id, set_show_update_device_modal, set_update_id, set_update_name, set_update_mac_address)>
                                <Icon icon=icondata::FiEdit style="font-size: 1em;"/>
                            </div>
                        </div>
                        <Button style="padding: 0.2em; margin-top: 0.4em; width: 100%;" color=ButtonColor::Success on:click=move|_|wake(devices, device.id, set_show_wake_device_modal, set_wake_device_id, set_wake_device_name) on_press=move|_| {} >"Wake Up"</Button>
                    </div>
                }).collect_view()}
                <Modal show_when=show_update_device_modal>
                    <ModalHeader>
                        <ModalTitle>"Edit Device"</ModalTitle>
                    </ModalHeader>
                    <ModalBody>
                        <div>
                            <Label>"Name"</Label>
                            <TextInput style="margin-top: 0.2em;" get=update_name set=set_update_name/>
                        </div>
                        <div>
                            <Label style="margin-top: 1em;">"MAC Address"</Label>
                            <TextInput style="margin-top: 0.2em;" get=update_mac_address set=set_update_mac_address/>
                        </div>
                    </ModalBody>
                    <ModalFooter>
                        <ButtonWrapper>
                            <Button on_press=move |_| delete_device() color=ButtonColor::Danger>"Delete"</Button>
                            <Button on_press=move|_| cancel() color=ButtonColor::Secondary>"Cancel"</Button>
                            <Button on_press=move|_| update_device() color=ButtonColor::Success>"Save"</Button>
                        </ButtonWrapper>
                    </ModalFooter>
                </Modal>
                <Modal show_when=show_add_device_modal>
                    <ModalHeader>
                        <ModalTitle>"Add Device"</ModalTitle>
                    </ModalHeader>
                    <ModalBody>
                        <div>
                            <Label>"Name"</Label>
                            <TextInput style="margin-top: 0.2em;" get=add_name set=set_add_name/>
                        </div>
                        <div>
                            <Label style="margin-top: 1em;">"MAC Address"</Label>
                            <TextInput style="margin-top: 0.2em;" get=add_mac_address set=set_add_mac_address/>
                        </div>
                    </ModalBody>
                    <ModalFooter>
                        <ButtonWrapper>
                            <Button on_press=move|_| cancel() color=ButtonColor::Secondary>"Cancel"</Button>
                            <Button on_press=move|_| add_device() color=ButtonColor::Success>"Save"</Button>
                        </ButtonWrapper>
                    </ModalFooter>
                </Modal>
                <Modal show_when=show_wake_device_modal>
                    <ModalHeader><ModalTitle>"Wake Up?"</ModalTitle></ModalHeader>
                    <p style="align-items: left;">"Do you want to wake "<b>{wake_device_name.get()}</b>" up?"</p>
                    <ModalFooter>
                        <ButtonWrapper>
                            <Button on_press=move |_| cancel() color=ButtonColor::Secondary>"Cancel"</Button>
                            <Button on_press=move|_| wake_device() color=ButtonColor::Success>"Wake Up"</Button>
                        </ButtonWrapper>
                    </ModalFooter>
                </Modal>
                <div class="alert-error" class:show=move || show_error_alert.get()>
                    <div class="alert-title">
                        <span>Error</span>
                        <AlertIcon variant=AlertVariant::Danger style="margin-left: 1em;" />
                    </div>
                    <div class="alert-content">{move ||error_alert_message.get()}!</div>
                </div>
                <div class="alert-success" class:show=move || show_success_alert.get()>
                    <div class="alert-title">
                        <span>Success</span>
                        <AlertIcon variant=AlertVariant::Success style="margin-left: 1em;" />
                    </div>
                    <div class="alert-content">{move ||success_alert_message.get()}!</div>
                </div>
                <div class="floating-btn" on:click=move|_| set_show_add_device_modal.set(true)>+</div>
            </Box>
    }
}
