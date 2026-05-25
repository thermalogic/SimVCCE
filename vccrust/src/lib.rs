//! simvcc — Rust implementation of the vapor compression refrigeration cycle simulator.
//!
//! This crate simulates a vapor compression refrigeration cycle (VCC) using
//! CoolProp for thermodynamic property calculations via FFI.
//!
//! # Prerequisite: CoolProp
//!
//! This crate requires the **CoolProp** shared library to be present at build
//! and runtime. You must obtain it yourself:
//!
//! 1. Download the CoolProp shared library for your platform from
//!    <http://www.coolprop.org/> (see the "Binaries" section).
//! 2. Place `CoolProp.dll` (Windows), `libCoolProp.so` (Linux), or
//!    `libCoolProp.dylib` (macOS) — along with the corresponding import
//!    library if applicable — into a `sharedlib/` directory next to this
//!    crate's `Cargo.toml`.
//! 3. The build script (`build.rs`) will automatically add `sharedlib/` to
//!    the linker search path and copy the DLL to the output directory on
//!    Windows.
//!
//! Without the CoolProp library in place, linking will fail at build time.
//!
//! # Key Design Principles
//! 1. **Node Sharing** — Connected component ports share the same memory,
//!    ensuring state consistency across the cycle
//! 2. **Component Calculation Order Detection** — No fixed order required;
//!    the algorithm automatically discovers the correct processing sequence
//!
//! # Quick Start
//! ```no_run
//! use simvcc::JSONLoader;
//!
//! let loader = JSONLoader::new();
//! let json = loader.load_file("jsonmodel/demovcc.json").unwrap();
//! let mut cycle = loader.create_cycle(&json).unwrap();
//! cycle.simulator();
//! cycle.outresults();
//! ```

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
