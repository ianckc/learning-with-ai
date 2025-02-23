# Rust OpenSSL

When running the `cargo` commands for [Apache Kafka](https://github.com/ianckc/learning-with-ai/tree/main/copilot/apache-kafka/simple/rust-kafka-app) I was getting errors around `openssl`

Warp was able to fix this.

## Prompt

```
can you help me install Rust and make sure it works with openssl@3
```

It then suggested these steps

```
brew list openssl@3
brew reinstall libssh2
```

Rust also needed updating to the latest version and Warp ran these commands:

```
brew link --overwrite libssh2
brew uninstall rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To test this it then create a Rust app with

`cargo.toml`

```toml
[package]
name = "openssl-test"
version = "0.1.0"
edition = "2024"

[dependencies]
openssl = "0.10"
```

`src/main.rs`

```rust
use openssl::version;

fn main() {
    let version = version::version();
    println!("OpenSSL version: {}", version);
}
```
