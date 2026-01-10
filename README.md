# devil

Devcontainer generator and utilities for working with Dev Container configurations.

## Workspace Structure

This is a Cargo workspace (resolver v3) containing the following crates in the `crates/` directory:

### `devcontainers`

A `no_std` compatible Rust library for parsing and working with devcontainer.json files according to the [Dev Container specification](https://containers.dev/).

Features:
- **`no_std` compatible** - uses `alloc` for dynamic allocations
- **Improved type safety** - structured types for ports and commands
- **Optional feature flags** - `vscode`, `docker-compose`, `allow-unknown-fields`
- Comprehensive type support for all devcontainer.json fields
- Uses `BTreeMap` for deterministic ordering
- Strict validation by default, with optional support for unknown fields
- All structs and enums are `non_exhaustive` for forward compatibility
- `Default` implementations for convenient construction

See the [devcontainers README](crates/devcontainers/README.md) for detailed usage information.

## Building

```bash
cargo build
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with specific features
cargo test --features vscode
cargo test --features docker-compose
cargo test --all-features
```

## License

MPL-2.0

## Goals
- [ ] Standardize DevContainer configurations
- [ ] Maintain `no_std` compatibility for parsing library

## Progress
- [ ] `devcontainers` crate implemented with extensive feature support

---
*AI assisted*
