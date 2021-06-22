# Rust Jumpcloud API

A WIP Rust Jumpcloud API tailored for a use-case within the [CEDAR](https://uwyo.edu/cedar) lab.

## Use

### Creating a Client

`jumpcloud::Client::new()` takes an optional [`PathBuf`](std::path::PathBuf) which
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

### Examples

```Rust
// Load the api key from a credential file called '.jumpcloud'
// within the current directory
let client = jumpcloud::Client::new(Some(std::path::PathBuf::from("./.jumpcloud")));
assert_ne!(client, None);
```


```Rust
// Load the api key from an envrionment variable
std::env::set_var(
	"JUMPCLOUD_API_KEY",
	"ffffffffffffffffffffffffffffffffffffffff",
);
let client = jumpcloud::Client::new(None);
assert_ne!(client, None);
```


```Rust
// Load the api key from a credential file called '.jumpcloud'
// found in the caller's home directory.
let client = jumpcloud::Client::new(None);
assert_ne!(client, None)
`
