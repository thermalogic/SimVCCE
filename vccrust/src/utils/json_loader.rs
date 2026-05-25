//! JSON loader module

use crate::common::{UMComponent};
use crate::vcc::VCCycle;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct JSONLoader;

impl JSONLoader {
    pub fn new() -> Self {
        JSONLoader
    }

    pub fn load_file<P: AsRef<Path>>(&self, filename: P) -> Result<serde_json::Value, String> {
        let mut file = File::open(filename).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn create_cycle(&self, json_value: &serde_json::Value) -> Result<VCCycle, String> {
        let mut components = Vec::new();
        let mut comp_order = Vec::new();
        let mut connectors = Vec::new();

        if let Some(comps) = json_value.get("components").and_then(|v| v.as_array()) {
            for comp in comps {
                if let Some(obj) = comp.as_object() {
                    let um_comp: UMComponent = obj.clone().into_iter().collect();
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        comp_order.push(name.to_string());
                    }
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

        Ok(VCCycle::new(components, connectors, comp_order))
    }
}

impl Default for JSONLoader {
    fn default() -> Self {
        Self::new()
    }
}
