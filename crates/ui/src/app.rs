use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use yew_oauth2::openid::*;
use yew_oauth2::prelude::*;

use tdp_api::apis::configuration::Configuration;

use crate::components::Login;
use crate::components::Logout;
use crate::components::Navbar;
use crate::routes::{switch, Route};

#[function_component(Application)]
pub fn app() -> Html {
    let config = Config {
        client_id: "tdp_server".into(),
        issuer_url: "http://localhost:8080/auth/realms/tdp_server".into(),
        additional: Default::default(),
    };
    let scopes = vec![
        "openid".to_owned(),
        "tdp_server:read".to_owned(),
        "tdp_server:write".to_owned(),
        "tdp_server:execute".to_owned(),
    ];
    let services = vec!["hdfs".into(), "yarn".into(), "hive".into()];
    html! {
        <>
            <OAuth2
                {config}
                scopes={scopes}
            >
                <BrowserRouter>
                    <Authenticated>
                        <div class={classes!("max-h-screen", "flex")}>
                            <div class={classes!("")} >
                            <Navbar services={services} />
                            </div>
                            <main class={classes!("h-screen", "overflow-y-auto", "w-full")}>
                                <ConfigurationContext>
                                    <Switch<Route> render={switch} />
                                </ConfigurationContext>
                                <Logout />
                            </main>
                        </div>
                    </Authenticated>
                    <NotAuthenticated><Login /></NotAuthenticated>
                </BrowserRouter>
           </OAuth2>
       </>
    }
}

/// Properties for the [`Authenticated`] component
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    /// The children to show then the context is authenticated.
    pub children: Children,
}

#[function_component(ConfigurationContext)]
fn configuration_context(props: &Props) -> Html {
    let auth = use_context::<OAuth2Context>();
    if let Some(auth) = auth {
        if let OAuth2Context::Authenticated(auth) = auth {
            let configuration = Rc::from(ConfigurationPEq(Configuration {
                base_path: String::from("http://localhost:8000"),
                oauth_access_token: Some(auth.access_token.clone()),
                bearer_access_token: Some(auth.access_token),
                ..Configuration::default()
            }));

            html! {
                <ContextProvider<Rc<ConfigurationPEq>> context={configuration}>
                    { for props.children.iter() }
                </ContextProvider<Rc<ConfigurationPEq>>>
            }
        } else {
            html! {
                "Not authenticated"
            }
        }
    } else {
        html! {"Auth context missing"}
    }
}

#[derive(Clone, Debug)]
pub struct ConfigurationPEq(pub Configuration);

// Gross hack, don't do this at home
impl PartialEq for ConfigurationPEq {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
