# SimVCC — Technical Report

## Vapor Compression Refrigeration Cycle Simulator in Rust

**Version**: 0.1.4  
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
16. [Extending the Simulator: Adding New Component Types](#16-extending-the-simulator-adding-new-component-types)

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
│   └── mod.rs           # CompSISO trait, type aliases, utilities, SimulationError
├── components/
│   ├── mod.rs           # Component module re-exports
│   ├── siso_component.rs # Shared SISOComponent struct and logic
│   ├── compressor.rs    # Compressor model
│   ├── condenser.rs     # Condenser model
│   ├── evaporator.rs    # Evaporator model
│   └── expansionvalve.rs # Expansion valve model
├── core/
│   ├── mod.rs           # Core module re-exports
│   ├── connector.rs     # Node sharing connector
│   └── port.rs          # Port thermodynamic state
└── utils/
    ├── mod.rs           # Utils module re-exports
    └── json_loader.rs   # JSON configuration parser

tests/
├── common_tests.rs          # Common module tests
├── port_tests.rs            # Port tests
├── connector_tests.rs       # Connector tests
├── siso_component_tests.rs  # SISOComponent tests
├── compressor_tests.rs      # Compressor tests
├── condenser_tests.rs       # Condenser tests
├── evaporator_tests.rs      # Evaporator tests
├── expansion_valve_tests.rs # Expansion valve tests
├── vcc_tests.rs             # VCCycle integration tests
└── json_loader_tests.rs     # JSON loader tests
```

### 3.2 Dependency Graph

```
main.rs
  └── JSONLoader (utils/json_loader.rs)
        └── VCCycle (vcc.rs)
              ├── Connector (core/connector.rs)
              │     └── Port (core/port.rs)
              │           └── CoolProp FFI
              └── Components (components/*.rs)
                    └── SISOComponent (components/siso_component.rs)
                          └── CompSISO trait (common/mod.rs)
                                └── Port (core/port.rs)
```

### 3.3 Key Abstractions

| Abstraction | Role |
|---|---|
| `Port` | Thermodynamic state at a connection point (defined in `core/port.rs`) |
| `CompSISO` | Trait for Single-Input Single-Output components |
| `SISOComponent` | Shared struct encapsulating common SISO component fields and logic |
| `Connector` | Manages node sharing between component ports |
| `VCCycle` | Orchestrates simulation and aggregates results |
| `JSONLoader` | Parses JSON configuration into VCCycle instances |
| `SimulationError` | Error type for simulation operations (not fatal — signals "not ready yet") |

---

## 4. Core Data Structures

### 4.1 Port

The `Port` struct is the fundamental data structure representing the thermodynamic state of the working fluid at a connection point. It is defined in `core/port.rs`:

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
    pub state_ok: bool,      // Whether state is fully determined
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
pub trait CompSISO: PortDict + PortDictMut {
    fn set_port_address(&mut self);
    fn state(&mut self) -> Result<(), SimulationError>;
    fn balance(&mut self) -> Result<(), SimulationError>;
    fn result_string(&self) -> String;
    fn name(&self) -> &str;
    fn energy(&self) -> &str;
    fn energy_value(&self) -> f64;
}
```

**Error Convention**: The `state()` and `balance()` methods return `Err(SimulationError)` when required input data is not yet available (i.e., when critical port properties are NaN). This is not a fatal error — it is a deliberate signaling mechanism that tells the `component_simulator` to defer processing of this component until upstream data becomes available. This replaces the previous panic-based control flow, eliminating the need for `catch_unwind` and panic hook suppression.

### 4.3 SISOComponent Struct

The `SISOComponent` struct encapsulates the shared fields and logic common to all SISO components. It is defined in `components/siso_component.rs`:

```rust
pub struct SISOComponent {
    pub name: String,
    pub energy: String,
    pub i_port: PortRef,     // Rc<RefCell<Port>>
    pub o_port: PortRef,     // Rc<RefCell<Port>>
    pub portdict: HashMap<String, PortRef>,
}
```

Each concrete component (Compressor, Condenser, Evaporator, ExpansionValve) embeds this struct and only implements the component-specific `state()` and `balance()` methods. `SISOComponent` provides shared methods:

- `new()` — Creates a new SISOComponent from a JSON configuration
- `set_port_address()` — Updates i_port/o_port from portdict (after node sharing)
- `propagate_mdot()` — Propagates mass flow rate between ports
- `get_enthalpies()` — Returns the enthalpy values of both ports
- `mdot()` — Returns the mass flow rate from the input port
- `port_result_string()` — Returns a formatted header string with port states

### 4.4 Supporting Traits

- **`PortDict`** / **`PortDictMut`**: Provide read/write access to a component's port dictionary (`HashMap<String, Rc<RefCell<Port>>>`).
- **`SimulationError`**: Error type implementing `std::error::Error`, used to signal that a component is not yet ready for processing.

---

## 5. Component Models

### 5.1 Compressor

**Thermodynamic process**: Isentropic compression (s_in = s_out)

The compressor model assumes an ideal isentropic compression process:

1. **`state()`**: Sets `oPort.s = iPort.s`. Returns `Err` if `iPort.s` is NaN.
2. **`balance()`**: 
   - Propagates mass flow rate: `mdot` is copied from whichever port has a valid value.
   - Calculates compression work: `Wc = mdot × (h_out - h_in)`.
   - Returns `Err` if both ports' `mdot` are NaN, or if either port's `h` is NaN.

**Energy category**: `"CompressionWork"` — contributes to cycle-level Wc.

### 5.2 Condenser

**Thermodynamic process**: Isobaric condensation (p_in = p_out)

The condenser model assumes an ideal isobaric heat rejection process:

1. **`state()`**: Propagates pressure between ports. If one port has a valid pressure and the other doesn't, copies it. Returns `Err` if both ports' `p` are NaN.
2. **`balance()`**:
   - Propagates mass flow rate.
   - Calculates heat transfer rate: `Qout = mdot × (h_in - h_out)`.
   - Returns `Err` if both ports' `mdot` are NaN, or if either port's `h` is NaN.

**Energy category**: `"QOUT"` — contributes to cycle-level Qout.

### 5.3 Evaporator

**Thermodynamic process**: Isobaric evaporation (p_in = p_out)

The evaporator model assumes an ideal isobaric heat absorption process:

1. **`state()`**: Propagates pressure between ports (same logic as condenser). Returns `Err` if both ports' `p` are NaN.
2. **`balance()`**:
   - Propagates mass flow rate.
   - Calculates refrigeration capacity: `Qin = mdot × (h_out - h_in)`.
   - Returns `Err` if both ports' `mdot` are NaN, or if either port's `h` is NaN.

**Energy category**: `"QIN"` — contributes to cycle-level Qin.

### 5.4 Expansion Valve

**Thermodynamic process**: Isenthalpic throttling (h_in = h_out)

The expansion valve model assumes an ideal isenthalpic throttling process:

1. **`state()`**: Propagates enthalpy between ports. If one port has a valid enthalpy and the other doesn't, copies it. Returns `Err` if both ports' `h` are NaN.
2. **`balance()`**: Propagates mass flow rate only. No energy calculation. Returns `Err` if both ports' `mdot` are NaN.

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
    pub nodes: Vec<Rc<RefCell<Port>>>,
}
```

Nodes are stored as `Rc<RefCell<Port>>`, enabling safe shared ownership and interior mutability. When two ports are connected, both port references in their respective component's `portdict` are replaced with the same `Rc<RefCell<Port>>`, ensuring they point to the same `Port` object.

### 6.3 Connection Process

When `add_connector()` is called with a specification like `("Compressor", "oPort") → ("Condenser", "iPort")`:

1. **Get port references**: Retrieve `Rc<RefCell<Port>>` references from both components' `portdict`.
2. **Set node index**: Assign the current node index to port0.
3. **Add port0 as node**: Port0's `Rc` is cloned and stored in the `nodes` vector.
4. **Merge port1 values**: Any known property values from port1 that are NaN in the node are copied over (via `RefCell::borrow_mut`).
5. **Replace port1 reference**: Port1's entry in its component's `portdict` is replaced with a clone of port0's `Rc`.
6. **Update port addresses**: The component's `i_port`/`o_port` fields are updated to point to the shared node.

After this process, both the compressor's `oPort` and the condenser's `iPort` hold `Rc` references to the **same `Port` object**. Any modification by one component (via `borrow_mut()`) is instantly visible to the other (via `borrow()`).

### 6.4 Memory Ownership

The `Connector` owns all shared node memory via `Rc<RefCell<Port>>`. Ports are initially created as `Rc::new(RefCell::new(Port::new(...)))` in component constructors. When a connector links two ports, both components' `portdict` entries are updated to hold clones of the same `Rc`. Memory is automatically freed when all `Rc` references are dropped — no manual memory management is required.

### 6.5 Node Sharing Diagram

```
Before connection:
  Compressor.oPort ──→ Rc<RefCell<Port A>>    Condenser.iPort ──→ Rc<RefCell<Port B>>

After connection:
  Compressor.oPort ──→ Rc<RefCell<Port A*>> ←── Condenser.iPort
                         (shared node, both Rc point to same Port)
  * Port A now contains merged values from both Port A and Port B
```

---

## 7. Component Calculation Order Detection Algorithm

### 7.1 Motivation

In a cycle, components depend on each other's outputs. For example, the compressor needs the evaporator's output state, which in turn depends on the expansion valve's output, which depends on the condenser's output, which depends on the compressor's output. This circular dependency means there is no obvious "first" component to process.

Rather than requiring the user to specify a processing order or implementing a topological sort, SimVCC uses an **iterative error-driven algorithm** that automatically discovers the correct calculation order.

### 7.2 Algorithm Description

The `component_simulator` method implements the following algorithm:

```
1. Initialize: keys = list of all component names
2. While keys is not empty and iteration count ≤ number of components:
   a. For each component in keys:
      i.   Call state()    — thermal process calculation
      ii.  Update unresolved nodes — propagate state through shared nodes
      iii. Call balance()   — energy and mass balance
      iv.  If all steps succeed (Ok(())): remove component from keys
      v.   If any step returns Err: skip component (remains in keys)
   b. Increment iteration counter
3. If keys is not empty after max iterations: report unresolved components
```

### 7.3 How It Works

The algorithm relies on two key mechanisms:

1. **Result as signal**: When a component's `state()` or `balance()` method encounters NaN input data, it returns `Err(SimulationError)`. This is treated as "not ready yet" rather than a fatal error.

2. **Node sharing propagation**: When a component successfully processes, it writes results to its output port. Because of node sharing (via `Rc<RefCell<Port>>`), these results are immediately visible to the connected downstream component's input port. On the next iteration, the downstream component may now have sufficient data to process successfully.

### 7.4 Convergence

For a well-defined cycle with sufficient initial conditions, the algorithm typically converges in at most N iterations (where N is the number of components). In practice, convergence often occurs much faster — typically 2–3 iterations for a standard four-component cycle.

### 7.5 Unresolved Node Resolution

After each component's `state()` call, the algorithm iterates through all unresolved nodes (those with `state_ok = false`) and attempts to calculate their complete state using `Port::state()`. Successfully resolved nodes are removed from the unresolved list. This ensures that partial information (e.g., a pressure value set by one component) is fully resolved before the next component attempts to use it.

### 7.6 Error Handling Benefits

The `Result`-based approach provides several advantages over the previous panic-based control flow:

- **No `catch_unwind` overhead**: Avoiding panic unwinding is more efficient and idiomatic.
- **No panic hook suppression**: The `main.rs` panic hook no longer needs to be suppressed, making debugging real panics easier.
- **Explicit error types**: `SimulationError` carries a descriptive message identifying which component and property caused the failure.
- **Composable error handling**: `Result` types can be chained with `?` operator for cleaner code.

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
       │   └── Iterative error-driven algorithm
       │       ├── For each component: state() → update nodes → balance()
       │       └── Repeat until all components processed (Err = not ready yet)
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

### 14.1 Result-Based Error Handling

**Decision**: Use `Result<(), SimulationError>` as a control flow mechanism for component readiness detection.

**Rationale**: This approach eliminates the need for explicit dependency tracking or topological sorting. Components return `Err` when they lack input data, and the algorithm retries them later. This makes the system simple to extend — adding a new component type requires no changes to the simulation algorithm. The `Result`-based approach replaced the previous panic-based control flow, providing better ergonomics, explicit error types, and eliminating the need for `catch_unwind` and panic hook suppression.

**Trade-off**: Components must propagate errors using `?` or explicit `match`, which adds minor boilerplate. However, this is idiomatic Rust and the clarity benefits outweigh the cost.

### 14.2 Rc<RefCell<Port>> for Node Sharing

**Decision**: Use `Rc<RefCell<Port>>` for node sharing between components.

**Rationale**: Rust's ownership model does not naturally support shared mutable references. `Rc<RefCell<Port>>` allows multiple components to share ownership of the same `Port` object while providing runtime-checked mutable access. This replaces the previous raw pointer (`*mut Port`) approach, eliminating all `unsafe` code from the component implementations.

**Trade-off**: `RefCell` enforces borrow rules at runtime — attempting to borrow mutably while already borrowed will panic. For a single-threaded simulation where the algorithm processes components sequentially, this is safe. The runtime check adds negligible overhead compared to CoolProp property calculations.

### 14.3 SISOComponent Shared Struct

**Decision**: Extract common SISO component fields and logic into a `SISOComponent` struct embedded by each component.

**Rationale**: All four components share the same fields (name, energy category, i_port, o_port, portdict) and common logic (port address update, mdot propagation, enthalpy retrieval). Extracting this into a shared struct reduces code duplication and ensures consistency across components.

**Trade-off**: Components must delegate `PortDict`/`PortDictMut` implementations to `inner`, adding minor boilerplate. The `inner` field is public to allow component-specific access to shared fields.

### 14.4 HashMap-Based Component Storage

**Decision**: Store components in a `HashMap<String, Box<dyn CompSISO>>`.

**Rationale**: Enables dynamic component creation from JSON configuration and O(1) lookup by name. The `Box<dyn CompSISO>` allows heterogeneous storage of different component types.

**Trade-off**: Downcasting is required to access component-specific fields during result aggregation (e.g., `Compressor.wc`). This adds runtime overhead but is acceptable since it only occurs once after simulation.

### 14.5 JSON Configuration

**Decision**: Use JSON files for cycle configuration rather than programmatic API.

**Rationale**: JSON is human-readable, widely supported, and allows non-programmers to define and modify cycle configurations. It also facilitates integration with other tools and workflows.

**Trade-off**: JSON parsing adds a dependency on `serde` and `serde_json`, and the schema is not formally validated. Invalid configurations may produce runtime errors rather than compile-time errors.

### 14.6 Test Organization

**Decision**: Separate integration tests into the `tests/` directory as independent modules, with one test file per source module.

**Rationale**: Separating tests from source code keeps the production codebase clean and follows Rust's conventional test organization. Integration tests can only access the public API, which serves as an additional verification that the public interface is sufficient.

**Trade-off**: Some internal functions that would benefit from unit testing are only testable through the public API. However, the current public API provides sufficient coverage for all critical functionality.

---

## 15. Future Directions

### 15.1 Potential Enhancements

- **Non-ideal component models**: Isentropic efficiency for compressors, pressure drops in heat exchangers, subcooling and superheating.
- **Multi-stage cycles**: Cascade systems, two-stage compression with intercooling.
- **Graphical output**: P-h diagrams, T-s diagrams, cycle visualization.
- **Sensitivity analysis**: Automated parameter sweeps and optimization.
- **Formal JSON schema**: Validation of configuration files before simulation.
- **Error recovery**: More graceful handling of unconverged simulations with diagnostic output.
- **Async/parallel simulation**: For larger systems with independent sub-cycles.

### 15.2 Completed Improvements

- ~~**Safe abstractions**: Replace raw pointers with `Rc<RefCell<Port>>` or similar safe Rust patterns.~~ — Completed. All `*mut Port` raw pointers replaced with `Rc<RefCell<Port>>`.
- ~~**Result-based error handling**: Replace panic-driven control flow with `Result<(), SimulationError>`.~~ — Completed. `state()` and `balance()` now return `Result`, eliminating `catch_unwind` and panic hook suppression.
- ~~**SISOComponent extraction**: Move shared SISO component logic into an independent module.~~ — Completed. `SISOComponent` is now in `components/siso_component.rs`.
- ~~**Test coverage**: Add comprehensive unit and integration tests.~~ — Completed. 75 tests + 1 doctest in `tests/` directory.

### 15.3 Integration Possibilities

- **WebAssembly**: Compile to Wasm for browser-based simulation tools.
- **Python bindings**: Via PyO3 for integration with scientific Python ecosystems.
- **Model exchange**: Support for Modelica, EnergyPlus, or other simulation frameworks.

---

## 16. Extending the Simulator: Adding New Component Types

SimVCC is designed as a demonstration of the iterative component calculation order detection algorithm. It ships with four SISO components (Compressor, Condenser, Evaporator, ExpansionValve). Users can extend the simulator by adding new component types following the patterns established in the codebase.

This section describes the steps required to add both SISO and MIMO (Multi-Input Multi-Output) components.

### 16.1 Adding a New SISO Component

A SISO component has exactly one input port and one output port. This is the simplest case and follows the same pattern as the existing components.

**Example: Adding an `Intercooler` (isobaric cooling between compression stages)**

**Step 1: Create the component file**

Create `src/components/intercooler.rs`:

```rust
//! Intercooler component: isobaric cooling process.

use crate::common::{CompSISO, PortDict, PortDictMut, SimulationError, UMComponent, to_string_with_precision};
use crate::components::siso_component::SISOComponent;

pub struct Intercooler {
    pub inner: SISOComponent,
    pub qc: f64,  // Heat transfer rate (kW)
}

impl Intercooler {
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        Intercooler {
            inner: SISOComponent::new(dict_comp, fluid_name, "Intercooler", "QOUT"),
            qc: 0.0,
        }
    }
}

impl CompSISO for Intercooler {
    fn name(&self) -> &str { &self.inner.name }
    fn energy(&self) -> &str { &self.inner.energy }
    fn energy_value(&self) -> f64 { self.qc }
    fn set_port_address(&mut self) { self.inner.set_port_address(); }

    fn state(&mut self) -> Result<(), SimulationError> {
        // Isobaric: propagate pressure between ports
        let i_p = self.inner.i_port.borrow().p;
        let o_p = self.inner.o_port.borrow().p;
        if !o_p.is_nan() && i_p.is_nan() {
            self.inner.i_port.borrow_mut().p = o_p;
        } else if !i_p.is_nan() && o_p.is_nan() {
            self.inner.o_port.borrow_mut().p = i_p;
        } else if i_p.is_nan() && o_p.is_nan() {
            return Err(SimulationError::new("Intercooler: both ports p are NaN"));
        }
        Ok(())
    }

    fn balance(&mut self) -> Result<(), SimulationError> {
        self.inner.propagate_mdot("Intercooler")?;
        let (i_h, o_h) = self.inner.get_enthalpies("Intercooler")?;
        self.qc = self.inner.mdot() * (i_h - o_h);
        Ok(())
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\nThe Intercooler Capacity(kW): {}\n",
            self.inner.name,
            self.inner.port_result_string(),
            to_string_with_precision(self.qc, 3)
        )
    }
}

impl PortDict for Intercooler {
    fn portdict(&self) -> &std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict()
    }
}

impl PortDictMut for Intercooler {
    fn portdict_mut(&mut self) -> &mut std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict_mut()
    }
}
```

**Step 2: Register the module in `src/components/mod.rs`**

```rust
pub mod intercooler;  // Add this line
pub use intercooler::Intercooler;  // Add this line
```

**Step 3: Register the classstr in `src/vcc.rs`**

In `VCCycle::new()`, add a match arm:

```rust
"Intercooler" => {
    comps.insert(name, Box::new(Intercooler::new(&item, fluid_name)) as Box<dyn CompSISO>);
}
```

**Step 4: Add tests**

Create `tests/intercooler_tests.rs` following the pattern of existing component tests.

**Step 5: Use in JSON**

```json
{
    "name": "Intercooler",
    "classstr": "Intercooler",
    "iPort": {},
    "oPort": { "t": 30.0, "x": 1.0 }
}
```

### 16.2 Adding a MIMO Component (Multi-Input Multi-Output)

MIMO components have more than two ports (e.g., FlashChamber with 1 input + 2 outputs, MixingChamber with 2 inputs + 1 output). This requires more extensive changes because the current `CompSISO` trait and `SISOComponent` assume exactly one input and one output port.

**Example: Adding a `FlashChamber` (isobaric flash separation)**

```
                    ↓ iPort
              ┌─────┴─────┐
              │           │
              │           │→ oPortV (vapor)
              │────────── │
              └─────┬─────┘
                    ↓ oPortL (liquid)
```

**Step 1: Define a `CompMIMO` trait**

Add to `src/common/mod.rs`:

```rust
/// Trait interface for Multi-Input Multi-Output components.
pub trait CompMIMO: PortDict + PortDictMut {
    fn set_port_address(&mut self);
    fn state(&mut self) -> Result<(), SimulationError>;
    fn balance(&mut self) -> Result<(), SimulationError>;
    fn result_string(&self) -> String;
    fn name(&self) -> &str;
    fn energy(&self) -> &str;
    fn energy_value(&self) -> f64;
}
```

**Step 2: Create the component file**

Create `src/components/flash_chamber.rs`:

```rust
//! Flash chamber component: isobaric flash separation.

use crate::common::{CompMIMO, Port, PortDict, PortDictMut, PortRef, SimulationError, UMComponent};
use std::collections::HashMap;
use std::rc::Rc;

pub struct FlashChamber {
    pub name: String,
    pub i_port: PortRef,
    pub o_port_v: PortRef,   // Vapor outlet
    pub o_port_l: PortRef,   // Liquid outlet
    pub portdict: HashMap<String, PortRef>,
}

impl FlashChamber {
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        let name = dict_comp.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("FlashChamber")
            .to_string();

        let i_port_data = /* parse "iPort" from dict_comp */;
        let o_port_v_data = /* parse "oPortV" from dict_comp */;
        let o_port_l_data = /* parse "oPortL" from dict_comp */;

        let i_port = Rc::new(std::cell::RefCell::new(Port::new(&i_port_data, fluid_name)));
        let o_port_v = Rc::new(std::cell::RefCell::new(Port::new(&o_port_v_data, fluid_name)));
        let o_port_l = Rc::new(std::cell::RefCell::new(Port::new(&o_port_l_data, fluid_name)));

        let mut portdict = HashMap::new();
        portdict.insert("iPort".to_string(), i_port.clone());
        portdict.insert("oPortV".to_string(), o_port_v.clone());
        portdict.insert("oPortL".to_string(), o_port_l.clone());

        FlashChamber { name, i_port, o_port_v, o_port_l, portdict }
    }
}

