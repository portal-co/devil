//! # devcontainers
//!
//! A Rust library for parsing and working with devcontainer.json files.
//!
//! This library provides strongly-typed representations of the Dev Container specification,
//! allowing you to parse, validate, and work with devcontainer.json files in Rust.
//!
//! This is a `no_std` compatible crate that uses `alloc` for dynamic allocations.
//!
//! ## Features
//!
//! - `allow-unknown-fields`: When enabled, allows parsing JSON files with unknown fields.
//!   When disabled (default), unknown fields will cause deserialization to fail.
//!
//! ## Example
//!
//! ```
//! use devcontainers::DevContainer;
//!
//! let json = r#"{
//!     "name": "My Dev Container",
//!     "image": "mcr.microsoft.com/devcontainers/rust:latest"
//! }"#;
//!
//! let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
//! assert_eq!(devcontainer.name, Some("My Dev Container".to_string()));
//! ```

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Main devcontainer.json configuration structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
#[cfg_attr(not(feature = "allow-unknown-fields"), serde(deny_unknown_fields))]
pub struct DevContainer {
    /// Display name for the dev container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Docker image to use as the base for the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Path to a Dockerfile in your repo
    #[serde(skip_serializing_if = "Option::is_none", rename = "dockerFile")]
    pub docker_file: Option<String>,

    /// Build configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildConfig>,

    /// Additional features or addons to install
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<BTreeMap<String, serde_json::Value>>,

    /// VS Code extensions to install
    #[cfg(feature = "vscode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,

    /// VS Code settings for the container
    #[cfg(feature = "vscode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<BTreeMap<String, serde_json::Value>>,

    /// Ports to forward from the container
    #[serde(skip_serializing_if = "Option::is_none", rename = "forwardPorts")]
    pub forward_ports: Option<Vec<PortSpec>>,

    /// Port attributes configuration
    #[serde(skip_serializing_if = "Option::is_none", rename = "portsAttributes")]
    pub ports_attributes: Option<BTreeMap<String, PortAttributes>>,

    /// Default port attributes for unspecified ports
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "otherPortsAttributes"
    )]
    pub other_ports_attributes: Option<PortAttributes>,

    /// Environment variables for the container
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerEnv")]
    pub container_env: Option<BTreeMap<String, String>>,

    /// Environment variables for remote processes
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteEnv")]
    pub remote_env: Option<BTreeMap<String, String>>,

    /// User to run as in the container
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteUser")]
    pub remote_user: Option<String>,

    /// User for container processes
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerUser")]
    pub container_user: Option<String>,

    /// Workspace folder path in the container
    #[serde(skip_serializing_if = "Option::is_none", rename = "workspaceFolder")]
    pub workspace_folder: Option<String>,

    /// Command to run after container creation
    #[serde(skip_serializing_if = "Option::is_none", rename = "postCreateCommand")]
    pub post_create_command: Option<LifecycleCommand>,

    /// Command to run after container starts
    #[serde(skip_serializing_if = "Option::is_none", rename = "postStartCommand")]
    pub post_start_command: Option<LifecycleCommand>,

    /// Command to run after attaching to the container
    #[serde(skip_serializing_if = "Option::is_none", rename = "postAttachCommand")]
    pub post_attach_command: Option<LifecycleCommand>,

    /// IDE-specific customizations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations: Option<BTreeMap<String, serde_json::Value>>,

    /// Whether to use init process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,

    /// Whether to run the container in privileged mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    /// Whether to override the default command
    #[serde(skip_serializing_if = "Option::is_none", rename = "overrideCommand")]
    pub override_command: Option<bool>,

    /// Action to take when shutting down
    #[serde(skip_serializing_if = "Option::is_none", rename = "shutdownAction")]
    pub shutdown_action: Option<ShutdownAction>,

    /// Mounts configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MountSpec>>,

    /// Run arguments for the container
    #[serde(skip_serializing_if = "Option::is_none", rename = "runArgs")]
    pub run_args: Option<Vec<String>>,

    /// Docker Compose file reference
    #[cfg(feature = "docker-compose")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "dockerComposeFile")]
    pub docker_compose_file: Option<DockerComposeFile>,

    /// Service name when using Docker Compose
    #[cfg(feature = "docker-compose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// Workspace mount location
    #[serde(skip_serializing_if = "Option::is_none", rename = "workspaceMount")]
    pub workspace_mount: Option<String>,

    /// Additional unknown fields when allow-unknown-fields feature is enabled
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, serde_json::Value>,
}

/// Build configuration for the dev container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
#[cfg_attr(not(feature = "allow-unknown-fields"), serde(deny_unknown_fields))]
pub struct BuildConfig {
    /// Path to Dockerfile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,

