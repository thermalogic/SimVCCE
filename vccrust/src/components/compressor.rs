//! Compressor component: isentropic compression process.
//!
//! Models an ideal compressor where entropy is conserved (s_in = s_out).
//! Calculates compression work: `Wc = mdot * (h_out - h_in)`.
//!
//! # Energy Category
//! `"CompressionWork"` — aggregated into cycle-level Wc.
//!
//! # Error Conditions
//! - `state()`: returns `Err` if iPort.s is NaN (input entropy not yet available)
//! - `balance()`: returns `Err` if both ports' mdot are NaN, or if either port's h is NaN

use crate::common::{CompSISO, Port, PortRef, SimulationError, UMComponent, to_string_with_precision, any_to_string, PortDictMut, PortDict};
use std::collections::HashMap;
use std::rc::Rc;

/// Compressor component for the vapor compression cycle.
///
/// Implements isentropic compression: the output port's entropy equals the
/// input port's entropy. Compression work is calculated from the enthalpy
/// difference and mass flow rate.
pub struct Compressor {
    /// Component name
    pub name: String,
    /// Energy category: "CompressionWork"
    pub energy: String,
    /// Input port reference
    pub i_port: PortRef,
    /// Output port reference
    pub o_port: PortRef,
    /// Port dictionary: {"iPort" → ref, "oPort" → ref}
    pub portdict: HashMap<String, PortRef>,
    /// Compression work (kW)
    pub wc: f64,
}

impl Compressor {
    /// Creates a new Compressor from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        let name = dict_comp.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Compressor")
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

        Compressor {
            name,
            energy: "CompressionWork".to_string(),
            i_port,
            o_port,
            portdict,
            wc: 0.0,
        }
    }
}

impl CompSISO for Compressor {
    fn name(&self) -> &str {
        &self.name
    }

    fn energy(&self) -> &str {
        &self.energy
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

    /// Isentropic compression: sets oPort.s = iPort.s.
    ///
    /// # Errors
    /// Returns `Err` if iPort.s is NaN (input entropy not yet determined).
    fn state(&mut self) -> Result<(), SimulationError> {
        let i_s = self.i_port.borrow().s;
        if i_s.is_nan() {
            return Err(SimulationError::new("Compressor: iPort.s is NaN"));
        }
        self.o_port.borrow_mut().s = i_s;
        Ok(())
    }

    /// Mass and energy balance for compression.
    ///
    /// - Mass: propagates mdot between ports
    /// - Energy: Wc = mdot * (h_out - h_in)
    ///
    /// # Errors
    /// - Returns `Err` if both ports' mdot are NaN
    /// - Returns `Err` if either port's h is NaN
    fn balance(&mut self) -> Result<(), SimulationError> {
        let i_mdot = self.i_port.borrow().mdot;
        let o_mdot = self.o_port.borrow().mdot;
        if i_mdot.is_nan() && o_mdot.is_nan() {
            return Err(SimulationError::new("Compressor: mdot is NaN"));
        }
        if !i_mdot.is_nan() {
            self.o_port.borrow_mut().mdot = i_mdot;
        } else if !o_mdot.is_nan() {
            self.i_port.borrow_mut().mdot = o_mdot;
        }
        let i_h = self.i_port.borrow().h;
        let o_h = self.o_port.borrow().h;
        if i_h.is_nan() || o_h.is_nan() {
            return Err(SimulationError::new("Compressor: h is NaN"));
        }
        let mdot = self.i_port.borrow().mdot;
        self.wc = mdot * (o_h - i_h);
        Ok(())
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\n{}\n{}\nThe compressor Work(kW): {}\n",
            self.name,
            Port::TITLE,
            self.i_port.borrow().result_string(),
            self.o_port.borrow().result_string(),
            to_string_with_precision(self.wc, 3)
        )
    }
}

impl PortDict for Compressor {
    fn portdict(&self) -> &HashMap<String, PortRef> {
        &self.portdict
    }
}

impl PortDictMut for Compressor {
    fn portdict_mut(&mut self) -> &mut HashMap<String, PortRef> {
        &mut self.portdict
    }
}
