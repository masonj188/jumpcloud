use crate::Client;
use crate::JCError;
use const_format::concatcp;
use serde::Deserialize;

const URL: &'static str = concatcp!(super::URL, "systemusers/");

#[derive(Debug, Deserialize)]
pub struct SystemUsersList {
  sdtnyfg results: Option<Vec<SystemUserReturn>>,
    #[serde(rename = "totalCount")]
    total_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SystemUserReturn {
    id: Option<String>,
    account_locked: Option<bool>,
    account_locked_date: Option<String>,
    activated: Option<bool>,
    addresses: Option<Vec<Address>>,
    allow_public_key: Option<bool>,
    attributes: Option<Vec<Attribute>>,
    #[serde(rename = "badLoginAttempts")]
    bad_login_attempts: Option<u32>,
    company: Option<String>,
    #[serde(rename = "costCenter")]
    cost_center: Option<String>,
    created: Option<String>,
    department: Option<String>,
    description: Option<String>,
    #[serde(rename = "disableDeviceMaxLoginAttempts")]
    disable_device_max_login_attempts: Option<bool>,
    displayname: Option<String>,
    email: Option<String>,
    #[serde(rename = "employeeIdentifier")]
    employee_identifier: Option<String>,
    #[serde(rename = "employeeType")]
    employee_type: Option<String>,
    enable_managed_uid: Option<bool>,
    enable_user_portal_multifactor: Option<bool>,
    external_dn: Option<String>,
    external_password_expiration_date: Option<String>,
    external_source_type: Option<String>,
    externally_managed: Option<bool>,
    firstname: Option<String>,
    #[serde(rename = "jobTitle")]
    job_title: Option<String>,
    lastname: Option<String>,
    ldap_binding_user: Option<bool>,
    location: Option<String>,
    mfa: Option<MFA>,
    middlename: Option<String>,
    organization: Option<String>,
    password_expiration_date: Option<String>,
    password_expired: Option<bool>,
    password_never_expires: Option<bool>,
    passwordless_sudo: Option<bool>,
    #[serde(rename = "phoneNumbers")]
    phone_numbers: Option<Vec<PhoneNumber>>,
    public_key: Option<String>,
    relationships: Option<Vec<String>>, // Not sure what to do here, the object isn't defined in their schema
    samba_service_user: Option<bool>,
    ssh_keys: Option<Vec<SSHKey>>,
    state: Option<String>,
    sudo: Option<bool>,
    suspended: Option<bool>,
    tags: Option<Vec<String>>,
    totp_enabled: Option<bool>,
    unix_guid: Option<u32>,
    unix_uid: Option<u32>,
    username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    country: Option<String>,
    #[serde(rename = "extendedAddress")]
    extended_address: Option<String>,
    id: Option<String>,
    locality: Option<String>,
    #[serde(rename = "poBox")]
    po_box: Option<String>,
    #[serde(rename = "postalCode")]
    postal_code: Option<String>,
    region: Option<String>,
    #[serde(rename = "streetAddress")]
    street_address: Option<String>,
    r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Attribute {
    name: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MFA {
    configured: Option<bool>,
    exclusion: Option<bool>,
    #[serde(rename = "exclusionUntil")]
    exclusion_until: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PhoneNumber {
    id: Option<String>,
    number: Option<String>,
    r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SSHKey {
    id: Option<String>,
    create_date: Option<String>,
    name: Option<String>,
    public_key: Option<String>,
}

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
    let user: SystemUserReturn = serde_json::from_str(&raw_json).unwrap();
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
)
}
