# SimVCC

Vapor compression refrigeration cycle simulator in Rust.

## Prerequisite: CoolProp

This crate calls CoolProp via FFI for thermodynamic property calculations. You must obtain the CoolProp shared library yourself:

1. Go to <http://www.coolprop.org/> and navigate to the **Binaries** section.
2. Download the shared library for your platform:
   - **Windows**: `CoolProp.dll` + `CoolProp.lib`
   - **Linux**: `libCoolProp.so`
   - **macOS**: `libCoolProp.dylib`
3. Create a `sharedlib/` directory next to this crate's `Cargo.toml` and place the files there:
   ```
   your-project/
   ├── sharedlib/
   │   ├── CoolProp.dll    # Windows
   │   └── CoolProp.lib    # Windows
   ├── src/
   ├── Cargo.toml
   └── build.rs
   ```
4. The build script (`build.rs`) will automatically:
   - Add `sharedlib/` to the linker search path
   - Copy the DLL to the output directory (Windows)

> **Note**: Without the CoolProp library in place, linking will fail at build time.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simvcc = "0.1.0"
```

### Quick Start

```rust,no_run
use simvcc::JSONLoader;

let loader = JSONLoader::new();
let json = loader.load_file("jsonmodel/demovcc.json").unwrap();
let mut cycle = loader.create_cycle(&json).unwrap();
cycle.simulator();
cycle.outresults();
```

### CLI

```bash
cargo run -- jsonmodel/demovcc.json
```

## Key Design Principles

1. **Node Sharing** — Connected component ports share the same memory, ensuring state consistency across the cycle.
2. **Component Calculation Order Detection** — No fixed order required; the algorithm automatically discovers the correct processing sequence.

## Components

| Component | Process | Energy |
|---|---|---|
| Compressor | Isentropic compression (constant s) | CompressionWork |
| Condenser | Isobaric condensation (constant p) | QOUT |
| Evaporator | Isobaric evaporation (constant p) | QIN |
| Expansion Valve | Isenthalpic throttling (constant h) | — |

## Cycle Performance Indicators

- COP = Qin / Wc
- COP_hp = Qout / Wc
- Capacity(ton) = Qin × 60 × (1/211)

## Tech Stack

- **Language**: Rust (2021 Edition)
- **Thermodynamic Properties**: CoolProp via FFI
- **Serialization**: serde + serde_json

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
