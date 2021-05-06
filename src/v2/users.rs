use crate::v2::models::GraphObjectWithPaths;
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &'static str = concatcp!(super::URL, "users/");

pub async fn list_parent_groups(
    c: &Client,
    id: &str,
) -> Result<Vec<GraphObjectWithPaths>, JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}/{}", URL, &id, "memberof"))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let groups: Vec<GraphObjectWithPaths> =
        serde_json::from_str(&resp.text().await.unwrap()).unwrap();
    Ok(groups)
}
