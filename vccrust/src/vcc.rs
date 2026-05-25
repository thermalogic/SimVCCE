//! Vapor Compression Cycle (VCC) simulator module.
//!
//! This module implements the core cycle simulation logic, including:
//! - Component calculation order detection algorithm (`component_simulator`)
//! - Cycle performance indicator aggregation (`simulator`)
//! - Result output formatting
//!
//! # Algorithm: Component Calculation Order Detection
//!
//! The system uses an iterative approach to automatically determine the correct
//! processing order of components. **No fixed order is required** — the algorithm
//! discovers the order dynamically:
//!
//! 1. All components are placed in a processing queue (`keys`).
//! 2. Each iteration attempts to process every component in the queue:
//!    - `state()` — thermal process calculation
//!    - Update unresolved nodes — propagate state through shared nodes
//!    - `balance()` — energy and mass balance calculation
//! 3. If a component is successfully processed (no panic), it is removed from the queue.
//! 4. If a component panics (input data not yet available), it is skipped and
//!    retried in the next iteration when upstream components may have provided
//!    the required data.
//! 5. The loop terminates when all components are processed or the maximum
//!    iteration count is reached.
//!
//! The key insight: **panics are not errors, but signals that the component
//! is "not ready yet"**. Through node sharing, upstream results propagate
//! to downstream components automatically.

use crate::common::{CompSISO, TupConnector, UMComponent, to_string_with_precision, Port};
use crate::core::Connector;
use crate::components::{Compressor, Condenser, Evaporator, ExpansionValve};
use std::collections::HashMap;

/// The vapor compression refrigeration cycle simulator.
///
/// Coordinates all components and connectors, runs the component calculation
/// order detection algorithm, and aggregates cycle-level performance indicators.
pub struct VCCycle {
    /// The connector managing shared nodes between component ports
    pub curcon: Connector,
    /// HashMap of component name -> component object
    pub comps: HashMap<String, Box<dyn CompSISO>>,
    /// Total compression work (kW)
    pub wc: f64,
    /// Total refrigeration capacity (kW)
    pub qin: f64,
    /// Total heat transfer rate of condenser (kW)
    pub qout: f64,
    /// Coefficient of performance (Qin / Wc)
    pub cop: f64,
    /// Coefficient of performance for heat pump (Qout / Wc)
    pub cop_hp: f64,
}

impl VCCycle {
    /// Creates a new VCCycle from component definitions and connector specifications.
    ///
    /// # Arguments
    /// * `dict_comps` - Vector of component configuration dictionaries
    /// * `vec_connectors` - Vector of connector tuples specifying port connections
    ///
    /// # Process
    /// 1. Instantiate components based on `classstr` field
    /// 2. Build connectors to create shared nodes between ports
    /// 3. Set port addresses for all components (point ports to shared nodes)
    pub fn new(dict_comps: Vec<UMComponent>, vec_connectors: Vec<TupConnector>) -> Self {
        let mut comps = HashMap::new();

        for item in dict_comps {
            let class_str = item.get("classstr")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let name = item.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("").to_string();

            match class_str {
                "Compressor" => {
                    comps.insert(name, Box::new(Compressor::new(&item)) as Box<dyn CompSISO>);
                }
                "Condenser" => {
                    comps.insert(name, Box::new(Condenser::new(&item)) as Box<dyn CompSISO>);
                }
                "Evaporator" => {
                    comps.insert(name, Box::new(Evaporator::new(&item)) as Box<dyn CompSISO>);
                }
                "ExpansionValve" => {
                    comps.insert(name, Box::new(ExpansionValve::new(&item)) as Box<dyn CompSISO>);
                }
                _ => {}
            }
        }

        let mut curcon = Connector::new();
        for tconn in vec_connectors {
            curcon.add_connector(tconn, &mut comps);
        }

        // Set port addresses for all components
        for comp in comps.values_mut() {
            comp.setportaddress();
        }

        VCCycle {
            curcon,
            comps,
            wc: 0.0,
            qin: 0.0,
            qout: 0.0,
            cop: 0.0,
            cop_hp: 0.0,
        }
    }

