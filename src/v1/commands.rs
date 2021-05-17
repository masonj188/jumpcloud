use crate::v1::models::{Command, CommandsList};
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &str = concatcp!(super::URL, "commands/");

pub async fn create_command(c: &Client, command: Command) -> Result<Command, JCError> {
    let resp = c
        .http_client
        .post(URL)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&command)?)
        .send()
        .await?;

    resp.error_for_status_ref()?;
    let command_response = resp.json::<Command>().await;
    let command_response = match command_response {
        Ok(s) => s,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };

    Ok(command_response)
}

pub async fn list_all_commands(c: &Client) -> Result<CommandsList, JCError> {
    let resp = c
        .http_client
        .get(URL)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;

    Ok(resp.json::<CommandsList>().await?)
}

pub async fn list_command(c: &Client, id: &str) -> Result<Command, JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;

    let mut cmd = resp.json::<Command>().await?;
    cmd.id = Some(id.to_string());
    Ok(cmd)
}
