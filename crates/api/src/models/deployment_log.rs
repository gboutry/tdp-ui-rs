/*
 * tdp-server
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeploymentLog {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    #[serde(rename = "targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
    #[serde(rename = "filter_expression", skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "restart", skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
    #[serde(rename = "state")]
    pub state: crate::models::StateEnum,
    #[serde(rename = "operations")]
    pub operations: Vec<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl DeploymentLog {
    pub fn new(id: i32, start_time: String, state: crate::models::StateEnum, operations: Vec<String>) -> DeploymentLog {
        DeploymentLog {
            id,
            sources: None,
            targets: None,
            filter_expression: None,
            start_time,
            end_time: None,
            restart: None,
            state,
            operations,
            user: None,
        }
    }
}


