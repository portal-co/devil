# devil

Devcontainer generator and utilities for working with Dev Container configurations.

## Workspace Structure

This is a Cargo workspace (resolver v3) containing the following crates in the `crates/` directory:

### `devcontainers`

A `no_std` compatible Rust library for parsing and working with devcontainer.json files according to the [Dev Container specification](https://containers.dev/).

Features:
- **`no_std` compatible** - uses `alloc` for dynamic allocations
- Comprehensive type support for all devcontainer.json fields
- Uses `BTreeMap` for deterministic ordering
- Strict validation by default, with optional support for unknown fields
- All structs are `non_exhaustive` for forward compatibility
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

# Run tests with allow-unknown-fields feature
cargo test --features allow-unknown-fields
```

## License

MPL-2.0
