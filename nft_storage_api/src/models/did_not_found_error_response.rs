/*
 * NFT Storage API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DidNotFoundErrorResponse {
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::DidNotFoundErrorResponseError>>,
}

impl DidNotFoundErrorResponse {
    pub fn new() -> DidNotFoundErrorResponse {
        DidNotFoundErrorResponse {
            ok: None,
            error: None,
        }
    }
}

impl Default for DidNotFoundErrorResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DidNotFoundErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DidNotFoundErrorResponse: ok = {:?}, error = {:?}",
            self.ok, self.error
        )
    }
}
