/*
 * NFT Storage API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForbiddenErrorResponse {
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::ForbiddenErrorResponseError>>,
}

impl ForbiddenErrorResponse {
    pub fn new() -> ForbiddenErrorResponse {
        ForbiddenErrorResponse {
            ok: None,
            error: None,
        }
    }
}


