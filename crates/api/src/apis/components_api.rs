/*
 * tdp-server
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_component_api_v1_service_service_id_component_component_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetComponentApiV1ServiceServiceIdComponentComponentIdGetError {
    Status401(),
    Status403(),
    Status400(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`patch_component_api_v1_service_service_id_component_component_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchComponentApiV1ServiceServiceIdComponentComponentIdPatchError {
    Status401(),
    Status403(),
    Status400(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_component_api_v1_service_service_id_component_component_id_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutComponentApiV1ServiceServiceIdComponentComponentIdPutError {
    Status401(),
    Status403(),
    Status400(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Gets component identified by its id
pub async fn get_component_api_v1_service_service_id_component_component_id_get(configuration: &configuration::Configuration, service_id: &str, component_id: &str) -> Result<crate::models::Component, Error<GetComponentApiV1ServiceServiceIdComponentComponentIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/service/{service_id}/component/{component_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), component_id=crate::apis::urlencode(component_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::AUTHORIZATION, String::from("Bearer ") + token);
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetComponentApiV1ServiceServiceIdComponentComponentIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Modifies a component definition.
pub async fn patch_component_api_v1_service_service_id_component_component_id_patch(configuration: &configuration::Configuration, service_id: &str, component_id: &str, component_update: crate::models::ComponentUpdate) -> Result<crate::models::ComponentUpdateResponse, Error<PatchComponentApiV1ServiceServiceIdComponentComponentIdPatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/service/{service_id}/component/{component_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), component_id=crate::apis::urlencode(component_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&component_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PatchComponentApiV1ServiceServiceIdComponentComponentIdPatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets a component definition.
pub async fn put_component_api_v1_service_service_id_component_component_id_put(configuration: &configuration::Configuration, service_id: &str, component_id: &str, component_update: crate::models::ComponentUpdate) -> Result<crate::models::ComponentUpdateResponse, Error<PutComponentApiV1ServiceServiceIdComponentComponentIdPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/service/{service_id}/component/{component_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), component_id=crate::apis::urlencode(component_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&component_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutComponentApiV1ServiceServiceIdComponentComponentIdPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

