//! Shared SISO (Single-Input Single-Output) component struct and logic.
//!
//! [`SISOComponent`] encapsulates the shared fields and logic (port dictionary,
//! name, energy category, port address updates, mdot propagation) common to all
//! SISO components. Individual components embed this struct and only implement
//! their specific `state()` and `balance()` logic.

use crate::common::{Port, PortDict, PortDictMut, PortRef, SimulationError, UMComponent};
use std::collections::HashMap;
use std::rc::Rc;

/// Shared fields and logic for all Single-Input Single-Output components.
///
/// Each concrete component (Compressor, Condenser, Evaporator, ExpansionValve)
/// embeds this struct via `Deref`/`DerefMut` delegation, and only implements
/// the component-specific `state()` and `balance()` methods.
pub struct SISOComponent {
    /// Component name
    pub name: String,
    /// Energy category: "CompressionWork", "QOUT", "QIN", or ""
    pub energy: String,
    /// Input port reference
    pub i_port: PortRef,
    /// Output port reference
    pub o_port: PortRef,
    /// Port dictionary: {"iPort" → ref, "oPort" → ref}
    pub portdict: HashMap<String, PortRef>,
}

impl SISOComponent {
    /// Creates a new SISOComponent from a JSON component configuration.
    ///
    /// Parses the "name", "iPort", and "oPort" fields from the JSON dictionary,
    /// constructs Port objects, and builds the port dictionary.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str, default_name: &str, energy: &str) -> Self {
        let name = dict_comp
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(default_name)
            .to_string();

        let i_port_data: HashMap<String, f64> = dict_comp
            .get("iPort")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
                    .collect()
            })
            .unwrap_or_default();

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

        SISOComponent {
            name,
            energy: energy.to_string(),
            i_port,
            o_port,
            portdict,
        }
    }

    /// Updates i_port/o_port from portdict (after node sharing).
    pub fn set_port_address(&mut self) {
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

    /// Propagates mass flow rate between ports.
    ///
    /// If one port has a valid mdot and the other doesn't, copies it.
    ///
    /// # Errors
    /// Returns `Err` if both ports' mdot are NaN.
    pub fn propagate_mdot(&self, comp_name: &str) -> Result<(), SimulationError> {
        let i_mdot = self.i_port.borrow().mdot;
        let o_mdot = self.o_port.borrow().mdot;
        if i_mdot.is_nan() && o_mdot.is_nan() {
            return Err(SimulationError::new(format!("{}: mdot is NaN", comp_name)));
        }
        if !i_mdot.is_nan() {
            self.o_port.borrow_mut().mdot = i_mdot;
        } else if !o_mdot.is_nan() {
            self.i_port.borrow_mut().mdot = o_mdot;
        }
        Ok(())
    }

    /// Returns the enthalpy values of both ports.
    ///
    /// # Errors
    /// Returns `Err` if either port's h is NaN.
    pub fn get_enthalpies(&self, comp_name: &str) -> Result<(f64, f64), SimulationError> {
        let i_h = self.i_port.borrow().h;
        let o_h = self.o_port.borrow().h;
        if i_h.is_nan() || o_h.is_nan() {
            return Err(SimulationError::new(format!("{}: h is NaN", comp_name)));
        }
        Ok((i_h, o_h))
    }

    /// Returns the mass flow rate from the input port.
    pub fn mdot(&self) -> f64 {
        self.i_port.borrow().mdot
    }

    /// Returns a formatted header string with port states.
    pub fn port_result_string(&self) -> String {
        format!(
            "{}\n{}\n{}",
            Port::TITLE,
            self.i_port.borrow().result_string(),
            self.o_port.borrow().result_string()
        )
    }
}

impl PortDict for SISOComponent {
    fn portdict(&self) -> &HashMap<String, PortRef> {
        &self.portdict
    }
}

impl PortDictMut for SISOComponent {
    fn portdict_mut(&mut self) -> &mut HashMap<String, PortRef> {
        &mut self.portdict
    }
}
