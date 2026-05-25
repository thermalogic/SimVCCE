# vccrust - Rust version of the vapor compression refrigeration cycle simulator

This is a Rust port of the C++ vapor compression refrigeration cycle simulator.

## Project Structure

```
vccrust/
├── src/
│   ├── common/          # Common types and utilities
│   │   └── mod.rs
│   ├── core/            # Core modules (Port, Connector)
│   │   ├── mod.rs
│   │   ├── port.rs
│   │   └── connector.rs
│   ├── components/      # Component modules
│   │   ├── mod.rs
│   │   ├── compressor.rs
│   │   ├── condenser.rs
│   │   ├── evaporator.rs
│   │   └── expansionvalve.rs
│   ├── utils/           # Utilities (JSON loader)
│   │   ├── mod.rs
│   │   └── json_loader.rs
│   ├── vcc.rs           # Main VCCycle module
│   ├── lib.rs           # Library entry point
│   └── main.rs          # Application entry point
├── jsonmodel/           # JSON model files
│   └── demovcc.json
├── Cargo.toml           # Cargo manifest
└── README.md
```

## Dependencies

- `serde` - Serialization framework
- `serde_json` - JSON serialization

## Building and Running

```bash
# Build the project
cargo build

# Run the example
cargo run -- jsonmodel/demovcc.json
```

## Notes

- This is a simplified version that uses approximate thermodynamic calculations instead of CoolProp bindings for simplicity.
- For production use, consider integrating proper CoolProp bindings.

## License

Same as the original C++ project.
