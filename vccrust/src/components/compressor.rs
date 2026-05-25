//! Compressor component: isentropic compression process.
//!
//! Models an ideal compressor where entropy is conserved (s_in = s_out).
//! Calculates compression work: `Wc = mdot * (h_out - h_in)`.
//!
//! # Energy Category
//! `"CompressionWork"` — aggregated into cycle-level Wc.
//!
//! # Panic Conditions
//! - `state()`: panics if iPort.s is NaN (input entropy not yet available)
//! - `balance()`: panics if both ports' mdot are NaN, or if either port's h is NaN

use crate::common::{CompSISO, Port, UMComponent, to_string_with_precision, any_to_string, PortDictMut, PortDict};
use std::collections::HashMap;

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
    /// Input port pointer
    pub i_port: *mut Port,
    /// Output port pointer
    pub o_port: *mut Port,
    /// Port dictionary: {"iPort" → ptr, "oPort" → ptr}
    pub portdict: HashMap<String, *mut Port>,
    /// Compression work (kW)
    pub wc: f64,
}

impl Compressor {
    /// Creates a new Compressor from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
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

        let i_port = Box::into_raw(Box::new(Port::new(&i_port_data, fluid_name)));
        let o_port = Box::into_raw(Box::new(Port::new(&o_port_data, fluid_name)));

        let mut portdict = HashMap::new();
        portdict.insert("iPort".to_string(), i_port);
        portdict.insert("oPort".to_string(), o_port);

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

    fn setportaddress(&mut self) {
        if self.i_port != *self.portdict.get("iPort").unwrap() {
            self.i_port = *self.portdict.get("iPort").unwrap();
        }
        if self.o_port != *self.portdict.get("oPort").unwrap() {
            self.o_port = *self.portdict.get("oPort").unwrap();
        }
    }

    /// Isentropic compression: sets oPort.s = iPort.s.
    ///
    /// # Panics
    /// Panics if iPort.s is NaN (input entropy not yet determined).
    fn state(&mut self) {
        unsafe {
            if (*self.i_port).s.is_nan() {
                panic!("Compressor: iPort.s is NaN");
            }
            (*self.o_port).s = (*self.i_port).s;
        }
    }

    /// Mass and energy balance for compression.
    ///
    /// - Mass: propagates mdot between ports
    /// - Energy: Wc = mdot * (h_out - h_in)
    ///
    /// # Panics
    /// - Panics if both ports' mdot are NaN
    /// - Panics if either port's h is NaN
    fn balance(&mut self) {
        unsafe {
            if (*self.i_port).mdot.is_nan() && (*self.o_port).mdot.is_nan() {
                panic!("Compressor: mdot is NaN");
            }
            if !(*self.i_port).mdot.is_nan() {
                (*self.o_port).mdot = (*self.i_port).mdot;
            } else if !(*self.o_port).mdot.is_nan() {
                (*self.i_port).mdot = (*self.o_port).mdot;
            }
            if (*self.i_port).h.is_nan() || (*self.o_port).h.is_nan() {
                panic!("Compressor: h is NaN");
            }
            self.wc = (*self.i_port).mdot * ((*self.o_port).h - (*self.i_port).h);
        }
    }

    fn resultstring(&self) -> String {
        unsafe {
            format!(
                "\n{}\n{}\n{}\n{}\nThe compressor Work(kW): {}\n",
                self.name,
                Port::TITLE,
                (*self.i_port).resultstring(),
                (*self.o_port).resultstring(),
                to_string_with_precision(self.wc, 3)
            )
        }
    }
}

impl PortDict for Compressor {
    fn portdict(&self) -> &HashMap<String, *mut Port> {
        &self.portdict
    }
}

impl PortDictMut for Compressor {
    fn portdict_mut(&mut self) -> &mut HashMap<String, *mut Port> {
        &mut self.portdict
    }
}
