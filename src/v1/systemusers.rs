use crate::v1::models::{SystemUserPost, SystemUserReturn, SystemUsersList};
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &'static str = concatcp!(super::URL, "systemusers/");

pub async fn list_user(c: &Client, id: &str) -> Result<SystemUserReturn, JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match resp.status() {
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(JCError::JumpCloud(crate::ErrorCode::Status401(None)))
        }
        _ => (),
    };

    let raw_json = resp.text().await.unwrap();
    let user: Result<SystemUserReturn, _> = serde_json::from_str(&raw_json);
    let user = match user {
        Ok(u) => u,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };

    Ok(user)
}

pub async fn list_all_users(c: &Client) -> Result<SystemUsersList, JCError> {
    let resp = c
        .http_client
        .get(URL)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let users: SystemUsersList = serde_json::from_str(&resp.text().await.unwrap()).unwrap();
    Ok(users)
}

pub async fn create_system_user(
    c: &Client,
    user: SystemUserPost,
) -> Result<SystemUserReturn, JCError> {
    // Serialize the user first
    let u = serde_json::to_string(&user);
    let u = match u {
        Ok(user) => user,
        Err(e) => {
            return Err(JCError::Other(format!(
                "Failed to serialize user: {}",
                e.to_string()
            )))
        }
    };

    let resp = c
        .http_client
        .post(URL)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(u)
        .send()
        .await?;

    match resp.status() {
        reqwest::StatusCode::BAD_REQUEST => {
            return Err(JCError::JumpCloud(crate::ErrorCode::Status400(Some(
                "Body was malformed".to_string(),
            ))))
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(JCError::JumpCloud(crate::ErrorCode::Status401(None)))
        }
        _ => (),
    }

    let raw_json = resp.text().await.unwrap();
    let user: Result<SystemUserReturn, _> = serde_json::from_str(&raw_json);
    let user = match user {
        Ok(u) => u,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };

    Ok(user)
}

pub async fn delete_system_user(c: &Client, id: &str) -> Result<SystemUserReturn, JCError> {
    let resp = c
        .http_client
        .delete(format!("{}{}", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match resp.status() {
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(JCError::JumpCloud(crate::ErrorCode::Status401(None)))
        }
        _ => (),
    };

    let raw_json = resp.text().await.unwrap();
    let user: Result<SystemUserReturn, _> = serde_json::from_str(&raw_json);
    let user = match user {
        Ok(u) => u,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };

    Ok(user)
}
