use yew::prelude::*;

use yew_oauth2::openid::*;
use yew_oauth2::prelude::*;

#[function_component(Logout)]
pub fn logout() -> Html {
    let agent = use_auth_agent().expect("Must be nested inside an OAuth2 component");

    let onclick = Callback::from(move |_: MouseEvent| {
        if let Err(err) = agent.logout() {
            log::warn!("Failed to logout: {err}");
        }
    });

    html! {
        <button {onclick}> {"Log out"}</button>
    }
}