    /// Build context path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Build arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<BTreeMap<String, String>>,

    /// Target stage in multi-stage build
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Cache from configuration
    #[serde(skip_serializing_if = "Option::is_none", rename = "cacheFrom")]
    pub cache_from: Option<Vec<String>>,

    /// Additional unknown fields when allow-unknown-fields feature is enabled
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, serde_json::Value>,
}

/// Service port specification
#[derive(Debug, Clone, PartialEq)]
pub struct ServicePort {
    /// Service name
    pub service: String,
    /// Port number
    pub port: u16,
}

impl ServicePort {
    /// Parse a service:port string
    pub fn parse(s: &str) -> Option<Self> {
        let (service, port_str) = s.split_once(':')?;
        
        // Validate service name is not empty
        if service.is_empty() {
            return None;
        }
        
        // Parse port
        let port = port_str.parse::<u16>().ok()?;
        
        Some(ServicePort {
            service: service.to_string(),
            port,
        })
    }
}

impl core::fmt::Display for ServicePort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.service, self.port)
    }
}

impl Serialize for ServicePort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ServicePort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ServicePort::parse(&s)
            .ok_or_else(|| serde::de::Error::custom("Expected format 'service:port'"))
    }
}

/// Port specification (can be a number or service:port)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortSpec {
    /// Numeric port
    Number(u16),
    /// Service name with port specification (e.g., "db:5432")
    Service(ServicePort),
}

/// Port attributes configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
#[cfg_attr(not(feature = "allow-unknown-fields"), serde(deny_unknown_fields))]
pub struct PortAttributes {
    /// Label for the port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Protocol for the port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Action when auto-forwarding
    #[serde(skip_serializing_if = "Option::is_none", rename = "onAutoForward")]
    pub on_auto_forward: Option<String>,

    /// Whether to require local port
    #[serde(skip_serializing_if = "Option::is_none", rename = "requireLocalPort")]
    pub require_local_port: Option<bool>,

    /// Elevate if needed
    #[serde(skip_serializing_if = "Option::is_none", rename = "elevateIfNeeded")]
    pub elevate_if_needed: Option<bool>,

    /// Additional unknown fields when allow-unknown-fields feature is enabled
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, serde_json::Value>,
}

/// Basic command specification (shell or args)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandSpec {
    /// Shell command string
    Shell(String),
    /// Array of command arguments
    Args(Vec<String>),
}

/// Lifecycle command specification (can be a command or object of commands)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LifecycleCommand {
    /// Single command
    Command(CommandSpec),
    /// Object with multiple named commands
    Object(BTreeMap<String, CommandSpec>),
}

/// Shutdown action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShutdownAction {
    /// No action
    None,
    /// Stop the container
    StopContainer,
    /// Stop Docker Compose
    StopCompose,
}

/// Mount specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
#[cfg_attr(not(feature = "allow-unknown-fields"), serde(deny_unknown_fields))]
pub struct MountSpec {
    /// Source path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Target path in container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Mount type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub mount_type: Option<String>,

    /// Additional unknown fields when allow-unknown-fields feature is enabled
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, serde_json::Value>,
}

