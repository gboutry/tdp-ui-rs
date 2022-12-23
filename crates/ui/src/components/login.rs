use yew::prelude::*;

use yew_oauth2::openid::*;
use yew_oauth2::prelude::*;

static BUTTON_CLASSES: &[&str] = &[
    "bg-white",
    "border",
    "border-gray-300",
    "text-gray-700",
    "hover:bg-gray-200",
    "rounded",
    "px-2.5",
    "py-1.5",
    "outline-none",
    "text-xl",
];

#[function_component(Login)]
pub fn login() -> Html {
    let agent = use_auth_agent().expect("Must be nested inside an OAuth2 component");

    let onclick = Callback::from(move |_| {
        if let Err(err) = agent.start_login() {
            log::warn!("Failed to start login: {err}");
        }
    });
    html! {
        <section class="h-screen">
            <div class={classes!("flex", "flex-row", "min-h-screen", "justify-center", "items-center")}>
                <button class={classes!(BUTTON_CLASSES)} {onclick}>{"LOG IN"}</button>
            </div>
        </section>
    }
}
