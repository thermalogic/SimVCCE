//! SimVCC — Vapor compression refrigeration cycle simulator in Rust.
//!
//! # Dependencies
//!
//! - **Thermodynamic Properties**: CoolProp via [`coolprop-sys`](https://crates.io/crates/coolprop-sys) crate
//! - **Serialization**: serde + serde_json
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! simvcc = "0.1.4"
//! ```
//!
//! ## Quick Start
//!
//! Example VCC: [demovcc.json](./jsonmodel/demovcc.json)
//!
//! ```no_run
//! use simvcc::JSONLoader;
//!
//! let loader = JSONLoader::new();
//! let json = loader.load_file("jsonmodel/demovcc.json").unwrap();
//! let mut cycle = loader.create_cycle(&json).unwrap();
//! cycle.simulator();
//! cycle.out_results();
//! ```
//!
//! ## CLI
//!
//! ```bash
//! cargo run -- jsonmodel/demovcc.json
//! ```
//!
//! # Key Design Principles
//!
//! 1. **Node Sharing** — Connected component ports share the same memory via `Rc<RefCell<Port>>`, ensuring state consistency across the cycle.
//! 2. **Component Calculation Order Detection** — No fixed order required; the algorithm automatically discovers the correct processing sequence using iterative `Result`-based error handling.
//! 3. **SISOComponent Abstraction** — Common SISO component fields and logic are encapsulated in a shared [`SISOComponent`] struct, reducing code duplication.
//!
//! # Component Implementations
//!
//! | Component | Process | Energy |
//! |---|---|---|
//! | Compressor | Isentropic compression (constant s) | CompressionWork |
//! | Condenser | Isobaric condensation (constant p) | QOUT |
//! | Evaporator | Isobaric evaporation (constant p) | QIN |
//! | Expansion Valve | Isenthalpic throttling (constant h) | — |
//!
//! # Cycle Performance Indicators
//!
//! - COP = Qin / Wc
//! - COP_hp = Qout / Wc
//! - Capacity(ton) = Qin × 60 × (1/211)
//!
//! # Example VCC JSON
//!
//! * [demovcc.json](./jsonmodel/demovcc.json)
//!
//! ```json
//! {
//!     "name": "Demo Vapor-Compression Cycle",
//!     "refrigerant": "R134a",
//!     "components": [
//!         {
//!             "name": "Compressor",
//!             "classstr": "Compressor",
//!             "iPort": { "t": 0.0,   "x": 1.0,   "mdot": 0.08     },
//!             "oPort": { "p": 0.6854     }
//!         },
//!         {
//!             "name": "Condenser",
//!             "classstr": "Condenser",
//!             "iPort": {},
//!             "oPort": {   "t": 26.0,  "x": 0.0  }
//!         },
//!         {
//!             "name": "ExpansionValve",
//!             "classstr": "ExpansionValve",
//!             "iPort": {},
//!             "oPort": {}
//!         },
//!         {
//!             "name": "Evaporator",
//!             "classstr": "Evaporator",
//!             "iPort": {},
//!             "oPort": {}
//!         }
//!     ],
//!     "connectors": {
//!         "Compressor.oPort": "Condenser.iPort",
//!         "Condenser.oPort": "ExpansionValve.iPort",
//!         "ExpansionValve.oPort": "Evaporator.iPort",
//!         "Evaporator.oPort": "Compressor.iPort"
//!     }
//! }
//! ```
//! 

pub mod common;
pub mod core;
pub mod components;
pub mod utils;
pub mod vcc;

pub use common::*;
pub use core::*;
pub use components::*;
pub use utils::*;
pub use vcc::VCCycle;
