use yew::prelude::*;
use yew_icons::IconId;

use crate::routes::Route;

mod section;
use section::Section;

mod section_item;
use section_item::SectionItem;

const LOGO_WIDTH: f32 = 126f32;
const LOGO_HEIGHT: f32 = 50f32;
const LOGO_RATIO: f32 = 0.8f32;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub services: Vec<AttrValue>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
        <aside class={classes!("h-screen", "text-xl", "w-72", "mr-0", "overflow-auto", "bg-gray-900", "flex", "flex-col")} aria-label="Sidebar">
          <div class={classes!("mb-3", "py-3", "self-center")}>
            <p class="flex">
              <img
                src="/assets/TDP_LOGO_INVERSE_notext.png"
                alt="tdp-logo"
                width={(LOGO_WIDTH * LOGO_RATIO).to_string()}
                height={(LOGO_HEIGHT * LOGO_RATIO).to_string()}
              />
              <span class={classes!("text-lg", "font-bold", "text-white")}>{"UI"}</span>
            </p>
          </div>
            <nav>
                <Section icon={IconId::HeroiconsSolidBeaker} title={"Services"}>
                    {for props.services.iter().map(|service| html!{
                        <SectionItem item={ service.clone() } to={Route::Service { id: service.to_string() }} />
                    })}
                </Section>
                <Section icon={IconId::HeroiconsSolidCog6Tooth} title={"Deployments"} >
                    <SectionItem item={"deploy"} to={Route::Deploy}/>
                </Section>
            </nav>
        </aside>
    }
}
