use reqwest;
use std::{env, path::PathBuf};

pub mod v1;
pub mod v2;

// TODO: Need to manually implement Debug
#[derive(Debug)]
pub enum JCError {
    JumpCloud(ErrorCode),
    Reqwest(reqwest::Error),
    Other(String),
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
