use crate::v2::models::UserGroupsMemberPost;
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &'static str = concatcp!(super::URL, "usergroups/");

pub async fn add_user_to_group(
    c: &Client,
    group_id: &str,
    group: &UserGroupsMemberPost,
) -> Result<(), JCError> {
    let req_body = serde_json::to_string(group)?;
    let resp = c
        .http_client
        .post(format!("{}{}/{}", URL, group_id, "members"))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(req_body)
        .send()
        .await?;

    match resp.status() {
        reqwest::StatusCode::BAD_REQUEST => {
            return Err(JCError::JumpCloud(crate::ErrorCode::Status400(Some(
                resp.text().await.unwrap(),
            ))))
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(JCError::JumpCloud(crate::ErrorCode::Status401(None)))
        }
        reqwest::StatusCode::NOT_FOUND => {
            return Err(JCError::JumpCloud(crate::ErrorCode::Status404(None)))
        }
        _ => (),
    };

    Ok(())
}
