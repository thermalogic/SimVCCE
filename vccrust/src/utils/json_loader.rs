//! JSON loader module: parses JSON configuration files into VCCycle instances.
//!
//! Reads a JSON file containing component definitions and connector specifications,
//! then constructs a [`VCCycle`](crate::vcc::VCCycle) object ready for simulation.
//!
//! # JSON Format
//! ```json
//! {
//!   "components": [
//!     { "classstr": "Compressor", "name": "Compressor", "iPort": {...}, "oPort": {...} },
//!     ...
//!   ],
//!   "connectors": {
//!     "Compressor.oPort": "Condenser.iPort",
//!     ...
//!   }
//! }
//! ```
//!
//! Note: The order of components in the JSON file does not matter — the
//! `component_simulator` algorithm automatically detects the correct
//! calculation order.

use crate::common::{UMComponent};
use crate::vcc::VCCycle;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// JSON configuration file loader for VCCycle.
pub struct JSONLoader;

impl JSONLoader {
    /// Creates a new JSONLoader instance.
    pub fn new() -> Self {
        JSONLoader
    }

    /// Loads and parses a JSON file.
    ///
    /// # Arguments
    /// * `filename` - Path to the JSON configuration file
    ///
    /// # Errors
    /// Returns an error string if the file cannot be opened, read, or parsed.
    pub fn load_file<P: AsRef<Path>>(&self, filename: P) -> Result<serde_json::Value, String> {
        let mut file = File::open(filename).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    /// Creates a VCCycle from a parsed JSON value.
    ///
    /// # Process
    /// 1. Extracts component definitions from the "components" array
    /// 2. Extracts connector specifications from the "connectors" object
    /// 3. Constructs and returns a VCCycle
    ///
    /// # Connector Format
    /// Connectors are specified as key-value pairs where both key and value
    /// use the format "ComponentName.PortName", e.g.:
    /// `"Compressor.oPort": "Condenser.iPort"`
    pub fn create_cycle(&self, json_value: &serde_json::Value) -> Result<VCCycle, String> {
        let mut components = Vec::new();
        let mut connectors = Vec::new();

        let fluid_name = json_value
            .get("refrigerant")
            .and_then(|v| v.as_str())
            .unwrap_or("R134a");

        if let Some(comps) = json_value.get("components").and_then(|v| v.as_array()) {
            for comp in comps {
                if let Some(obj) = comp.as_object() {
                    let um_comp: UMComponent = obj.clone().into_iter().collect();
                    components.push(um_comp);
                }
            }
        }

        if let Some(conns) = json_value.get("connectors").and_then(|v| v.as_object()) {
            for (key, value) in conns {
                if let Some(val_str) = value.as_str() {
                    let key_parts: Vec<&str> = key.split('.').collect();
                    let val_parts: Vec<&str> = val_str.split('.').collect();
                    if key_parts.len() == 2 && val_parts.len() == 2 {
                        connectors.push((
                            (key_parts[0].to_string(), key_parts[1].to_string()),
                            (val_parts[0].to_string(), val_parts[1].to_string()),
                        ));
                    }
                }
            }
        }

        VCCycle::new(components, connectors, fluid_name).map_err(|e| e.to_string())
    }
}

impl Default for JSONLoader {
    fn default() -> Self {
        Self::new()
    }
}
