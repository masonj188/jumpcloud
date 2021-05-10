use crate::v1::models::{SystemUserPost, SystemUserReturn, SystemUsersList};
use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &str = concatcp!(super::URL, "systemusers/");

pub async fn list_user(c: &Client, id: &str) -> Result<SystemUserReturn, JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    resp.error_for_status_ref()?;
    let user = resp.json::<SystemUserReturn>().await;
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

    resp.error_for_status_ref()?;
    let users = resp.json::<SystemUsersList>().await;
    let users = match users {
        Ok(u) => u,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };
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

    resp.error_for_status_ref()?;

    let user = resp.json::<SystemUserReturn>().await;
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

    resp.error_for_status_ref()?;

    let user = resp.json::<SystemUserReturn>().await;
    let user = match user {
        Ok(u) => u,
        Err(e) => return Err(JCError::Other(e.to_string())),
    };

    Ok(user)
}
