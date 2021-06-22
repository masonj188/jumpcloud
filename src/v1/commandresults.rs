use crate::v1::models::{CommandResult, CommandResultList};
use crate::{Client, JCError};
use const_format::concatcp;

const URL: &str = concatcp!(super::URL, "commandresults/");

pub async fn list_all(c: &Client) -> Result<CommandResultList, JCError> {
    let resp = c
        .http_client
        .get(URL)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;

    Ok(resp.json::<CommandResultList>().await?)
}

pub async fn list(c: &Client, id: &str) -> Result<CommandResult, JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;

    Ok(resp.json::<CommandResult>().await?)
}
