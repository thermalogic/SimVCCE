# SimVCC

Vapor compression refrigeration cycle simulator in Rust.

## Dependencies

- **Thermodynamic Properties**: CoolProp via  [`coolprop-sys`](https://crates.io/crates/coolprop-sys)  crate
- **Serialization**: serde + serde_json

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simvcc = "0.1.0"
```

### Quick Start
 
 
- Example VCC：[demovcc.json](./jsonmodel/demovcc.json)  

```rust
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

## Component Implementations

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




