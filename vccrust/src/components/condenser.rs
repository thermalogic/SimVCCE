//! Condenser component: isobaric condensation process.
//!
//! Models an ideal condenser where pressure is conserved (p_in = p_out).
//! Calculates heat transfer rate: `Qout = mdot * (h_in - h_out)`.
//!
//! # Energy Category
//! `"QOUT"` — aggregated into cycle-level Qout.
//!
//! # Panic Conditions
//! - `state()`: panics if both ports' p are NaN
//! - `balance()`: panics if both ports' mdot are NaN, or if either port's h is NaN

use crate::common::{CompSISO, Port, UMComponent, to_string_with_precision, any_to_string, PortDictMut, PortDict};
use std::collections::HashMap;

/// Condenser component for the vapor compression cycle.
///
/// Implements isobaric condensation: the output port's pressure equals the
/// input port's pressure. Heat transfer rate is calculated from the enthalpy
/// drop and mass flow rate.
pub struct Condenser {
    /// Component name
    pub name: String,
    /// Energy category: "QOUT"
    pub energy: String,
    /// Input port pointer
    pub i_port: *mut Port,
    /// Output port pointer
    pub o_port: *mut Port,
    /// Port dictionary: {"iPort" → ptr, "oPort" → ptr}
    pub portdict: HashMap<String, *mut Port>,
    /// Heat transfer rate (kW)
    pub qc: f64,
}

impl Condenser {
    /// Creates a new Condenser from a JSON component configuration.
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

        Condenser {
            name,
            energy: "QOUT".to_string(),
            i_port,
            o_port,
            portdict,
            qc: f64::NAN,
        }
    }
}

impl CompSISO for Condenser {
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

    /// Isobaric condensation: propagates pressure between ports.
    ///
    /// If one port has a valid pressure and the other doesn't, copies it.
    ///
    /// # Panics
    /// Panics if both ports' p are NaN (no pressure information available).
    fn state(&mut self) {
        unsafe {
            if !(*self.o_port).p.is_nan() && (*self.i_port).p.is_nan() {
                (*self.i_port).p = (*self.o_port).p;
            } else if !(*self.i_port).p.is_nan() && (*self.o_port).p.is_nan() {
                (*self.o_port).p = (*self.i_port).p;
            } else if (*self.i_port).p.is_nan() && (*self.o_port).p.is_nan() {
                panic!("Condenser: both ports p are NaN");
            }
        }
    }

    /// Mass and energy balance for condensation.
    ///
    /// - Mass: propagates mdot between ports
    /// - Energy: Qout = mdot * (h_in - h_out)
    ///
    /// # Panics
    /// - Panics if both ports' mdot are NaN
    /// - Panics if either port's h is NaN
    fn balance(&mut self) {
        unsafe {
            if (*self.i_port).mdot.is_nan() && (*self.o_port).mdot.is_nan() {
                panic!("Condenser: mdot is NaN");
            }
            if !(*self.i_port).mdot.is_nan() {
                (*self.o_port).mdot = (*self.i_port).mdot;
            } else if !(*self.o_port).mdot.is_nan() {
                (*self.i_port).mdot = (*self.o_port).mdot;
            }
            if (*self.i_port).h.is_nan() || (*self.o_port).h.is_nan() {
                panic!("Condenser: h is NaN");
            }
            self.qc = (*self.i_port).mdot * ((*self.i_port).h - (*self.o_port).h);
        }
    }

    fn resultstring(&self) -> String {
        unsafe {
            format!(
                "\n{}\n{}\n{}\n{}\nThe condenser Capacity(kW): {}\n",
                self.name,
                Port::TITLE,
                (*self.i_port).resultstring(),
                (*self.o_port).resultstring(),
                to_string_with_precision(self.qc, 3)
            )
        }
    }
}

impl PortDict for Condenser {
    fn portdict(&self) -> &HashMap<String, *mut Port> {
        &self.portdict
    }
}

impl PortDictMut for Condenser {
    fn portdict_mut(&mut self) -> &mut HashMap<String, *mut Port> {
        &mut self.portdict
    }
}
