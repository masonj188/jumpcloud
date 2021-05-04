use crate::Client;
use crate::JCError;
use const_format::concatcp;

const URL: &'static str = concatcp!(super::URL, "systemusers/");

pub async fn list_user(c: &Client, id: &str) -> Result<(), JCError> {
    let resp = c
        .http_client
        .get(format!("{}{}", URL, id))
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    println!("{:?}", resp.text().await);
    Ok(())
}

pub async fn list_all_users(c: &Client) -> Result<(), JCError> {
    let resp = c
        .http_client
        .get(URL)
        .header("x-api-key", &c.api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?;

    println!("{}", resp.text().await.unwrap());
    Ok(())
}