    /// Component calculation order detection algorithm.
    ///
    /// Iteratively processes components until all are successfully calculated
    /// or the maximum iteration count is reached. Components that panic
    /// (due to missing input data) are skipped and retried in subsequent iterations.
    ///
    /// # Algorithm Steps (per iteration)
    /// 1. Copy the current keys list to avoid modifying during iteration
    /// 2. For each component in the copy:
    ///    - Call `state()` for thermal process calculation
    ///    - Update all unresolved nodes by calling their `state()` method
    ///    - Call `balance()` for energy and mass balance
    ///    - If all steps succeed, remove the component from keys
    ///    - If any step panics, skip this component (it stays in keys)
    /// 3. Repeat until keys is empty or max iterations reached
    fn component_simulator(&mut self) {
        let mut state_nodes = self.curcon.nodes.clone();

        let mut keys: Vec<String> = self.comps.keys().cloned().collect();
        let mut deviceok = false;
        let counts_dev = self.comps.len();
        let mut i = 0;

        while !deviceok && i <= counts_dev {
            let keys_to_process = keys.clone();
            for curdev in keys_to_process {
                // Try to process this device; catch panics to allow retry
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // Step 1: thermal process calculation
                    let comp = self.comps.get_mut(&curdev).unwrap();
                    comp.state();

                    // Step 2: update unresolved nodes
                    let mut j = 0;
                    while j < state_nodes.len() {
                        unsafe {
                            if !(*state_nodes[j]).stateok {
                                (*state_nodes[j]).state();
                                if (*state_nodes[j]).stateok {
                                    state_nodes.remove(j);
                                } else {
                                    j += 1;
                                }
                            } else {
                                j += 1;
                            }
                        }
                    }

                    // Step 3: energy and mass balance
                    let comp = self.comps.get_mut(&curdev).unwrap();
                    comp.balance();
                }));

                if result.is_ok() {
                    // Successfully processed, remove from keys
                    if let Some(pos) = keys.iter().position(|x| x == &curdev) {
                        keys.remove(pos);
                    }
                }
            }
            i += 1;
            if keys.is_empty() {
                deviceok = true;
            }
        }

        if !keys.is_empty() {
            println!("{:?}", keys);
        }
    }

    /// Runs the full cycle simulation.
    ///
    /// 1. Calls `component_simulator` to process all components
    /// 2. Aggregates cycle performance indicators from component results
    ///    by checking each component's `energy` field:
    ///    - "CompressionWork" → Wc (Compressor)
    ///    - "QIN" → Qin (Evaporator)
    ///    - "QOUT" → Qout (Condenser)
    /// 3. Calculates COP and COP_hp
    pub fn simulator(&mut self) {
        self.component_simulator();

        self.wc = 0.0;
        self.qin = 0.0;
        self.qout = 0.0;

        for (_, comp) in &self.comps {
            if comp.energy() == "CompressionWork" {
                if let Some(compressor) = comp.as_any().downcast_ref::<Compressor>() {
                    self.wc += compressor.wc;
                }
            } else if comp.energy() == "QIN" {
                if let Some(evaporator) = comp.as_any().downcast_ref::<Evaporator>() {
                    self.qin += evaporator.qe;
                }
            } else if comp.energy() == "QOUT" {
                if let Some(condenser) = comp.as_any().downcast_ref::<Condenser>() {
                    self.qout += condenser.qc;
                }
            }
        }

        self.cop = self.qin / self.wc;
        self.cop_hp = self.qout / self.wc;
    }

    /// Runs the simulator (alias for `simulator()`).
    pub fn state(&mut self) {
        self.simulator();
    }

    /// Balance is already handled within `simulator()`.
    pub fn balance(&mut self) {
        // Already handled in simulator
    }

    /// Returns a formatted string of cycle-level performance indicators.
    pub fn resultstr(&self) -> String {
        format!(
            "\n --- The Cycle ---
\tCompression Work(kW): {}
\tRefrigeration Capacity(kW): {}
\tCapacity(ton): {}
\tThe heat transfer rate(kW): {}
\tThe coefficient of performance: {}
\tThe coefficient of performance(heat pump): {}
",
            to_string_with_precision(self.wc, 3),
            to_string_with_precision(self.qin, 3),
            to_string_with_precision(self.qin * 60.0 * (1.0 / 211.0), 3),
            to_string_with_precision(self.qout, 3),
            to_string_with_precision(self.cop, 3),
            to_string_with_precision(self.cop_hp, 3)
        )
    }

    /// Prints each component's result string and all node states.
    pub fn outdevresultstr(&self) {
        for (_, comp) in &self.comps {
            println!("{}", comp.resultstring());
        }
        println!("\n{}", Port::TITLE);
        for &node in &self.curcon.nodes {
            unsafe {
                println!("{}", (*node).resultstring());
            }
        }
    }

    /// Prints the full cycle results (summary + component details + node states).
    pub fn outresults(&self) {
        println!("{}", self.resultstr());
        self.outdevresultstr();
    }
}
