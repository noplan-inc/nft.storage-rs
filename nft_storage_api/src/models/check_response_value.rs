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
pub struct CheckResponseValue {
    /// Self-describing content-addressed identifiers for distributed systems. Check [spec](https://github.com/multiformats/cid) for more info.
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "pin", skip_serializing_if = "Option::is_none")]
    pub pin: Option<Box<crate::models::Pin>>,
    #[serde(rename = "deals", skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<crate::models::Deal>>,
}

impl CheckResponseValue {
    pub fn new() -> CheckResponseValue {
        CheckResponseValue {
            cid: None,
            pin: None,
            deals: None,
        }
    }
}

impl Default for CheckResponseValue {
    fn default() -> Self {
        Self::new()
    }
}
