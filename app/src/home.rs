use leptonic::components::prelude::*;
use leptos::{
    error::{Error, Result},
    logging::log,
    *,
};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name="invoke", js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_login(cmd: &str, args: JsValue) -> Result<(), JsValue>;
}

#[derive(Serialize, Deserialize)]
struct LoginArgs<'a> {
    username: &'a str,
    password: &'a str,
}

#[component]
pub fn Home() -> impl IntoView {
    let (username, set_username) = create_signal("".to_owned());
    let (password, set_password) = create_signal("".to_owned());
    let (login_error, set_login_error) = create_signal(String::new());

    // create an effect to reactively update the login error text
    create_effect(move |prev_value| {
        // Retrieve current username and password values
        let (current_username, current_password) = (username.get(), password.get());

        // Compare current values with previous values
        if let Some((prev_username, prev_password)) = prev_value {
            if prev_username != current_username || prev_password != current_password {
                // Reset login error if values have changed
                set_login_error.set("".to_string());
            }
        }

        // Return current values for memoization
        (current_username, current_password)
    });

    let login = move || {
        spawn_local(async move {
            let username = username.get_untracked();
            if username.is_empty() {
                return;
            }
            let password = password.get_untracked();
            if password.is_empty() {
                return;
            }

            let args = to_value(&LoginArgs {
                username: &username,
                password: &password,
            })
            .unwrap();
            let result = invoke_login("login", args).await;
            match result {
                Ok(_) => {
                    let navigate = leptos_router::use_navigate();
                    navigate("/devices", Default::default());
                }
                Err(error) => set_login_error.set(error.as_string().unwrap().to_string()),
            }
        });
    };
    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100vh; min-width: 100vw; background: white;">
            <H1>"Wake me up!"</H1>

            <FormControl>
                <Label style="margin-top: 1em;">
                    "Username"
                </Label>
                <TextInput placeholder="Username" style="margin-top: 0.2em;" get=username set=set_username/>

                <Label style="margin-top: 1em;">
                    "Password"
                </Label>
                <PasswordInput placeholder="••••••••" style="margin-top: 0.2em;" get=password set=set_password/>

                <Button style="margin-top: 1em;" on_press=move|_| login()>
                "Login"
                </Button>
                <H5 style="margin-left: 0.2em; color: red;">{ move || login_error.get() }</H5>
            </FormControl>
        </Box>
    }
}
