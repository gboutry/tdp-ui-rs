use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub icon: IconId,
    #[prop_or(AttrValue::from("Title"))]
    pub title: AttrValue,
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div>
            <div class={classes!("px-2", "py-2", "text-white", "flex", "items-center", "gap-2", "hover:bg-gray-800",)}>
                <Icon icon_id={props.icon} class={classes!("h-5", "w-5")} />
                {props.title.clone()}
            </div>
            { for props.children.iter() }
        </div>
    }
}
