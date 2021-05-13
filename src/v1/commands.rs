use crate::v1::models::Command;
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
