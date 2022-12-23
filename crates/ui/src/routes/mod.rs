use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Service;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/service/:id")]
    Service { id: String },
    #[at("/deploy")]
    Deploy,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <h1>{"Home"}</h1>
        },
        Route::Service { id } => html! {
            <Service id={id} />
        },
        Route::Deploy => html! {},
        Route::NotFound => html! {
            <h1>{"No Found"}</h1>
        },
    }
}
