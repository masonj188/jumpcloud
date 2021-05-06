use reqwest;
use std::{env, error::Error, fmt::Display, path::PathBuf};

pub mod v1;
pub mod v2;

// TODO: Need to manually implement Debug
#[derive(Debug)]
pub enum JCError {
    JumpCloud(ErrorCode),
    Reqwest(reqwest::Error),
    Serde(serde_json::error::Error),
    Other(String),
}

impl Error for JCError {
    fn description(&self) -> &str {
        match self {
            JCError::JumpCloud(_) => "Error interacting with JumpCloud API",
            JCError::Reqwest(_) => "Underlying reqwest client error",
            JCError::Serde(_) => "Serde serialization error",
            JCError::Other(_) => "Undefined error",
        }
    }
}

impl Display for JCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JCError::JumpCloud(e) => match e {
                ErrorCode::Status400(o) => match o {
                    Some(s) => write!(f, "400 - bad request: {}", s),
                    None => write!(f, "400 - bad request"),
                },
                ErrorCode::Status401(o) => match o {
                    Some(s) => write!(f, "401 - request not authorized: {}", s),
                    None => write!(f, "400 - bad request"),
                },
                ErrorCode::Status403(o) => match o {
                    Some(s) => write!(f, "403 - client forbidden from accessing resource: {}", s),
                    None => write!(f, "403 - client forbidden from accessing resource"),
                },
                ErrorCode::Status404(o) => match o {
                    Some(s) => write!(f, "404 - not found: {}", s),
                    None => write!(f, "404 - not found"),
                },
                ErrorCode::Status409(o) => match o {
                    Some(s) => write!(f, "409 - conflict: {}", s),
                    None => write!(f, "409 - conflict"),
                },
                ErrorCode::Status500(o) => match o {
                    Some(s) => write!(f, "500 - Internal server error: {}", s),
                    None => write!(f, "500 - Internal server error"),
                },
            },

            JCError::Reqwest(e) => write!(f, "{}", e),
            JCError::Serde(e) => write!(f, "{}", e),
            JCError::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    Status400(Option<String>),
    Status401(Option<String>),
    Status403(Option<String>),
    Status404(Option<String>),
    Status409(Option<String>),
    Status500(Option<String>),
}

impl From<reqwest::Error> for JCError {
    fn from(re: reqwest::Error) -> Self {
        JCError::Reqwest(re)
    }
}

impl From<serde_json::error::Error> for JCError {
    fn from(se: serde_json::error::Error) -> Self {
        JCError::Serde(se)
    }
}

#[derive(Debug)]
pub struct Client {
    api_key: String,
    http_client: reqwest::Client,
}

impl Client {
    /**
    Creates a new Jumpcloud API Client. Takes an optional [`PathBuf`](std::path::PathBuf) which
    lets you specify an alternate path where your Jumpcloud credential
    file lives.

    Checks for API credentials with the following precedence:
    - Filepath passed to new (if [`Some<PathBuf>`] is given as an argument)
    - An environment variable `JUMPCLOUD_API_KEY` if set
    - Credential file `.jumpcloud` in the user's home directory

    If a path is given but no credential was able to be loaded from that path,
    [`None`] will be returned in order to prevent an unexpected credential file from
    being loaded.

    Returns [`None`] if no API Key was found.

    # Examples

    ```no_run
    // Load the api key from a credential file called '.jumpcloud'
    // within the current directory
    let client = jumpcloud::Client::new(Some(std::path::PathBuf::from("./.jumpcloud")));
    assert_ne!(client, None);
    ```


    ```
    // Load the api key from an envrionment variable
    std::env::set_var(
        "JUMPCLOUD_API_KEY",
        "ffffffffffffffffffffffffffffffffffffffff",
    );
    let client = jumpcloud::Client::new(None);
    assert_ne!(client, None);
    ```


    ```no_run
    // Load the api key from a credential file called '.jumpcloud'
    // found in the caller's home directory.
    let client = jumpcloud::Client::new(None);
    assert_ne!(client, None)
    ```
    */
    pub fn new(fp: Option<PathBuf>) -> Option<Self> {
        if let Some(fp) = fp {
            match load_key_from_file(fp) {
                Some(k) => {
                    return Some(Self {
                        api_key: k,
                        http_client: reqwest::Client::new(),
                    })
                }
                None => return None,
            };
        }

        if let Ok(key) = env::var("JUMPCLOUD_API_KEY") {
            return Some(Self {
                api_key: key,
                http_client: reqwest::Client::new(),
            });
        };

        let mut fpath = match home::home_dir() {
            Some(p) => p,
            None => return None,
        };
        fpath.push(".jumpcloud");
        let key = load_key_from_file(fpath);
        if let Some(key) = key {
            return Some(Self {
                api_key: key,
                http_client: reqwest::Client::new(),
            });
        }

        None
    }
}

fn load_key_from_file(fp: PathBuf) -> Option<String> {
    let toml_string = std::fs::read_to_string(fp);
    let toml_string = match toml_string {
        Ok(s) => s,
        Err(_) => return None,
    };
    let toml_string = toml_string.parse::<toml::Value>();
    let toml_string = match toml_string {
        Ok(v) => v,
        Err(_) => return None,
    };

    match toml_string["api-key"].as_str() {
        Some(k) => Some(k.to_string()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn give_path() {
        match Client::new(Some(PathBuf::from("./.jumpcloud"))) {
            Some(_) => (),
            None => panic!(),
        };
    }

    #[test]
    fn load_env() {
        env::set_var(
            "JUMPCLOUD_API_KEY",
            "ffffffffffffffffffffffffffffffffffffffff",
        );
        match Client::new(None) {
            Some(_) => (),
            None => panic!(),
        }
    }

    #[test]
    fn load_home() {
        match Client::new(None) {
            Some(_) => (),
            None => panic!(),
        }
    }

    #[test]
    fn list_user() {
        let client = Client::new(None).unwrap();
        tokio_test::block_on(v1::systemusers::list_user(
            &client,
            "608c931d678ad52c3a629d0c",
        ))
        .unwrap();
    }

    #[test]
    fn list_all_users() {
        let client = Client::new(None).unwrap();
        tokio_test::block_on(v1::systemusers::list_all_users(&client)).unwrap();
    }
}
