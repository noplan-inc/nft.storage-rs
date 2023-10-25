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
pub struct ForbiddenErrorResponseError {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
}

impl ForbiddenErrorResponseError {
    pub fn new() -> ForbiddenErrorResponseError {
        ForbiddenErrorResponseError {
            name: None,
            message: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Message {
    #[serde(rename = "Token is not valid")]
    TokenIsNotValid,
    #[serde(rename = "Session expired")]
    SessionExpired,
}

impl Default for Message {
    fn default() -> Message {
        Self::TokenIsNotValid
    }
}
