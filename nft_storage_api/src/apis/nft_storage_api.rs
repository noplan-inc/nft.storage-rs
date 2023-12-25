/*
 * NFT Storage API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::path::Path;

use reqwest::{self, header, multipart};
use tokio::fs::File;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`check`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckError {
    Status404(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteError {
    Status401(crate::models::UnauthorizedErrorResponse),
    Status403(crate::models::ForbiddenErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`did_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DidGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListError {
    Status401(crate::models::UnauthorizedErrorResponse),
    Status403(crate::models::ForbiddenErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusError {
    Status401(crate::models::UnauthorizedErrorResponse),
    Status403(crate::models::ForbiddenErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`store`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoreError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::UnauthorizedErrorResponse),
    Status403(crate::models::ForbiddenErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ucan_token_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UcanTokenPostError {
    Status400(crate::models::DidNotFoundErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::UnauthorizedErrorResponse),
    Status403(crate::models::ForbiddenErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_did_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserDidPostError {
    Status400(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Includes the IPFS pinning state and the Filecoin deal state.
pub async fn check(
    configuration: &configuration::Configuration,
    cid: &str,
) -> Result<crate::models::CheckResponse, Error<CheckError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/check/{cid}",
        local_var_configuration.base_path,
        cid = crate::apis::urlencode(cid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CheckError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Stop storing the content with the passed CID on nft.storage. - Unpin the item from the underlying IPFS pinning service. - Cease renewals for expired Filecoin deals involving the CID.    ⚠️ This does not remove the content from the network.  - Does not terminate any established Filecoin deal. - Does not remove the content from other IPFS nodes in the network that already cached or pinned the CID.    Note: the content will remain available if another user has stored the CID with nft.storage.
pub async fn delete(
    configuration: &configuration::Configuration,
    cid: &str,
) -> Result<crate::models::DeleteResponse, Error<DeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/{cid}",
        local_var_configuration.base_path,
        cid = crate::apis::urlencode(cid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn did_get(
    configuration: &configuration::Configuration,
) -> Result<crate::models::DidGet200Response, Error<DidGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/did", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DidGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list(
    configuration: &configuration::Configuration,
    before: Option<&String>,
    limit: Option<i32>,
) -> Result<crate::models::ListResponse, Error<ListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(local_var_str) = before {
        local_var_req_builder =
            local_var_req_builder.query(&[("before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Includes the IPFS pinning state and the Filecoin deal state.
pub async fn status(
    configuration: &configuration::Configuration,
    cid: &str,
) -> Result<crate::models::GetResponse, Error<StatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/{cid}",
        local_var_configuration.base_path,
        cid = crate::apis::urlencode(cid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Store an [ERC-1155](https://eips.ethereum.org/EIPS/eip-1155)-compatible NFT as  a collection of content-addressed objects connected by IPFS CID links.  **Not recommended for performance sensitive applications**. While the `/store` endpoint is convenient, it is less performant than `/upload`, and might be deprecated in the future. If you would like to have a similar level of convenience with better performance, check out the Javascript client's [`store` method](https://nft.storage/docs/client/js/#store---store-erc1155-nft-data). We recommend constructing CAR files on the client wherever possible and using the `/upload` endpoint to store your data. This lets you compute the CID for your upload on the client and verify it against what the API returns.  The POST request accepts `multipart/form-data` content restricted to a body size of 100MB (see \"Size limitations\" below for more information). The request must contain a form field named `meta`.  The `meta` field must contain a JSON string that conforms to the [ERC-1155 metadata schema](https://eips.ethereum.org/EIPS/eip-1155#metadata).  Any field(s) inside the `meta` object can be substituted with an ipfs URL to a file(s), by providing a form data field with a name matching a (`.` delimited) property path and value containing the file content (in binary string or plain text depending on file format).  The name of the form data field containing the file content should be the \"path\" of the JSON field, using `.` to traverse nested objects.  For example, with a `meta` object of the form:    ```json   {     \"name\": \"Hello\",     \"image\": undefined,     \"properties\": {       \"videoClip\": undefined     }   }   ```  You must include form fields named `image` and `properties.videoClip` in your request body, with the content of the image and video files as the form field values.  All form fields other than `meta` must contain binary file data, and the field name will be used as a '.' delimited property path for the `meta` object, as described above. If the form field name matches a `meta` property with a non-falsy value, the request will be rejected with an error.  ### Mime Type Recommendations  The ERC-1155 metadata standard specifies that the `image` metadata field should reference a file with a content type of `image/_*`. An earlier version of this API enforced this as a requirement, but this proved to be incompatible with some existing systems and the requirement was relaxed, although you may see a warning when using the official JavaScript client.  We highly recommend that you only use content with an `image/_*` content type for your `image` field, and include other content types in the `properties` field as additional references.  ### Size limitations  The store endpoint is restricted to a total request body size of 100MB, which includes the metadata and all attached files. To store larger files, you can use the /upload endpoint with chunked CAR files (see \"/upload\").  ### Rate limits  This API imposes rate limits to ensure quality of service. You may receive a 429 \"Too many requests\" error if you make more than 30 requests with the same API token within a 10 second window. Upon receiving a response with a 429 status, clients should retry the failed request after a small delay. To avoid 429 responses, you may wish to implement client-side request throttling to stay within the limits (note: the JS client automatically does this).
pub async fn store(
    configuration: &configuration::Configuration,
    meta: Option<&str>,
) -> Result<crate::models::UploadResponse, Error<StoreError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/store", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = meta {
        local_var_form = local_var_form.text("meta", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = match local_var_req_builder.build() {
        Ok(req) => req,
        Err(e) => {
            return Err(Error::from(e));
        }
    };
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoreError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint returns a root UCAN for nft.storage. The UCAN will be valid for **two weeks** and has full capabilities. The JWT payload will looking similar to this:    ```json {   \"att\": [     {       \"with\": \"storage://did:key:z6MkheUPoHhYRS5LoHbbttpaTkkxvFFFUV5VPSziwTJmbb7D\",       \"can\": \"upload/_*\"     }   ],   \"exp\": 1646668606,   \"iss\": \"did:key:z6MkheUPoHhYRS5LoHbbttpaTkkxvFFFUV5VPSziwTJmbb7D\",   \"prf\": [] } ```  A valid UCAN can be used for authorization in this endpoint.
pub async fn ucan_token_post(
    configuration: &configuration::Configuration,
) -> Result<crate::models::UcanTokenPost200Response, Error<UcanTokenPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ucan/token", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UcanTokenPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Store a file with nft.storage. You can upload either a single file or multiple files in a directory. Each request to /upload is restricted to a body size of 100MB, though this does not mean you cannot upload larger files (see \"Size limitations\" below).  ### Single file Send the POST request with `Blob`/`File` data as the request body.  ### Multiple files Send the POST request as `FormData` with the header `Content-Type: multipart/form-data` set. Each part should have a `Content-Disposition` header to specify \"name\" (which must be \"file\") and \"filename\". e.g.  ``` ------WebKitFormBoundary5peilISl2YOOweQy Content-Disposition: form-data; name=\"file\"; filename=\"image.png\" Content-Type: image/png  <data> ------WebKitFormBoundary5peilISl2YOOweQy-- ```  ### Content Addressed Archive (CAR) files You can also upload a CAR file, by setting the request body as a single CAR Blob/File object and providing the request header `Content-Type: application/car` Providing a CAR file allows you to pre-compute the root CID for 1 or more files, ensures that NFT.Storage will store and provide your assets with the same CID.  ### Size limitations Each request to the upload endpoint is limited to a total request body size of 100MB. However, you can upload files larger than 100MB by packing them into a CAR file and splitting the CAR into chunks of less than 100MB. This strategy is used by the JavaScript client library to support uploads of large files.  The simplest method of splitting CAR files is with the [carbites cli tool](https://github.com/nftstorage/carbites):  ``` npm i -g carbites-cli  # Split a big CAR into many smaller CARs carbites split big.car --size 100MB --strategy treewalk ```  Note that you MUST use the `treewalk` strategy, so that all the chunked CARs have the same root CID. Once all the CAR chunks have been uploaded, the CARs will be combined, made available via IPFS, and queued for storage on Filecoin.  For more about working with CARs, see <https://docs.web3.storage/how-tos/work-with-car-files>  ### Rate limits  This API imposes rate limits to ensure quality of service. You may receive a 429 \"Too many requests\" error if you make more than 30 requests with the same API token within a ten second window. Upon receiving a response with a 429 status, clients should retry the failed request after a small delay. To avoid 429 responses, you may wish to implement client-side request throttling to stay within the limits (note: the JS client automatically does this).
pub async fn upload<'a, P>(
    configuration: &configuration::Configuration,
    paths: &'a [P],
    x_agent_did: Option<&str>,
) -> Result<crate::models::UploadResponse, Error<UploadError>>
where
    P: AsRef<Path> + Send + Sync + 'a,
{
    let client = &configuration.client;
    let uri = format!("{}/upload", &configuration.base_path);

    let mut request_builder = client.post(&uri);

    if let Some(user_agent) = &configuration.user_agent {
        request_builder = request_builder.header(header::USER_AGENT, user_agent);
    }
    if let Some(token) = &configuration.bearer_access_token {
        request_builder = request_builder.bearer_auth(token);
    }
    if let Some(agent_did) = x_agent_did {
        request_builder = request_builder.header("x-agent-did", agent_did);
    }

    let mut form = multipart::Form::new();
    for path in paths {
        let file_name = path
            .as_ref()
            .file_name()
            .ok_or(crate::apis::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File name could not be retrieved.",
            )))?
            .to_string_lossy();
        let file = File::open(path.as_ref())
            .await
            .map_err(crate::apis::Error::Io)?;
        form = form.part(
            "file",
            multipart::Part::stream(reqwest::Body::wrap_stream(
                tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new()),
            ))
            .file_name(file_name.to_string()),
        );
    }

    let response = request_builder
        .multipart(form)
        .send()
        .await
        .map_err(crate::apis::Error::Reqwest)?;

    if response.status().is_success() {
        let response_body = response.text().await.map_err(crate::apis::Error::Reqwest)?;
        serde_json::from_str(&response_body).map_err(crate::apis::Error::Serde)
    } else {
        let status = response.status();
        let local_var_content = response.text().await.map_err(crate::apis::Error::Reqwest)?;
        let local_var_entity: Option<UploadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status,
            content: local_var_content.to_string(),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn upload_multiple_bytes(
    configuration: &configuration::Configuration,
    files: Vec<(Vec<u8>, String)>,
    x_agent_did: Option<&str>,
) -> Result<crate::models::UploadResponse, Error<UploadError>> {
    let client = &configuration.client;
    let uri = format!("{}/upload", &configuration.base_path);

    let mut request_builder = client.post(&uri);

    if let Some(user_agent) = &configuration.user_agent {
        request_builder = request_builder.header(header::USER_AGENT, user_agent);
    }
    if let Some(token) = &configuration.bearer_access_token {
        request_builder = request_builder.bearer_auth(token);
    }
    if let Some(agent_did) = x_agent_did {
        request_builder = request_builder.header("x-agent-did", agent_did);
    }

    let mut form = multipart::Form::new();
    for (data, file_name) in files {
        form = form.part("file", multipart::Part::bytes(data).file_name(file_name));
    }
    let response = request_builder
        .multipart(form)
        .send()
        .await
        .map_err(crate::apis::Error::Reqwest)?;

    if response.status().is_success() {
        let response_body = response.text().await.map_err(crate::apis::Error::Reqwest)?;
        serde_json::from_str(&response_body).map_err(crate::apis::Error::Serde)
    } else {
        let status = response.status();
        let local_var_content = response.text().await.map_err(crate::apis::Error::Reqwest)?;
        let local_var_entity: Option<UploadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status,
            content: local_var_content.to_string(),
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn user_did_post(
    configuration: &configuration::Configuration,
    user_did_post_request: crate::models::UserDidPostRequest,
) -> Result<crate::models::DidGet200Response, Error<UserDidPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/did", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&user_did_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserDidPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
