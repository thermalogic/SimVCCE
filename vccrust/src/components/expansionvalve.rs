//! Expansion valve component: isenthalpic throttling process.
//!
//! Models an ideal expansion valve where enthalpy is conserved (h_in = h_out).
//! No energy calculation — the expansion valve only performs mass balance.
//!
//! # Energy Category
//! `""` (empty) — no energy contribution to cycle-level indicators.
//!
//! # Error Conditions
//! - `state()`: returns `Err` if both ports' h are NaN
//! - `balance()`: returns `Err` if both ports' mdot are NaN

use crate::common::{CompSISO, Port, PortRef, SimulationError, UMComponent, PortDictMut, PortDict};
use std::collections::HashMap;
use std::rc::Rc;

/// Expansion valve component for the vapor compression cycle.
///
/// Implements isenthalpic throttling: the output port's enthalpy equals the
/// input port's enthalpy. Only mass balance is performed (no energy calculation).
pub struct ExpansionValve {
    /// Component name
    pub name: String,
    /// Energy category: "" (no energy)
    pub energy: String,
    /// Input port reference
    pub i_port: PortRef,
    /// Output port reference
    pub o_port: PortRef,
    /// Port dictionary: {"iPort" → ref, "oPort" → ref}
    pub portdict: HashMap<String, PortRef>,
}

impl ExpansionValve {
    /// Creates a new ExpansionValve from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        let name = dict_comp.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("ExpansionValve")
            .to_string();
        
        // Parse iPort
        let i_port_data: HashMap<String, f64> = dict_comp
            .get("iPort")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
                    .collect()
            })
            .unwrap_or_default();
        
        // Parse oPort
        let o_port_data: HashMap<String, f64> = dict_comp
            .get("oPort")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
                    .collect()
            })
            .unwrap_or_default();

        let i_port = Rc::new(std::cell::RefCell::new(Port::new(&i_port_data, fluid_name)));
        let o_port = Rc::new(std::cell::RefCell::new(Port::new(&o_port_data, fluid_name)));

        let mut portdict = HashMap::new();
        portdict.insert("iPort".to_string(), i_port.clone());
        portdict.insert("oPort".to_string(), o_port.clone());

        ExpansionValve {
            name,
            energy: "".to_string(),
            i_port,
            o_port,
            portdict,
        }
    }
}

impl CompSISO for ExpansionValve {
    fn name(&self) -> &str {
        &self.name
    }

    fn energy(&self) -> &str {
        &self.energy
    }

    fn energy_value(&self) -> f64 {
        0.0
    }

    fn set_port_address(&mut self) {
        if let Some(i_port) = self.portdict.get("iPort") {
            if !Rc::ptr_eq(&self.i_port, i_port) {
                self.i_port = i_port.clone();
            }
        }
        if let Some(o_port) = self.portdict.get("oPort") {
            if !Rc::ptr_eq(&self.o_port, o_port) {
                self.o_port = o_port.clone();
            }
        }
    }

    /// Isenthalpic throttling: propagates enthalpy between ports.
    ///
    /// If one port has a valid enthalpy and the other doesn't, copies it.
    ///
    /// # Errors
    /// Returns `Err` if both ports' h are NaN (no enthalpy information available).
    fn state(&mut self) -> Result<(), SimulationError> {
        let i_h = self.i_port.borrow().h;
        let o_h = self.o_port.borrow().h;
        if !i_h.is_nan() && o_h.is_nan() {
            self.o_port.borrow_mut().h = i_h;
        } else if !o_h.is_nan() && i_h.is_nan() {
            self.i_port.borrow_mut().h = o_h;
        } else if i_h.is_nan() && o_h.is_nan() {
            return Err(SimulationError::new("ExpansionValve: both ports h are NaN"));
        }
        Ok(())
    }

    /// Mass balance for expansion valve.
    ///
    /// Propagates mdot between ports. No energy calculation.
    ///
    /// # Errors
    /// Returns `Err` if both ports' mdot are NaN.
    fn balance(&mut self) -> Result<(), SimulationError> {
        let i_mdot = self.i_port.borrow().mdot;
        let o_mdot = self.o_port.borrow().mdot;
        if i_mdot.is_nan() && o_mdot.is_nan() {
            return Err(SimulationError::new("ExpansionValve: mdot is NaN"));
        }
        if !i_mdot.is_nan() {
            self.o_port.borrow_mut().mdot = i_mdot;
        } else if !o_mdot.is_nan() {
            self.i_port.borrow_mut().mdot = o_mdot;
        }
        Ok(())
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\n{}\n{}\n",
            self.name,
            Port::TITLE,
            self.i_port.borrow().result_string(),
            self.o_port.borrow().result_string()
        )
    }
}

impl PortDict for ExpansionValve {
    fn portdict(&self) -> &HashMap<String, PortRef> {
        &self.portdict
    }
}

impl PortDictMut for ExpansionValve {
    fn portdict_mut(&mut self) -> &mut HashMap<String, PortRef> {
        &mut self.portdict
    }
}
