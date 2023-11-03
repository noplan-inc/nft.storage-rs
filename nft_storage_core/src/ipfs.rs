use crate::{error::CoreError, Result};
use rand::seq::SliceRandom as _;
use reqwest::Client;

static GATEWAYS: &[&str] = &[
    "https://ipfs.io/ipfs",
    "https://gateway.ipfs.io/ipfs",
    "https://cloudflare-ipfs.com/ipfs",
    "https://dweb.link/ipfs",
];

async fn download_from_gateway(cid: String, gateway: String) -> Result<Vec<u8>> {
    let url = format!("{}/{}", gateway, cid);
    let client = Client::new();
    let resp = client.get(&url).send().await.map_err(|e| {
        CoreError::IpfsError(format!(
            "Error downloading from gateway: {} cid: {}, url: {}, e: {}",
            gateway, cid, url, e
        ))
    })?;

    let status = resp.status();

    if !status.is_success() {
        return Err(CoreError::IpfsError(format!(
            "Error downloading from gateway: {} cid: {}, url: {}, status: {}",
            gateway, cid, url, status
        )));
    }
    let bytes = resp.bytes().await.map_err(|e| {
        CoreError::IpfsError(format!(
            "Error downloading bytes() from gateway: {} cid: {}, url: {}, e: {}",
            gateway, cid, url, e
        ))
    })?;

    Ok(bytes.to_vec())
}

pub async fn download_from_gateways(cid: String, name: String) -> Result<(Vec<u8>, String)> {
    // random gateway
    let gateway = GATEWAYS
        .choose(&mut rand::thread_rng())
        .ok_or("No gateways")
        .map_err(|e| {
            CoreError::IpfsError(format!(
                "Error choosing random gateway, cid: {}, e: {}",
                cid, e
            ))
        })?;

    // try to download from random gateway
    let resp = download_from_gateway(cid, gateway.to_string()).await?;

    Ok((resp, name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_from_gateways() {
        let name = "rust1.png.enc".to_string();

        let cid = format!(
            "bafybeidikdidxlvdblrvqcyamusqjzktsy7wbn7ygl4qaqolkrg2nwqlk4/{}",
            name
        );
        let (resp, name) = download_from_gateways(cid, name).await.unwrap();
        assert_eq!(resp.len(), 1905120);
        assert_eq!(name, "rust1.png.enc".to_string());

        let name2 = "rust2.png.enc".to_string();
        let cid2 = format!(
            "bafybeidikdidxlvdblrvqcyamusqjzktsy7wbn7ygl4qaqolkrg2nwqlk4/{}",
            name2
        );
        let (resp2, name2) = download_from_gateways(cid2, name2).await.unwrap();
        assert_eq!(resp2.len(), 1390240);
        assert_eq!(name2, "rust2.png.enc".to_string());
    }
}
