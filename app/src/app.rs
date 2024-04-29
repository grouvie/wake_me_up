use leptonic::components::prelude::*;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::*;

use crate::devices::Devices;
use crate::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Wake Me Up"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Title text="Wake Me Up"/>

        <Root default_theme=LeptonicTheme::Dark>
            <Router>
                <nav>
                    /* ... */
                </nav>
                <main>
                    // all our routes will appear inside <main>
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/devices" view=Devices/>
                    /* ... */
                    </Routes>
                </main>
            </Router>
        </Root>
    }
}