/// Docker Compose file specification
#[cfg(feature = "docker-compose")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DockerComposeFile {
    /// Single file path
    String(String),
    /// Array of file paths
    Array(Vec<String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    #[cfg(feature = "vscode")]
    use alloc::vec;

    #[test]
    fn test_basic_devcontainer() {
        let json = r#"{
            "name": "Test Container",
            "image": "mcr.microsoft.com/devcontainers/rust:latest"
        }"#;

        let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
        assert_eq!(devcontainer.name, Some("Test Container".to_string()));
        assert_eq!(
            devcontainer.image,
            Some("mcr.microsoft.com/devcontainers/rust:latest".to_string())
        );
    }

    #[test]
    fn test_with_build_config() {
        let json = r#"{
            "name": "Custom Build",
            "build": {
                "dockerfile": "Dockerfile",
                "context": "..",
                "args": {
                    "VARIANT": "bullseye"
                }
            }
        }"#;

        let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
        assert!(devcontainer.build.is_some());
        let build = devcontainer.build.unwrap();
        assert_eq!(build.dockerfile, Some("Dockerfile".to_string()));
        assert_eq!(build.context, Some("..".to_string()));
    }

    #[test]
    #[cfg(feature = "vscode")]
    fn test_with_features_and_extensions() {
        let json = r#"{
            "name": "Feature Test",
            "image": "ubuntu:latest",
            "features": {
                "ghcr.io/devcontainers/features/docker-in-docker:2": {}
            },
            "extensions": [
                "dbaeumer.vscode-eslint",
                "esbenp.prettier-vscode"
            ]
        }"#;

        let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
        assert!(devcontainer.features.is_some());
        assert!(devcontainer.extensions.is_some());
        assert_eq!(devcontainer.extensions.unwrap().len(), 2);
    }

    #[test]
    #[cfg(not(feature = "vscode"))]
    fn test_with_features_no_vscode() {
        let json = r#"{
            "name": "Feature Test",
            "image": "ubuntu:latest",
            "features": {
                "ghcr.io/devcontainers/features/docker-in-docker:2": {}
            }
        }"#;

        let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
        assert!(devcontainer.features.is_some());
    }

    #[test]
    fn test_port_forwarding() {
        let json = r#"{
            "name": "Port Test",
            "image": "node:18",
            "forwardPorts": [3000, "db:5432"],
            "portsAttributes": {
                "3000": {
                    "label": "Application",
                    "onAutoForward": "notify"
                }
            }
        }"#;

        let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
        assert!(devcontainer.forward_ports.is_some());
        assert!(devcontainer.ports_attributes.is_some());
    }

    #[test]
    fn test_lifecycle_commands() {
        let json = r#"{
            "name": "Lifecycle Test",
            "image": "node:18",
            "postCreateCommand": "npm install",
            "postStartCommand": ["npm", "run", "dev"]
        }"#;

        let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
        assert!(devcontainer.post_create_command.is_some());
        assert!(devcontainer.post_start_command.is_some());
    }

    #[test]
    fn test_environment_variables() {
        let json = r#"{
            "name": "Env Test",
            "image": "python:3.9",
            "containerEnv": {
                "MY_VAR": "value",
                "ANOTHER_VAR": "another_value"
            },
            "remoteUser": "vscode"
        }"#;

        let devcontainer: DevContainer = serde_json::from_str(json).unwrap();
        assert!(devcontainer.container_env.is_some());
        assert_eq!(devcontainer.remote_user, Some("vscode".to_string()));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut devcontainer = DevContainer::default();
        devcontainer.name = Some("Test".to_string());
        devcontainer.image = Some("ubuntu:latest".to_string());
        #[cfg(feature = "vscode")]
        {
            devcontainer.extensions = Some(vec!["test.extension".to_string()]);
        }

        let json = serde_json::to_string(&devcontainer).unwrap();
        let parsed: DevContainer = serde_json::from_str(&json).unwrap();
        assert_eq!(devcontainer, parsed);
    }

    #[cfg(not(feature = "allow-unknown-fields"))]
    #[test]
    fn test_unknown_fields_rejected_by_default() {
        let json = r#"{
            "name": "Test",
            "image": "ubuntu:latest",
            "unknownField": "should fail"
        }"#;

        let result: Result<DevContainer, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[cfg(feature = "allow-unknown-fields")]
    #[test]
    fn test_unknown_fields_captured_with_feature() {
        let json = r#"{
            "name": "Test",
            "image": "ubuntu:latest",
            "unknownField": "should be captured",
            "anotherUnknown": 42
        }"#;

        let result: DevContainer = serde_json::from_str(json).unwrap();
        assert_eq!(result.name, Some("Test".to_string()));
        assert_eq!(result.additional_fields.len(), 2);
        assert!(result.additional_fields.contains_key("unknownField"));
        assert!(result.additional_fields.contains_key("anotherUnknown"));
    }

    #[test]
    fn test_default_implementations() {
        // Test Default for DevContainer
        let devcontainer = DevContainer::default();
        assert_eq!(devcontainer.name, None);
        assert_eq!(devcontainer.image, None);

        // Test Default for BuildConfig
        let build_config = BuildConfig::default();
        assert_eq!(build_config.dockerfile, None);
        assert_eq!(build_config.context, None);

        // Test Default for PortAttributes
        let port_attrs = PortAttributes::default();
        assert_eq!(port_attrs.label, None);

        // Test Default for MountSpec
        let mount = MountSpec::default();
        assert_eq!(mount.source, None);
    }

    #[test]
    fn test_default_with_modifications() {
        // Demonstrate using Default as a builder pattern
        let mut devcontainer = DevContainer::default();
        devcontainer.name = Some("Test Container".to_string());
        devcontainer.image = Some("ubuntu:latest".to_string());

        let json = serde_json::to_string(&devcontainer).unwrap();
        let parsed: DevContainer = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, Some("Test Container".to_string()));
        assert_eq!(parsed.image, Some("ubuntu:latest".to_string()));
    }
}