impl CompMIMO for FlashChamber {
    fn name(&self) -> &str { &self.name }
    fn energy(&self) -> &str { "" }
    fn energy_value(&self) -> f64 { 0.0 }
    fn set_port_address(&mut self) { /* update from portdict */ }

    fn state(&mut self) -> Result<(), SimulationError> {
        // Isobaric: all ports share the same pressure
        let i_p = self.i_port.borrow().p;
        let ov_p = self.o_port_v.borrow().p;
        let ol_p = self.o_port_l.borrow().p;
        // Propagate known pressure to unknown ports...
        if i_p.is_nan() && ov_p.is_nan() && ol_p.is_nan() {
            return Err(SimulationError::new("FlashChamber: all ports p are NaN"));
        }
        Ok(())
    }

    fn balance(&mut self) -> Result<(), SimulationError> {
        // Mass balance: mdot_v = mdot * x, mdot_l = mdot * (1 - x)
        let i_mdot = self.i_port.borrow().mdot;
        let i_x = self.i_port.borrow().x;
        if i_mdot.is_nan() { return Err(SimulationError::new("FlashChamber: mdot is NaN")); }
        if i_x.is_nan() { return Err(SimulationError::new("FlashChamber: x is NaN")); }
        self.o_port_v.borrow_mut().mdot = i_mdot * i_x;
        self.o_port_l.borrow_mut().mdot = i_mdot * (1.0 - i_x);
        Ok(())
    }

