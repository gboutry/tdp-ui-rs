use std::rc::Rc;
use tdp_api::apis::services_api::get_service_api_v1_service_service_id_get;
use tdp_api::models;

use yew::prelude::*;

use crate::{app::ConfigurationPEq, components::Editor, hooks::use_configuration};

#[derive(Properties, PartialEq)]
pub struct ServiceProps {
    #[prop_or_default]
    pub id: AttrValue,
}

#[function_component(Service)]
pub fn service(props: &ServiceProps) -> Html {
    let configuration = use_configuration();
    html! {
        if let Some(configuration) = configuration {
            <InternalService configuration={configuration} id={props.id.clone()} />
        } else {
            <></>
        }
    }
}

#[derive(Properties, PartialEq)]
struct InternalServiceProps {
    id: AttrValue,
    configuration: Rc<ConfigurationPEq>,
}

enum ServiceMessage {
    FetchService,
    ServiceFetched(Option<models::Service>),
    ParseYaml,
    YamlParsed(AttrValue),
}

struct InternalService {
    service: Option<models::Service>,
    yaml: AttrValue,
}

impl Component for InternalService {
    type Message = ServiceMessage;

    type Properties = InternalServiceProps;

    fn create(ctx: &Context<Self>) -> Self {
        let self_ = InternalService {
            service: None,
            yaml: AttrValue::from(""),
        };
        ctx.link().send_message(ServiceMessage::FetchService);
        self_
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ServiceMessage::FetchService => {
                log::debug!("Fetching service {}", &ctx.props().id);
                ctx.link().send_future({
                    let configuration = ctx.props().configuration.0.clone();
                    let id = ctx.props().id.clone();
                    async move {
                        let service =
                            get_service_api_v1_service_service_id_get(&configuration, &id)
                                .await
                                .ok();
                        ServiceMessage::ServiceFetched(service)
                    }
                });
                true
            }
            ServiceMessage::ServiceFetched(service) => {
                log::debug!("Service fecthed");
                self.service = service;
                ctx.link().send_message(ServiceMessage::ParseYaml);
                true
            }
            ServiceMessage::ParseYaml => {
                if let Some(service) = &self.service {
                    log::debug!("Parsing {} service's yaml", &service.id);
                    let yaml = AttrValue::from(serde_yaml::to_string(&service.variables).unwrap());
                    ctx.link().send_message(ServiceMessage::YamlParsed(yaml));
                }
                false
            }
            ServiceMessage::YamlParsed(yaml) => {
                log::debug!("Yaml parsed");
                self.yaml = yaml;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>
                { if let Some(service) = self.service.clone() {
                    html!(<h1>{ "Service page: " }{ service.id }</h1>)
                    } else {
                        html!(<></>)
                    }
                }
                </div>
                <Editor content={self.yaml.clone()} />
            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().id != old_props.id {
            ctx.link().send_message(ServiceMessage::FetchService);
            true
        } else {
            false
        }
    }
}
