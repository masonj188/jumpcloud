use crate::v1::models::{System, SystemsList};
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &str = concatcp!(super::URL, "systems/");

pub async fn list_system(c: &Client, id: &str) -> Result<System, JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}", URL, id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;
    let system = resp.json::<System>().await;
    let system = match system {
        Ok(s) => s,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };

    Ok(system)
}

pub async fn list_all_systems(c: &Client) -> Result<SystemsList, JCError> {
    let resp = c
        .http_client
        .get(URL)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;
    let systems = resp.json::<SystemsList>().await;
    let systems = match systems {
        Ok(u) => u,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };

    Ok(systems)
}

pub async fn reboot_system(c: &Client, id: &str) -> Result<(), JCError> {
    let resp = c
        .http_client
        .post(format!("{}{}/command/builtin/restart", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;
    resp.error_for_status_ref()?;
    Ok(())
}
