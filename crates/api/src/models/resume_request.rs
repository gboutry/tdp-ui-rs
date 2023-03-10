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
pub struct ResumeRequest {
    /// Deployment ID to resume from. (must be a failed deployment)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl ResumeRequest {
    pub fn new() -> ResumeRequest {
        ResumeRequest {
            id: None,
        }
    }
}


