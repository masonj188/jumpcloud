use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SystemUsersList {
    pub results: Option<Vec<SystemUserReturn>>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<i64>,
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
    pub bad_login_attempts: Option<i64>,
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
    pub mfa: Option<Mfa>,
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
    pub mfa: Option<Mfa>,
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
pub struct Mfa {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub id: Option<String>,
    pub active: Option<bool>,
    #[serde(rename = "agentVersion")]
    pub agent_version: Option<String>,
    #[serde(rename = "allowMultiFactorAuthentication")]
    pub allow_multi_factor_authentication: Option<bool>,
    #[serde(rename = "allowPublicKeyAuthentication")]
    pub allow_public_key_authentication: Option<bool>,
    #[serde(rename = "allowSshPasswordAuthentication")]
    pub allow_ssh_password_authentication: Option<bool>,
    #[serde(rename = "allowSshRootLogin")]
    pub allow_ssh_root_login: Option<bool>,
    #[serde(rename = "amazonInstanceID")]
    pub amazon_instance_i_d: Option<String>,
    pub arch: Option<String>,
    #[serde(rename = "connectionHistory")]
    pub connection_history: Option<Vec<String>>, // Not correct but the object isn't defined in the api docs
    pub created: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub fde: Option<Fde>,
    #[serde(rename = "fileSystem")]
    pub file_system: Option<String>,
    #[serde(rename = "hasServiceAccount")]
    pub has_service_account: Option<bool>,
    pub hostname: Option<String>,
    #[serde(rename = "lastContact")]
    pub last_contact: Option<String>,
    #[serde(rename = "modifySSHDConfig")]
    pub modify_sshd_config: Option<bool>,
    #[serde(rename = "networkInterfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    pub organization: Option<String>,
    pub os: Option<String>,
    #[serde(rename = "provisionMetadata")]
    pub provision_metadata: Option<Provisioner>,
    #[serde(rename = "remoteIP")]
    pub remote_ip: Option<String>,
    #[serde(rename = "sshRootEnabled")]
    pub ssh_root_enabled: Option<bool>,
    #[serde(rename = "sshdParams")]
    pub sshd_params: Option<Vec<SshdParam>>,
    #[serde(rename = "systemInsights")]
    pub system_insights: Option<SystemInsights>,
    #[serde(rename = "systemTimezone")]
    pub system_timezone: Option<i64>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "templateName")]
    pub template_name: Option<String>,
    pub version: Option<String>,
    pub mdm: Option<Mdm>,
    #[serde(rename = "builtInCommands")]
    pub built_in_commands: Option<Vec<BuiltInCommand>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemsList {
    pub results: Option<Vec<System>>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuiltInCommand {
    pub r#type: Option<BuiltInCommandType>,
    pub name: Option<BuiltInCommandName>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BuiltInCommandType {
    Security,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BuiltInCommandName {
    Erase,
    Lock,
    Restart,
    Shutdown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mdm {
    pub vendor: Option<Vendor>,
    pub internal: Option<MdmInternal>,
    #[serde(rename = "profileIdentifier")]
    pub profile_identifier: Option<String>,
    pub deb: Option<bool>,
    #[serde(rename = "userApproved")]
    pub user_approved: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Vendor {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "none")]
    r#None,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MdmInternal {
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemInsights {
    pub state: Option<SystemInsightState>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SystemInsightState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "deferred")]
    Deferred,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SshdParam {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provisioner {
    pub r#type: Option<ProvisionerType>,
    #[serde(rename = "provisionerId")]
    pub provisioner_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProvisionerType {
    Administrator,
    Mdm,
    User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkInterface {
    pub address: Option<String>,
    pub family: Option<String>,
    pub internal: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fde {
    pub active: Option<bool>,
    #[serde(rename = "keyPresent")]
    pub key_present: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CommandsList {
    pub results: Option<Vec<CommandsListCommand>>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CommandsListCommand {
    pub id: Option<String>,
    pub command: Option<String>,
    #[serde(rename = "commandType")]
    pub command_type: Option<String>,
    #[serde(rename = "launchType")]
    pub launch_type: Option<String>,
    #[serde(rename = "listensTo")]
    pub listens_to: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub schedule: Option<String>,
    #[serde(rename = "scheduleRepeatType")]
    pub schedule_repeat_type: Option<String>,
    pub trigger: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Command {
    #[serde(skip)]
    // For some reason the API doesn't include the ID, unlike every other model
    pub id: Option<String>,
    pub command: String,
    #[serde(rename = "commandRunners")]
    pub command_runners: Option<Vec<String>>,
    #[serde(rename = "commandType")]
    pub command_type: Option<String>,
    pub files: Option<Vec<String>>,
    #[serde(rename = "launchType")]
    pub launch_type: Option<String>,
    #[serde(rename = "listensTo")]
    pub listens_to: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub schedule: Option<String>,
    #[serde(rename = "scheduleRepeatType")]
    pub schedule_repeat_type: Option<String>,
    pub sudo: Option<bool>,
    pub systems: Option<Vec<String>>,
    pub timeout: Option<String>,
    pub trigger: Option<String>,
    pub user: Option<String>,
    pub shell: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandFileReturn {
    pub results: Option<Vec<CommandFile>>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandFile {
    pub id: Option<String>,
    pub destination: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResultList {
    pub results: Option<Vec<CommandResult>>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub id: Option<String>,
    pub command: Option<String>,
    pub files: Option<Vec<String>>,
    pub name: Option<String>,
    pub organization: Option<String>,
    #[serde(rename = "requestTime")]
    pub request_time: Option<String>,
    pub response: Option<CommandResultResponse>,
    #[serde(rename = "responseTime")]
    pub response_time: Option<String>,
    pub sudo: Option<bool>,
    pub system: Option<String>,
    #[serde(rename = "systemId")]
    pub system_id: Option<String>,
    pub user: Option<String>,
    #[serde(rename = "workflowId")]
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowInstanceId")]
    pub workflow_instance_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResultResponse {
    pub data: Option<CommandResultResponseData>,
    pub error: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResultResponseData {
    #[serde(rename = "exitCode")]
    pub exit_code: Option<u32>,
    pub output: Option<String>,
}
