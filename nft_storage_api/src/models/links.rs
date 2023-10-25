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
pub struct Links {
    #[serde(rename = "ipfs", skip_serializing_if = "Option::is_none")]
    pub ipfs: Option<String>,
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<String>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<crate::models::LinksFileInner>>,
}

impl Links {
    pub fn new() -> Links {
        Links {
            ipfs: None,
            http: None,
            file: None,
        }
    }
}