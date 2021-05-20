use crate::v2::models::SystemGroup;
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &str = concatcp!(super::URL, "systemgroups/");

pub async fn list(c: &Client, id: &str) -> Result<SystemGroup, JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;

    Ok(resp.json::<SystemGroup>().await?)
}

pub async fn list_all(c: &Client) -> Result<Vec<SystemGroup>, JCError> {
    let resp = c
        .http_client
        .get(URL)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;

    Ok(resp.json::<Vec<SystemGroup>>().await?)
}
