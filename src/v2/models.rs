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

#[derive(Debug, Deserialize, Serialize)]
pub enum GraphType {
    #[serde(rename = "active_directory")]
    ActiveDirectory,
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "command")]
    Command,
    #[serde(rename = "g_suite")]
    GSuite,
    #[serde(rename = "ldap_server")]
    LdapServer,
    #[serde(rename = "office_365")]
    Office365,
    #[serde(rename = "policy")]
    Policy,
    #[serde(rename = "policy_group")]
    PolicyGroup,
    #[serde(rename = "radius_server")]
    RadiusServer,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "system_group")]
    SystemGroup,
    #[serde(rename = "user")]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandAssociationsPost {
    pub id: String,
    pub op: GraphObjectModificationType,
    pub attributes: Option<String>,
    pub r#type: CommandAssociationType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CommandAssociationType {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "system_group")]
    SystemGroup,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GraphObjectModificationType {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "update")]
    Update,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemGroup {
    pub attributes: Option<String>, // NOT CORRECT
    pub description: Option<String>,
    pub email: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<GraphType>,
}
