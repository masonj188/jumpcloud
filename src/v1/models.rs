use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct SystemUsersList {
    pub results: Option<Vec<SystemUserReturn>>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SystemUserReturn {
    pub id: Option<String>,
    pub account_locked: Option<bool>,
    pub account_locked_date: Option<String>,
    pub activated: Option<bool>,
    pub addresses: Option<Vec<Address>>,
    pub allow_public_key: Option<bool>,
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "badLoginAttempts")]
    pub bad_login_attempts: Option<u32>,
    pub company: Option<String>,
    #[serde(rename = "costCenter")]
    pub cost_center: Option<String>,
    pub created: Option<String>,
    pub department: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "disableDeviceMaxLoginAttempts")]
    pub disable_device_max_login_attempts: Option<bool>,
    pub displayname: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "employeeIdentifier")]
    pub employee_identifier: Option<String>,
    #[serde(rename = "employeeType")]
    pub employee_type: Option<String>,
    pub enable_managed_uid: Option<bool>,
    pub enable_user_portal_multifactor: Option<bool>,
    pub external_dn: Option<String>,
    pub external_password_expiration_date: Option<String>,
    pub external_source_type: Option<String>,
    pub externally_managed: Option<bool>,
    pub firstname: Option<String>,
    #[serde(rename = "jobTitle")]
    pub job_title: Option<String>,
    pub lastname: Option<String>,
    pub ldap_binding_user: Option<bool>,
    pub location: Option<String>,
    pub mfa: Option<MFA>,
    pub middlename: Option<String>,
    pub organization: Option<String>,
    pub password_expiration_date: Option<String>,
    pub password_expired: Option<bool>,
    pub password_never_expires: Option<bool>,
    pub passwordless_sudo: Option<bool>,
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub public_key: Option<String>,
    pub relationships: Option<Vec<String>>, // Not sure what to do here, the object isn't defined in their schema
    pub samba_service_user: Option<bool>,
    pub ssh_keys: Option<Vec<SSHKey>>,
    pub state: Option<String>,
    pub sudo: Option<bool>,
    pub suspended: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub totp_enabled: Option<bool>,
    pub unix_guid: Option<u32>,
    pub unix_uid: Option<u32>,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct SystemUserPost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_public_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "costCenter")]
    pub cost_center: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disableDeviceMaxLoginAttempts")]
    pub disable_device_max_login_attempts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayname: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "employeeIdentifier")]
    pub employee_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "employeeType")]
    pub employee_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_managed_uid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_user_portal_multifactor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_password_expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jobTitle")]
    pub job_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_binding_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<MFA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middlename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_never_expires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwordless_sudo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<String>>, // Not sure what to do here, the object isn't defined in their schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samba_service_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sudo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_guid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_uid: Option<u32>,
    pub username: String,
}

impl Default for SystemUserPost {
    fn default() -> SystemUserPost {
        SystemUserPost {
            account_locked: None,
            activated: None,
            addresses: None,
            allow_public_key: None,
            attributes: None,
            company: None,
            cost_center: None,
            department: None,
            description: None,
            disable_device_max_login_attempts: None,
            displayname: None,
            email: "".to_string(),
            employee_identifier: None,
            employee_type: None,
            enable_managed_uid: None,
            enable_user_portal_multifactor: None,
            external_dn: None,
            external_password_expiration_date: None,
            external_source_type: None,
            externally_managed: None,
            firstname: None,
            job_title: None,
            lastname: None,
            ldap_binding_user: None,
            location: None,
            mfa: None,
            middlename: None,
            password: None,
            password_never_expires: None,
            passwordless_sudo: None,
            phone_numbers: None,
            public_key: None,
            relationships: None, // Not sure what to do here, the object isn't defined in their schema
            samba_service_user: None,
            sudo: None,
            suspended: None,
            tags: None,
            unix_guid: None,
            unix_uid: None,
            username: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub country: Option<String>,
    #[serde(rename = "extendedAddress")]
    pub extended_address: Option<String>,
    pub id: Option<String>,
    pub locality: Option<String>,
    #[serde(rename = "poBox")]
    pub po_box: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "streetAddress")]
    pub street_address: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attribute {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MFA {
    pub configured: Option<bool>,
    pub exclusion: Option<bool>,
    #[serde(rename = "exclusionUntil")]
    pub exclusion_until: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhoneNumber {
    pub id: Option<String>,
    pub number: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SSHKey {
    pub id: Option<String>,
    pub create_date: Option<String>,
    pub name: Option<String>,
    pub public_key: Option<String>,
}
