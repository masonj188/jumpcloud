use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserGroupsMemberPost {
    #[serde(rename = "id")]
    pub user_id: String,
    pub op: String,
    // Only valid value is "user"
    pub r#type: String,
}

impl UserGroupsMemberPost {
    pub fn new(user_id: &str) -> Self {
        UserGroupsMemberPost {
            user_id: user_id.to_string(),
            op: String::from("add"),
            r#type: String::from("user"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GraphObjectWithPaths {
    #[serde(rename = "compiledAttributes")]
    pub compiled_attributes: Option<Option<CompiledAttributes>>,
    pub id: String,
    //paths: HashMap<String, String>,
    pub r#type: GraphType,
}

#[derive(Debug, Deserialize)]
pub enum GraphType {
    #[serde(rename = "active_directory")]
    ActiveDirectory,
    Application,
    Command,
    #[serde(rename = "g_suite")]
    GSuite,
    #[serde(rename = "ldap_server")]
    LdapServer,
    #[serde(rename = "office_365")]
    Office365,
    Policy,
    #[serde(rename = "policy_group")]
    PolicyGroup,
    #[serde(rename = "radius_server")]
    RadiusServer,
    System,
    #[serde(rename = "system_group")]
    SystemGroup,
    User,
    #[serde(rename = "user_group")]
    UserGroup,
}

#[derive(Debug, Deserialize)]
pub struct CompiledAttributes {
    #[serde(rename = "ldapGroups")]
    pub ldap_groups: Vec<AttributeName>,
}

#[derive(Debug, Deserialize)]
pub struct AttributeName {
    pub name: String,
}
