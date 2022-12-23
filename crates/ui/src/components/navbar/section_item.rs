use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct SectionItemProps {
    #[prop_or_default]
    pub item: AttrValue,
    pub to: Route,
}

#[function_component(SectionItem)]
pub fn section_item(props: &SectionItemProps) -> Html {
    let route: Option<Route> = use_route();
    let is_selected_classes = {
        let item = props.item.clone();
        route
            .and_then(|route| {
                if let Route::Service { id } = route {
                    if item == id {
                        return Some(classes!("text-white", "bg-gray-800"));
                    }
                }
                None
            })
            .or(Some(classes!("text-gray-400")))
    };

    html! {
        <div>
            <Link<Route> to={props.to.clone()} >
                <div class={classes!(
                    "pl-6", "py-[0.2rem]", "hover:bg-gray-800",
                    is_selected_classes
                )}>
                    {props.item.clone()}
                </div>
            </Link<Route>>
        </div>
    }
}
