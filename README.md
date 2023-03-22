# About
Secretize crate is a solution to store secrets, passwords, or any kind of sensitive data in a very secure manner.\
Under the hood it's using Argon2 hashing algorithm, which is a memory-hard key derivation function. Chosen as the winner
of the [Password Hashing Competition](https://www.password-hashing.net) in July 2015.
# Usage
A `Secret` can be initialized via `Secret::new()` or `Secret::wrap()`. The difference between the two is meanwhile
calling `Secret::new()` is simply initializing the Secret with the given value, calling `Secret::wrap()` moves the
ownership of the given secret, and zeroize it after hashing the secret. So, there is no chance to recover the "wrapped"
secret's value.

To check the secret against any bytes-like value use the `.verify()` method.
```rust
use secretize::Secret;

fn main() {
    let secret = "my-secret".to_string();
    let my_secret = Secret::wrap(secret).expect("failed to hash secret");
    assert!(my_secret.verify("my-secret"));
}
```
## Features
### `base64`
Turning on this feature will serialize the hash as a base64 encoded string. If it turned off, the hash serialized as a
PHC string.
### `serde`
Turn on serde support. This feature can be combined with `base64`, then the secret value will be serialized as a base64
encoded string, otherwise as a PHC string.
```rust
#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: Secret,
}
```
### `openapi`
Turn on poem_openapi support. This feature can be combined with `base64`, then the secret value will be serialized as a
base64 encoded string in the OpenAPI JSON specification, otherwise as a PHC string.
```rust
#[derive(Debug, Serialize, Deserialize, Object)]
struct User {
    username: String,
    password: Secret,
}
```
### `eq`
Turning on this feature will allow equality between `Secter` and any bytes-like value. Since, it's not trivial and
straightforward what equals to what, this feature was put behind a feature flag.
```rust
use secretize::Secret;

fn main() {
    let secret = "my-secret".to_string();
    let my_secret = Secret::wrap(secret).expect("failed to hash secret");
    assert_eq!(my_secret, "my-secret");
}
```
## Minimum Supported Rust Version
Rust 1.65 or higher.
