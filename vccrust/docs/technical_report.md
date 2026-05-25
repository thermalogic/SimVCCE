# SimVCC — Technical Report

## Vapor Compression Refrigeration Cycle Simulator in Rust

**Version**: 0.1.0  
**Author**: Cheng Maohua <cmh@seu.edu.cn>  
**License**: MIT  
**Repository**: https://github.com/thermalogic/SimVCCE

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Thermodynamic Background](#2-thermodynamic-background)
3. [Architecture Overview](#3-architecture-overview)
4. [Core Data Structures](#4-core-data-structures)
5. [Component Models](#5-component-models)
6. [Node Sharing and the Connector](#6-node-sharing-and-the-connector)
7. [Component Calculation Order Detection Algorithm](#7-component-calculation-order-detection-algorithm)
8. [CoolProp Integration](#8-coolprop-integration)
9. [JSON Configuration Format](#9-json-configuration-format)
10. [Build System and Deployment](#10-build-system-and-deployment)
11. [Simulation Workflow](#11-simulation-workflow)
12. [Cycle Performance Indicators](#12-cycle-performance-indicators)
13. [Demo Example](#13-demo-example)
14. [Design Decisions and Trade-offs](#14-design-decisions-and-trade-offs)
15. [Future Directions](#15-future-directions)

---

## 1. Introduction

SimVCC is a vapor compression refrigeration cycle (VCC) simulator written in Rust. It models the four fundamental components of a standard VCC system — compressor, condenser, expansion valve, and evaporator — and computes cycle-level performance indicators such as the coefficient of performance (COP), refrigeration capacity, and compression work.

The simulator leverages [CoolProp](http://www.coolprop.org/) for high-accuracy thermodynamic property calculations via Rust's Foreign Function Interface (FFI). Cycle configurations are specified in JSON, making the system flexible and easy to configure without code changes.

### Key Features

- **Automatic calculation order detection**: No need to specify the order in which components should be processed. The algorithm discovers the correct sequence dynamically.
- **Node sharing**: Connected component ports share the same memory, ensuring state consistency and automatic propagation of thermodynamic properties across the cycle.
- **CoolProp integration**: Real refrigerant properties via FFI, supporting any fluid available in CoolProp (e.g., R134a, R410A, R290).
- **JSON-driven configuration**: Cycle topologies and initial conditions are defined in JSON files, enabling rapid prototyping of different cycle configurations.

---

## 2. Thermodynamic Background

### 2.1 The Vapor Compression Refrigeration Cycle

The standard vapor compression cycle consists of four thermodynamic processes:

| Process | Component | Description |
|---|---|---|
| 1 → 2 | Compressor | Isentropic compression (s = const) |
| 2 → 3 | Condenser | Isobaric heat rejection (p = const) |
| 3 → 4 | Expansion Valve | Isenthalpic throttling (h = const) |
| 4 → 1 | Evaporator | Isobaric heat absorption (p = const) |

### 2.2 State Properties

At each connection point (port/node) in the cycle, the thermodynamic state is described by:

- **P** — Pressure (MPa)
- **T** — Temperature (°C)
- **H** — Specific enthalpy (kJ/kg)
- **S** — Specific entropy (kJ/kg·K)
- **X** — Vapor quality (mass fraction of vapor, 0–1; NaN for supercritical/subcooled)
- **mdot** — Mass flow rate (kg/s)

Two independent properties are sufficient to determine the complete state. The simulator uses CoolProp's `PropsSI` function to compute all remaining properties from any valid pair.

### 2.3 Governing Equations

**Compressor (Isentropic Compression)**:
- s_out = s_in
- W_c = mdot × (h_out - h_in)

**Condenser (Isobaric Condensation)**:
- p_out = p_in
- Q_out = mdot × (h_in - h_out)

**Expansion Valve (Isenthalpic Throttling)**:
- h_out = h_in
- No work or heat transfer

**Evaporator (Isobaric Evaporation)**:
- p_out = p_in
- Q_in = mdot × (h_out - h_in)

---

## 3. Architecture Overview

### 3.1 Module Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Crate root and public API
├── vcc.rs               # VCCycle simulator and algorithm
├── common/
│   └── mod.rs           # Port, CompSISO trait, FFI bindings, utilities
├── components/
│   ├── mod.rs           # Component module re-exports
│   ├── compressor.rs    # Compressor model
│   ├── condenser.rs     # Condenser model
│   ├── evaporator.rs    # Evaporator model
│   └── expansionvalve.rs # Expansion valve model
├── core/
│   ├── mod.rs           # Core module re-exports
│   ├── connector.rs     # Node sharing connector
│   └── port.rs          # Port re-export
└── utils/
    ├── mod.rs           # Utils module re-exports
    └── json_loader.rs   # JSON configuration parser
```

### 3.2 Dependency Graph

```
main.rs
  └── JSONLoader (utils/json_loader.rs)
        └── VCCycle (vcc.rs)
              ├── Connector (core/connector.rs)
              │     └── Port (common/mod.rs)
              │           └── CoolProp FFI
              └── Components (components/*.rs)
                    └── CompSISO trait (common/mod.rs)
                          └── Port (common/mod.rs)
```

### 3.3 Key Abstractions

| Abstraction | Role |
|---|---|
| `Port` | Thermodynamic state at a connection point |
| `CompSISO` | Trait for Single-Input Single-Output components |
| `Connector` | Manages node sharing between component ports |
| `VCCycle` | Orchestrates simulation and aggregates results |
| `JSONLoader` | Parses JSON configuration into VCCycle instances |

---

## 4. Core Data Structures

### 4.1 Port

The `Port` struct is the fundamental data structure representing the thermodynamic state of the working fluid at a connection point:

```rust
pub struct Port {
    pub name: String,
    pub fluid_name: String,  // e.g., "R134a"
    pub p: f64,              // Pressure (MPa)
    pub t: f64,              // Temperature (°C)
    pub h: f64,              // Enthalpy (kJ/kg)
    pub s: f64,              // Entropy (kJ/kg·K)
    pub x: f64,              // Quality
    pub mdot: f64,           // Mass flow rate (kg/s)
    pub stateok: bool,       // Whether state is fully determined
    pub index: usize,        // Node index in connector's list
}
```

**State Calculation Methods**:

| Method | Input Pair | Properties Computed |
|---|---|---|
| `tx()` | T, x | P, H, S |
| `px()` | P, x | T, H, S |
| `ps()` | P, S | H, T, x |
| `ph()` | P, H | S, T, x |
| `pt()` | P, T | S, H, x |

The `state()` method attempts to resolve an unresolved port by trying `ps()`, `ph()`, and `pt()` in order, using whichever pair of properties is available.

### 4.2 CompSISO Trait

The `CompSISO` (Single-Input Single-Output) trait defines the interface for all cycle components:

```rust
pub trait CompSISO: PortDict + PortDictMut + AsAny {
    fn setportaddress(&mut self);
    fn state(&mut self);
    fn balance(&mut self);
    fn resultstring(&self) -> String;
    fn name(&self) -> &str;
    fn energy(&self) -> &str;
}
```

**Panic Convention**: The `state()` and `balance()` methods are expected to **panic** when required input data is not yet available (i.e., when critical port properties are NaN). This is not an error condition — it is a deliberate signaling mechanism that tells the `component_simulator` to defer processing of this component until upstream data becomes available.

### 4.3 Supporting Traits

- **`PortDict`** / **`PortDictMut`**: Provide read/write access to a component's port dictionary (`HashMap<String, *mut Port>`).
- **`AsAny`**: Enables runtime type downcasting from `Box<dyn CompSISO>` to concrete component types, used during result aggregation.

---

## 5. Component Models

### 5.1 Compressor

**Thermodynamic process**: Isentropic compression (s_in = s_out)

The compressor model assumes an ideal isentropic compression process:

1. **`state()`**: Sets `oPort.s = iPort.s`. Panics if `iPort.s` is NaN.
2. **`balance()`**: 
   - Propagates mass flow rate: `mdot` is copied from whichever port has a valid value.
   - Calculates compression work: `Wc = mdot × (h_out - h_in)`.
   - Panics if both ports' `mdot` are NaN, or if either port's `h` is NaN.

**Energy category**: `"CompressionWork"` — contributes to cycle-level Wc.

### 5.2 Condenser

**Thermodynamic process**: Isobaric condensation (p_in = p_out)

The condenser model assumes an ideal isobaric heat rejection process:

1. **`state()`**: Propagates pressure between ports. If one port has a valid pressure and the other doesn't, copies it. Panics if both ports' `p` are NaN.
2. **`balance()`**:
   - Propagates mass flow rate.
   - Calculates heat transfer rate: `Qout = mdot × (h_in - h_out)`.
   - Panics if both ports' `mdot` are NaN, or if either port's `h` is NaN.

**Energy category**: `"QOUT"` — contributes to cycle-level Qout.

### 5.3 Evaporator

**Thermodynamic process**: Isobaric evaporation (p_in = p_out)

The evaporator model assumes an ideal isobaric heat absorption process:

1. **`state()`**: Propagates pressure between ports (same logic as condenser). Panics if both ports' `p` are NaN.
2. **`balance()`**:
   - Propagates mass flow rate.
   - Calculates refrigeration capacity: `Qin = mdot × (h_out - h_in)`.
   - Panics if both ports' `mdot` are NaN, or if either port's `h` is NaN.

**Energy category**: `"QIN"` — contributes to cycle-level Qin.

### 5.4 Expansion Valve

**Thermodynamic process**: Isenthalpic throttling (h_in = h_out)

The expansion valve model assumes an ideal isenthalpic throttling process:

1. **`state()`**: Propagates enthalpy between ports. If one port has a valid enthalpy and the other doesn't, copies it. Panics if both ports' `h` are NaN.
2. **`balance()`**: Propagates mass flow rate only. No energy calculation. Panics if both ports' `mdot` are NaN.

**Energy category**: `""` (empty) — no contribution to cycle-level energy indicators.

### 5.5 Component Summary

| Component | `state()` | `balance()` | Energy |
|---|---|---|---|
| Compressor | oPort.s = iPort.s | mdot propagation; Wc = mdot×(h_out−h_in) | CompressionWork |
| Condenser | p propagation | mdot propagation; Qout = mdot×(h_in−h_out) | QOUT |
| Evaporator | p propagation | mdot propagation; Qin = mdot×(h_out−h_in) | QIN |
| Expansion Valve | h propagation | mdot propagation only | — |

---

## 6. Node Sharing and the Connector

### 6.1 Concept

Node sharing is the central mechanism that ensures state consistency across the cycle. When two components are connected (e.g., the compressor's output port connects to the condenser's input port), their ports share the **same memory location**. This means any state change written by one component is immediately visible to the connected component.

### 6.2 The Connector

The `Connector` struct manages the creation and lifecycle of shared nodes:

```rust
pub struct Connector {
    pub index: usize,
    pub nodes: Vec<*mut Port>,
}
```

### 6.3 Connection Process

When `add_connector()` is called with a specification like `("Compressor", "oPort") → ("Condenser", "iPort")`:

1. **Get port pointers**: Retrieve raw pointers to both ports from their respective components.
2. **Set node index**: Assign the current node index to port0.
3. **Add port0 as node**: Port0's pointer becomes the shared node in the `nodes` vector.
4. **Merge port1 values**: Any known property values from port1 that are NaN in the node are copied over.
5. **Replace port1 pointer**: Port1's entry in its component's `portdict` is replaced with the node pointer.
6. **Update port addresses**: The component's `i_port`/`o_port` fields are updated to point to the shared node.

After this process, both the compressor's `oPort` and the condenser's `iPort` point to the **same `Port` object in memory**. Any modification by one component is instantly visible to the other.

### 6.4 Memory Ownership

The `Connector` owns all shared node memory. Ports are initially created via `Box::into_raw(Box::new(Port::new(...)))` in component constructors, transferring ownership to raw pointers. When a connector replaces a port pointer with a shared node, the original port's memory is effectively abandoned (the pointer is overwritten). The `Drop` implementation for `Connector` reclaims all node memory via `Box::from_raw`.

### 6.5 Node Sharing Diagram

```
Before connection:
  Compressor.oPort ──→ [Port A]    Condenser.iPort ──→ [Port B]

After connection:
  Compressor.oPort ──→ [Port A*] ←── Condenser.iPort
                         (shared node)
  * Port A now contains merged values from both Port A and Port B
```

---

## 7. Component Calculation Order Detection Algorithm

### 7.1 Motivation

In a cycle, components depend on each other's outputs. For example, the compressor needs the evaporator's output state, which in turn depends on the expansion valve's output, which depends on the condenser's output, which depends on the compressor's output. This circular dependency means there is no obvious "first" component to process.

Rather than requiring the user to specify a processing order or implementing a topological sort, SimVCC uses an **iterative panic-driven algorithm** that automatically discovers the correct calculation order.

### 7.2 Algorithm Description

The `component_simulator` method implements the following algorithm:

```
1. Initialize: keys = list of all component names
2. While keys is not empty and iteration count ≤ number of components:
   a. For each component in keys:
      i.   Call state()    — thermal process calculation
      ii.  Update unresolved nodes — propagate state through shared nodes
      iii. Call balance()   — energy and mass balance
      iv.  If all steps succeed (no panic): remove component from keys
      v.   If any step panics: skip component (remains in keys)
   b. Increment iteration counter
3. If keys is not empty after max iterations: report unresolved components
```

### 7.3 How It Works

The algorithm relies on two key mechanisms:

1. **Panic as signal**: When a component's `state()` or `balance()` method encounters NaN input data, it panics. This panic is caught by `catch_unwind` and is treated as "not ready yet" rather than an error.

2. **Node sharing propagation**: When a component successfully processes, it writes results to its output port. Because of node sharing, these results are immediately visible to the connected downstream component's input port. On the next iteration, the downstream component may now have sufficient data to process successfully.

### 7.4 Convergence

For a well-defined cycle with sufficient initial conditions, the algorithm typically converges in at most N iterations (where N is the number of components). In practice, convergence often occurs much faster — typically 2–3 iterations for a standard four-component cycle.

### 7.5 Unresolved Node Resolution

After each component's `state()` call, the algorithm iterates through all unresolved nodes (those with `stateok = false`) and attempts to calculate their complete state using `Port::state()`. Successfully resolved nodes are removed from the unresolved list. This ensures that partial information (e.g., a pressure value set by one component) is fully resolved before the next component attempts to use it.

### 7.6 Panic Hook Suppression

In `main.rs`, the panic hook is set to suppress output:

```rust
std::panic::set_hook(Box::new(|_| {}));
```

This prevents the intentional panics from `state()` and `balance()` from producing error output during `catch_unwind` calls. Without this, each iteration would produce spurious panic messages for components that are simply "not ready yet."

---

## 8. CoolProp Integration

### 8.1 The `coolprop-sys` Crate

SimVCC interfaces with CoolProp through the `coolprop-sys` crate, which provides raw FFI bindings to the CoolProp C library. The crate bundles the native CoolProp dynamic libraries for all supported platforms (Windows x86-64, Windows AArch64, Linux x86-64, macOS x86-64, macOS AArch64) and handles linking and DLL deployment automatically — no manual setup is required.

The `PropsSI` function is accessed through the global `COOLPROP` static provided by the crate. Since `COOLPROP` is wrapped in a `LazyLock<Mutex<CoolProp>>`, it must be locked before use:

```rust
let coolprop = coolprop_sys::COOLPROP.lock().unwrap();
unsafe {
    (coolprop.PropsSI)(
        c_output.as_ptr(),
        c_name1.as_ptr(),
        prop1,
        c_name2.as_ptr(),
        prop2,
        c_fluid.as_ptr(),
    )
}
```

### 8.2 Property Calculation

The `Port::propssi()` method wraps the FFI call, converting between the simulator's units and CoolProp's SI units:

| Property | Simulator Unit | CoolProp Unit | Conversion |
|---|---|---|---|
| Pressure | MPa | Pa | × 10^6 |
| Enthalpy | kJ/kg | J/kg | × 1000 |
| Entropy | kJ/kg·K | J/kg·K | × 1000 |
| Temperature | °C | K | + 273.15 |

### 8.3 Quality Handling

CoolProp returns a quality value of -1.0 when the state is outside the two-phase region (subcooled liquid or superheated vapor). The simulator converts this to NaN to distinguish it from a valid quality value.

### 8.4 CoolProp Dependency

The `coolprop-sys` crate (version 7.2.2) is listed as a dependency in `Cargo.toml`. It bundles the native CoolProp dynamic libraries for all supported platforms and handles linking and DLL deployment automatically during the build process. No manual download or setup of the CoolProp shared library is required.

---

## 9. JSON Configuration Format

### 9.1 Structure

A cycle configuration JSON file has the following structure:

```json
{
    "name": "Cycle name",
    "refrigerant": "R134a",
    "components": [
        {
            "name": "ComponentName",
            "classstr": "ClassName",
            "iPort": { ... },
            "oPort": { ... }
        }
    ],
    "connectors": {
        "Component0.portName": "Component1.portName"
    }
}
```

### 9.2 Component Specification

Each component requires:

- **`name`**: Unique identifier for the component
- **`classstr`**: One of `"Compressor"`, `"Condenser"`, `"Evaporator"`, `"ExpansionValve"`
- **`iPort`**: Input port initial conditions (key-value pairs of property names to values)
- **`oPort`**: Output port initial conditions

Port properties that can be specified: `p`, `t`, `h`, `s`, `x`, `mdot`. Only provide the known properties; the simulator will compute the rest.

### 9.3 Connector Specification

Connectors are specified as key-value pairs where both key and value use the format `"ComponentName.PortName"`:

```json
"Compressor.oPort": "Condenser.iPort"
```

This connects the compressor's output port to the condenser's input port, creating a shared node.

### 9.4 Component Order Independence

The order of components in the JSON array does not affect simulation results. The `component_simulator` algorithm automatically discovers the correct processing order regardless of how the components are listed.

---

## 10. Build System and Deployment

### 10.1 Build Script

The `build.rs` script is a no-op — all CoolProp linking and DLL deployment is handled automatically by the `coolprop-sys` crate, which bundles the native CoolProp dynamic libraries for all supported platforms and copies the appropriate library to the target directory during build.

### 10.2 Prerequisites

- **Rust toolchain**: 2021 Edition (Rust 1.56+)
- **CoolProp**: No manual setup required. The `coolprop-sys` crate bundles the native libraries for all supported platforms (Windows x86-64, Windows AArch64, Linux x86-64, macOS x86-64, macOS AArch64).

### 10.3 Building and Running

```bash
# Build
cargo build

# Run with default configuration
cargo run

# Run with custom configuration
cargo run -- path/to/config.json
```

---

## 11. Simulation Workflow

The complete simulation workflow proceeds as follows:

```
1. Load JSON configuration file
   └── JSONLoader::load_file() → serde_json::Value

2. Parse JSON into VCCycle
   └── JSONLoader::create_cycle()
       ├── Parse components → Vec<UMComponent>
       ├── Parse connectors → Vec<TupConnector>
       └── VCCycle::new()
           ├── Instantiate components based on "classstr"
           ├── Build connectors (node sharing)
           └── Set port addresses

3. Run simulation
   └── VCCycle::simulator()
       ├── component_simulator()
       │   └── Iterative panic-driven algorithm
       │       ├── For each component: state() → update nodes → balance()
       │       └── Repeat until all components processed
       └── Aggregate cycle performance indicators
           ├── Wc ← sum of Compressor.wc
           ├── Qin ← sum of Evaporator.qe
           ├── Qout ← sum of Condenser.qc
           ├── COP = Qin / Wc
           └── COP_hp = Qout / Wc

4. Output results
   └── VCCycle::outresults()
       ├── Cycle-level summary (Wc, Qin, Qout, COP, COP_hp)
       ├── Per-component results
       └── Per-node thermodynamic states
```

---

## 12. Cycle Performance Indicators

The simulator computes the following cycle-level performance indicators:

| Indicator | Formula | Unit |
|---|---|---|
| Compression Work (Wc) | mdot × (h_out − h_in) for each compressor | kW |
| Refrigeration Capacity (Qin) | mdot × (h_out − h_in) for each evaporator | kW |
| Capacity (tons) | Qin × 60 × (1/211) | tons |
| Heat Transfer Rate (Qout) | mdot × (h_in − h_out) for each condenser | kW |
| COP (cooling) | Qin / Wc | dimensionless |
| COP (heat pump) | Qout / Wc | dimensionless |

---

## 13. Demo Example

The `jsonmodel/demovcc.json` file demonstrates a standard R134a vapor compression cycle:

### 13.1 Configuration

- **Refrigerant**: R134a
- **Compressor inlet**: T = 0°C, x = 1.0 (saturated vapor), mdot = 0.08 kg/s
- **Compressor outlet**: P = 0.6854 MPa
- **Condenser outlet**: T = 26°C, x = 0.0 (saturated liquid)

### 13.2 Cycle Topology

```
Compressor.oPort → Condenser.iPort
Condenser.oPort  → ExpansionValve.iPort
ExpansionValve.oPort → Evaporator.iPort
Evaporator.oPort → Compressor.iPort
```

### 13.3 Simulation Process

1. **Iteration 1**: The compressor can process first (it has T, x at inlet and P at outlet). It sets oPort.s = iPort.s, then the node resolves the full state at the compressor outlet. The condenser can now process (it has P from the shared node and T, x at its outlet). The expansion valve and evaporator follow in subsequent iterations.

2. **Result aggregation**: After all components are processed, the cycle-level indicators are computed.

### 13.4 Expected Output Format

```
 --- The Cycle ---
    Compression Work(kW): ...
    Refrigeration Capacity(kW): ...
    Capacity(ton): ...
    The heat transfer rate(kW): ...
    The coefficient of performance: ...
    The coefficient of performance(heat pump): ...

Compressor
Port     P(MPa)   T(C)  H(kJ/kg)    S(kJ/kg.K)  Quality MDOT(kg/s)
0        ...      ...   ...         ...          ...     ...
1        ...      ...   ...         ...          ...     ...
The compressor Work(kW): ...

Condenser
...
```

---

## 14. Design Decisions and Trade-offs

### 14.1 Panic-Driven Control Flow

**Decision**: Use panics as a control flow mechanism for component readiness detection.

**Rationale**: This approach eliminates the need for explicit dependency tracking or topological sorting. Components simply panic when they lack input data, and the algorithm retries them later. This makes the system simple to extend — adding a new component type requires no changes to the simulation algorithm.

**Trade-off**: Panics are expensive in Rust (they unwind the stack). However, for a small number of components (typically 4–8), the performance impact is negligible. The simplicity and extensibility benefits outweigh the cost.

### 14.2 Raw Pointers for Node Sharing

**Decision**: Use `*mut Port` raw pointers for node sharing between components.

**Rationale**: Rust's ownership model does not naturally support shared mutable references. Raw pointers allow multiple components to reference and modify the same `Port` object without borrow checker conflicts, while the `Connector` maintains clear ownership of the memory.

**Trade-off**: This introduces `unsafe` code throughout the component implementations. The safety guarantee relies on the invariant that the `Connector` outlives all components and that no two components simultaneously modify the same port property. For a single-threaded simulation, this is safe.

### 14.3 HashMap-Based Component Storage

**Decision**: Store components in a `HashMap<String, Box<dyn CompSISO>>`.

**Rationale**: Enables dynamic component creation from JSON configuration and O(1) lookup by name. The `Box<dyn CompSISO>` allows heterogeneous storage of different component types.

**Trade-off**: Downcasting is required to access component-specific fields during result aggregation (e.g., `Compressor.wc`). This adds runtime overhead but is acceptable since it only occurs once after simulation.

### 14.4 JSON Configuration

**Decision**: Use JSON files for cycle configuration rather than programmatic API.

**Rationale**: JSON is human-readable, widely supported, and allows non-programmers to define and modify cycle configurations. It also facilitates integration with other tools and workflows.

**Trade-off**: JSON parsing adds a dependency on `serde` and `serde_json`, and the schema is not formally validated. Invalid configurations may produce runtime errors rather than compile-time errors.

---

## 15. Future Directions

### 15.1 Potential Enhancements

- **Non-ideal component models**: Isentropic efficiency for compressors, pressure drops in heat exchangers, subcooling and superheating.
- **Multi-stage cycles**: Cascade systems, two-stage compression with intercooling.
- **Graphical output**: P-h diagrams, T-s diagrams, cycle visualization.
- **Sensitivity analysis**: Automated parameter sweeps and optimization.
- **Formal JSON schema**: Validation of configuration files before simulation.
- **Error recovery**: More graceful handling of unconverged simulations with diagnostic output.
- **Safe abstractions**: Replace raw pointers with `Rc<RefCell<Port>>` or similar safe Rust patterns.
- **Async/parallel simulation**: For larger systems with independent sub-cycles.

### 15.2 Integration Possibilities

- **WebAssembly**: Compile to Wasm for browser-based simulation tools.
- **Python bindings**: Via PyO3 for integration with scientific Python ecosystems.
- **Model exchange**: Support for Modelica, EnergyPlus, or other simulation frameworks.

---

*End of Technical Report*
