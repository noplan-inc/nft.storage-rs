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
pub struct DidGet200Response {
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    /// NFT Storage DID.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DidGet200Response {
    pub fn new() -> DidGet200Response {
        DidGet200Response {
            ok: None,
            value: None,
        }
    }
}

impl Default for DidGet200Response {
    fn default() -> Self {
        Self::new()
    }
}