    fn result_string(&self) -> String { /* format output */ }
}

impl PortDict for FlashChamber {
    fn portdict(&self) -> &HashMap<String, PortRef> { &self.portdict }
}
impl PortDictMut for FlashChamber {
    fn portdict_mut(&mut self) -> &mut HashMap<String, PortRef> { &mut self.portdict }
}
```

**Step 3: Unify component storage**

The current `VCCycle.comps` uses `HashMap<String, Box<dyn CompSISO>>`. To support both SISO and MIMO components, introduce a unified trait or enum:

**Option A: Unified trait**

```rust
pub trait Comp: PortDict + PortDictMut {
    fn set_port_address(&mut self);
    fn state(&mut self) -> Result<(), SimulationError>;
    fn balance(&mut self) -> Result<(), SimulationError>;
    fn result_string(&self) -> String;
    fn name(&self) -> &str;
    fn energy(&self) -> &str;
    fn energy_value(&self) -> f64;
}
// CompSISO and CompMIMO both implement Comp
```

Then `VCCycle.comps` becomes `HashMap<String, Box<dyn Comp>>`.

**Option B: Enum dispatch**

```rust
pub enum Component {
    SISO(Box<dyn CompSISO>),
    MIMO(Box<dyn CompMIMO>),
}
```

**Step 4: Update `Connector` for multi-port nodes**

The current `add_connector` connects exactly two ports. For MIMO components, a node may need to be shared by 3+ ports. Extend `Connector`:

```rust
/// Connects multiple ports to the same shared node.
pub fn add_multi_connector(
    &mut self,
    port_specs: Vec<(String, String)>,  // [(comp_name, port_name), ...]
    comps: &mut HashMap<String, Box<dyn Comp>>,
) -> Result<(), SimulationError> {
    // 1. Get first port as the node
    // 2. Merge values from all other ports
    // 3. Replace all other ports' references with the node
    // 4. Update port addresses for all affected components
}
```

**Step 5: Update `component_simulator`**

The algorithm itself requires minimal changes — it already iterates over all components and calls `state()`/`balance()`. The only change is using the unified `Comp` trait instead of `CompSISO`.

**Step 6: Update JSON loader**

In `create_cycle()`, add match arms for new classstr values:

```rust
"FlashChamber" => {
    comps.insert(name, Box::new(FlashChamber::new(&item, fluid_name)) as Box<dyn Comp>);
}
```

### 16.3 Summary: Effort Comparison

| Task | SISO Component | MIMO Component |
|---|---|---|
| Create component file | New file, ~80 lines | New file, ~120 lines |
| Implement `state()`/`balance()` | Use `SISOComponent` helpers | Manual, custom logic |
| Register in `mod.rs` | 2 lines | 2 lines |
| Register in `vcc.rs` | 1 match arm | 1 match arm + trait unification |
| Update `Connector` | No change | Add `add_multi_connector` |
| Update `component_simulator` | No change | Use unified `Comp` trait |
| Update JSON loader | 1 match arm | 1 match arm |
| Add tests | New test file | New test file |
| **Total effort** | **~30 minutes** | **~2-4 hours** (first MIMO component) |

### 16.4 Design Principles for Extensions

1. **Follow the `Err` convention**: `state()` and `balance()` must return `Err(SimulationError)` when input data is not yet available. This is the signal that drives the iterative algorithm.

2. **Use `Rc<RefCell<Port>>` for all ports**: This ensures node sharing works correctly. Never store `Port` directly in a component — always use `PortRef`.

3. **Keep `portdict` complete**: All ports must be registered in `portdict` so that `Connector::add_connector` can find and replace them during node sharing.

4. **`state()` writes, `balance()` reads**: The two-phase design (state → node resolution → balance) is essential. `state()` should only propagate thermodynamic constraints (pressure, entropy, enthalpy). `balance()` should only compute energy and mass balances.

5. **Test with the iterative algorithm**: New components must work correctly when processed in any order. Test that your component returns `Err` when inputs are missing and `Ok` when they are available.

---

*End of Technical Report*
