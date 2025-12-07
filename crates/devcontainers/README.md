# devcontainers

A Rust library for parsing and working with devcontainer.json files.

## Overview

This library provides strongly-typed representations of the [Dev Container specification](https://containers.dev/), allowing you to parse, validate, and work with devcontainer.json files in Rust.

## Features

- **`no_std` Compatible**: This crate is fully `no_std` compatible, using `alloc` for dynamic allocations. Perfect for embedded systems and constrained environments.

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

- **BTreeMap for Sorted Keys**: Uses `BTreeMap` instead of `HashMap` for deterministic ordering and `no_std` compatibility

- **Non-exhaustive Structs**: All structs are marked with `#[non_exhaustive]`, allowing the library to add new fields in the future without breaking backward compatibility

- **Default Trait**: All structs implement `Default`, making it easy to create instances and use a builder-like pattern

- **Optional Feature Flags**: Fine-grained control over what's included:
  - **`allow-unknown-fields`**: Capture unknown JSON fields in an `additional_fields` BTreeMap for forward compatibility
  - **`vscode`**: Enable VS Code-specific fields (extensions, settings)
  - **`docker-compose`**: Enable Docker Compose support (dockerComposeFile, service)

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

### Using Default for Builder Pattern

```rust
use devcontainers::DevContainer;

let mut devcontainer = DevContainer::default();
devcontainer.name = Some("My Container".to_string());
devcontainer.image = Some("rust:latest".to_string());
devcontainer.remote_user = Some("vscode".to_string());

let json = serde_json::to_string_pretty(&devcontainer)?;
println!("{}", json);
```

### Using Optional Features

Enable specific features in your `Cargo.toml` based on your needs:

```toml
[dependencies]
# For VS Code extension and settings support
devcontainers = { version = "0.1", features = ["vscode"] }

# For Docker Compose support
devcontainers = { version = "0.1", features = ["docker-compose"] }

# For all features
devcontainers = { version = "0.1", features = ["vscode", "docker-compose", "allow-unknown-fields"] }
```

#### `vscode` Feature

Enables VS Code-specific fields:

```rust
use devcontainers::DevContainer;

let json = r#"{
    "name": "VS Code Container",
    "image": "ubuntu:latest",
    "extensions": ["rust-lang.rust-analyzer"],
    "settings": {
        "terminal.integrated.shell.linux": "/bin/bash"
    }
}"#;

let devcontainer: DevContainer = serde_json::from_str(json)?;
```

#### `docker-compose` Feature

Enables Docker Compose integration:

```rust
use devcontainers::DevContainer;

let json = r#"{
    "name": "Compose Container",
    "dockerComposeFile": "docker-compose.yml",
    "service": "app"
}"#;

let devcontainer: DevContainer = serde_json::from_str(json)?;
```

#### `allow-unknown-fields` Feature

Captures unknown fields for forward compatibility:

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

Run tests without features:
```bash
cargo test
```

Run tests with specific features:
```bash
cargo test --features vscode
cargo test --features docker-compose
cargo test --all-features
```

## License

MPL-2.0
