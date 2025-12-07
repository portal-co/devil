//! Example: Parse a devcontainer.json file
//!
//! Run with: cargo run --example parse_devcontainer

use devcontainers::DevContainer;

fn main() {
    // Example 1: Parse a basic devcontainer
    let basic_json = r#"{
        "name": "Rust Development",
        "image": "mcr.microsoft.com/devcontainers/rust:latest",
        "features": {
            "ghcr.io/devcontainers/features/docker-in-docker:2": {}
        },
        "extensions": [
            "rust-lang.rust-analyzer"
        ],
        "postCreateCommand": "cargo --version",
        "remoteUser": "vscode"
    }"#;

    println!("=== Parsing Basic Devcontainer ===");
    match serde_json::from_str::<DevContainer>(basic_json) {
        Ok(container) => {
            println!("✓ Successfully parsed!");
            println!("  Name: {:?}", container.name);
            println!("  Image: {:?}", container.image);
            println!("  Remote User: {:?}", container.remote_user);
            #[cfg(feature = "vscode")]
            if let Some(extensions) = &container.extensions {
                println!("  Extensions: {} installed", extensions.len());
            }
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
        }
    }

    println!();

    // Example 2: Create using Default and serialize
    println!("=== Creating with Default ===");
    let mut container = DevContainer::default();
    container.name = Some("My Custom Container".to_string());
    container.image = Some("ubuntu:22.04".to_string());
    container.remote_user = Some("developer".to_string());

    match serde_json::to_string_pretty(&container) {
        Ok(json) => {
            println!("✓ Created and serialized:");
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("✗ Serialization error: {}", e);
        }
    }

    println!();

    // Example 3: Complex configuration
    println!("=== Parsing Complex Configuration ===");
    let complex_json = r#"{
        "name": "Full Stack Dev",
        "build": {
            "dockerfile": "Dockerfile",
            "context": "..",
            "args": {
                "VARIANT": "bullseye"
            }
        },
        "forwardPorts": [3000, 5432],
        "portsAttributes": {
            "3000": {
                "label": "Frontend",
                "onAutoForward": "notify"
            }
        },
        "containerEnv": {
            "DATABASE_URL": "postgres://localhost:5432",
            "NODE_ENV": "development"
        },
        "postCreateCommand": "npm install && cargo build"
    }"#;

    match serde_json::from_str::<DevContainer>(complex_json) {
        Ok(container) => {
            println!("✓ Successfully parsed complex config!");
            println!("  Name: {:?}", container.name);
            if let Some(build) = &container.build {
                println!("  Build dockerfile: {:?}", build.dockerfile);
            }
            if let Some(ports) = &container.forward_ports {
                println!("  Forwarding {} ports", ports.len());
            }
            if let Some(env) = &container.container_env {
                println!("  Environment variables: {}", env.len());
            }
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
        }
    }
}
