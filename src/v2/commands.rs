use crate::v2::models::{CommandAssociationType, CommandAssociationsPost, GraphObjectWithPaths};
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &str = concatcp!(super::URL, "commands/");

pub async fn update_command_associations(
    c: &Client,
    id: &str,
    associations: CommandAssociationsPost,
) -> Result<(), JCError> {
    println!("{}", serde_json::to_string(&associations)?);

    let resp = c
        .http_client
        .post(format!("{}{}/associations", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&associations)?)
        .send()
        .await?;

    resp.error_for_status_ref()?;

    Ok(())
}

pub async fn list_command_associations(
    c: &Client,
    id: &str,
    assoc_type: CommandAssociationType,
) -> Result<Vec<GraphObjectWithPaths>, JCError> {
    let new_url = match assoc_type {
        CommandAssociationType::System => format!("{}{}/systems", URL, id),
        CommandAssociationType::SystemGroup => format!("{}{}/systemgroups", URL, id),
    };

    let resp = c
        .http_client
        .get(new_url)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;

    Ok(resp.json::<Vec<GraphObjectWithPaths>>().await?)
}
