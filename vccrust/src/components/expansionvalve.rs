//! Expansion valve component: isenthalpic throttling process.
//!
//! Models an ideal expansion valve where enthalpy is conserved (h_in = h_out).
//! No energy calculation — the expansion valve only performs mass balance.
//!
//! # Energy Category
//! `""` (empty) — no energy contribution to cycle-level indicators.
//!
//! # Panic Conditions
//! - `state()`: panics if both ports' h are NaN
//! - `balance()`: panics if both ports' mdot are NaN

use crate::common::{CompSISO, Port, UMComponent, any_to_string, PortDictMut, PortDict};
use std::collections::HashMap;

/// Expansion valve component for the vapor compression cycle.
///
/// Implements isenthalpic throttling: the output port's enthalpy equals the
/// input port's enthalpy. Only mass balance is performed (no energy calculation).
pub struct ExpansionValve {
    /// Component name
    pub name: String,
    /// Energy category: "" (no energy)
    pub energy: String,
    /// Input port pointer
    pub i_port: *mut Port,
    /// Output port pointer
    pub o_port: *mut Port,
    /// Port dictionary: {"iPort" → ptr, "oPort" → ptr}
    pub portdict: HashMap<String, *mut Port>,
}

impl ExpansionValve {
    /// Creates a new ExpansionValve from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent) -> Self {
        let name = any_to_string(dict_comp.get("name").unwrap());
        
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

        let i_port = Box::into_raw(Box::new(Port::new(&i_port_data)));
        let o_port = Box::into_raw(Box::new(Port::new(&o_port_data)));

        let mut portdict = HashMap::new();
        portdict.insert("iPort".to_string(), i_port);
        portdict.insert("oPort".to_string(), o_port);

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

    fn setportaddress(&mut self) {
        if self.i_port != *self.portdict.get("iPort").unwrap() {
            self.i_port = *self.portdict.get("iPort").unwrap();
        }
        if self.o_port != *self.portdict.get("oPort").unwrap() {
            self.o_port = *self.portdict.get("oPort").unwrap();
        }
    }

    /// Isenthalpic throttling: propagates enthalpy between ports.
    ///
    /// If one port has a valid enthalpy and the other doesn't, copies it.
    ///
    /// # Panics
    /// Panics if both ports' h are NaN (no enthalpy information available).
    fn state(&mut self) {
        unsafe {
            if !(*self.i_port).h.is_nan() && (*self.o_port).h.is_nan() {
                (*self.o_port).h = (*self.i_port).h;
            } else if !(*self.o_port).h.is_nan() && (*self.i_port).h.is_nan() {
                (*self.i_port).h = (*self.o_port).h;
            } else if (*self.i_port).h.is_nan() && (*self.o_port).h.is_nan() {
                panic!("ExpansionValve: both ports h are NaN");
            }
        }
    }

    /// Mass balance for expansion valve.
    ///
    /// Propagates mdot between ports. No energy calculation.
    ///
    /// # Panics
    /// Panics if both ports' mdot are NaN.
    fn balance(&mut self) {
        unsafe {
            if (*self.i_port).mdot.is_nan() && (*self.o_port).mdot.is_nan() {
                panic!("ExpansionValve: mdot is NaN");
            }
            if !(*self.i_port).mdot.is_nan() {
                (*self.o_port).mdot = (*self.i_port).mdot;
            } else if !(*self.o_port).mdot.is_nan() {
                (*self.i_port).mdot = (*self.o_port).mdot;
            }
        }
    }

    fn resultstring(&self) -> String {
        unsafe {
            format!(
                "\n{}\n{}\n{}\n{}\n",
                self.name,
                Port::TITLE,
                (*self.i_port).resultstring(),
                (*self.o_port).resultstring()
            )
        }
    }
}

impl PortDict for ExpansionValve {
    fn portdict(&self) -> &HashMap<String, *mut Port> {
        &self.portdict
    }
}

impl PortDictMut for ExpansionValve {
    fn portdict_mut(&mut self) -> &mut HashMap<String, *mut Port> {
        &mut self.portdict
    }
}
