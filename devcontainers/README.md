# devcontainers

A Rust library for parsing and working with devcontainer.json files.

## Overview

This library provides strongly-typed representations of the [Dev Container specification](https://containers.dev/), allowing you to parse, validate, and work with devcontainer.json files in Rust.

## Features

- **Comprehensive Type Support**: Full coverage of devcontainer.json fields including:
  - Basic configuration (name, image, dockerFile)
  - Build configuration
  - Features and extensions
  - Port forwarding and attributes
  - Environment variables
  - Lifecycle commands (postCreateCommand, postStartCommand, etc.)
  - Customizations for different IDEs
  - Docker Compose integration
  - And more!

- **Feature Flag: `allow-unknown-fields`**: Control how unknown JSON fields are handled:
  - **Default behavior**: Unknown fields cause deserialization errors, ensuring strict validation
  - **With feature enabled**: Unknown fields are captured in an `additional_fields` HashMap, allowing forward compatibility with newer devcontainer.json versions

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
devcontainers = "0.1"
```

### Basic Example

```rust
use devcontainers::DevContainer;

let json = r#"{
    "name": "My Dev Container",
    "image": "mcr.microsoft.com/devcontainers/rust:latest",
    "features": {
        "ghcr.io/devcontainers/features/docker-in-docker:2": {}
    },
    "extensions": [
        "rust-lang.rust-analyzer"
    ]
}"#;

let devcontainer: DevContainer = serde_json::from_str(json)?;
println!("Container name: {:?}", devcontainer.name);
```

### With Build Configuration

```rust
use devcontainers::DevContainer;

let json = r#"{
    "name": "Custom Build",
    "build": {
        "dockerfile": "Dockerfile",
        "context": "..",
        "args": {
            "VARIANT": "bullseye"
        }
    },
    "postCreateCommand": "npm install"
}"#;

let devcontainer: DevContainer = serde_json::from_str(json)?;
```

### Using the `allow-unknown-fields` Feature

Enable the feature in your `Cargo.toml`:

```toml
[dependencies]
devcontainers = { version = "0.1", features = ["allow-unknown-fields"] }
```

Now unknown fields will be captured:

```rust
use devcontainers::DevContainer;

let json = r#"{
    "name": "Test",
    "image": "ubuntu:latest",
    "futureField": "this will be captured",
    "experimentalFeature": true
}"#;

let devcontainer: DevContainer = serde_json::from_str(json)?;
// Access unknown fields
println!("Additional fields: {:?}", devcontainer.additional_fields);
```

## Testing

Run tests without the feature (strict mode):
```bash
cargo test
```

Run tests with the feature enabled:
```bash
cargo test --features allow-unknown-fields
```

## License

MIT OR Apache-2.0
