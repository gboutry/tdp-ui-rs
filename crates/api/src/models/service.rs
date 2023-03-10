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
pub struct Service {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "components")]
    pub components: Vec<crate::models::Component>,
    /// A free form object used as a dictionnary in TDP sdk. You can use any kind of data understood as a JSON object
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}

impl Service {
    pub fn new(id: String, components: Vec<crate::models::Component>) -> Service {
        Service {
            id,
            components,
            variables: None,
        }
    }
}


