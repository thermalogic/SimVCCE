//! Evaporator component: isobaric evaporation process.
//!
//! Models an ideal evaporator where pressure is conserved (p_in = p_out).
//! Calculates refrigeration capacity: `Qin = mdot * (h_out - h_in)`.
//!
//! # Energy Category
//! `"QIN"` — aggregated into cycle-level Qin.
//!
//! # Panic Conditions
//! - `state()`: panics if both ports' p are NaN
//! - `balance()`: panics if both ports' mdot are NaN, or if either port's h is NaN

use crate::common::{CompSISO, Port, UMComponent, to_string_with_precision, any_to_string, PortDictMut, PortDict};
use std::collections::HashMap;

/// Evaporator component for the vapor compression cycle.
///
/// Implements isobaric evaporation: the output port's pressure equals the
/// input port's pressure. Refrigeration capacity is calculated from the
/// enthalpy increase and mass flow rate.
pub struct Evaporator {
    /// Component name
    pub name: String,
    /// Energy category: "QIN"
    pub energy: String,
    /// Input port pointer
    pub i_port: *mut Port,
    /// Output port pointer
    pub o_port: *mut Port,
    /// Port dictionary: {"iPort" → ptr, "oPort" → ptr}
    pub portdict: HashMap<String, *mut Port>,
    /// Refrigeration capacity (kW)
    pub qe: f64,
}

impl Evaporator {
    /// Creates a new Evaporator from a JSON component configuration.
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

        Evaporator {
            name,
            energy: "QIN".to_string(),
            i_port,
            o_port,
            portdict,
            qe: 0.0,
        }
    }
}

impl CompSISO for Evaporator {
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

    /// Isobaric evaporation: propagates pressure between ports.
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
                panic!("Evaporator: both ports p are NaN");
            }
        }
    }

    /// Mass and energy balance for evaporation.
    ///
    /// - Mass: propagates mdot between ports
    /// - Energy: Qin = mdot * (h_out - h_in)
    ///
    /// # Panics
    /// - Panics if both ports' mdot are NaN
    /// - Panics if either port's h is NaN
    fn balance(&mut self) {
        unsafe {
            if (*self.i_port).mdot.is_nan() && (*self.o_port).mdot.is_nan() {
                panic!("Evaporator: mdot is NaN");
            }
            if !(*self.i_port).mdot.is_nan() {
                (*self.o_port).mdot = (*self.i_port).mdot;
            } else if !(*self.o_port).mdot.is_nan() {
                (*self.i_port).mdot = (*self.o_port).mdot;
            }
            if (*self.i_port).h.is_nan() || (*self.o_port).h.is_nan() {
                panic!("Evaporator: h is NaN");
            }
            self.qe = (*self.i_port).mdot * ((*self.o_port).h - (*self.i_port).h);
        }
    }

    fn resultstring(&self) -> String {
        unsafe {
            format!(
                "\n{}\n{}\n{}\n{}\nThe Refrigeration Capacity(kW): {}\n",
                self.name,
                Port::TITLE,
                (*self.i_port).resultstring(),
                (*self.o_port).resultstring(),
                to_string_with_precision(self.qe, 3)
            )
        }
    }
}

impl PortDict for Evaporator {
    fn portdict(&self) -> &HashMap<String, *mut Port> {
        &self.portdict
    }
}

impl PortDictMut for Evaporator {
    fn portdict_mut(&mut self) -> &mut HashMap<String, *mut Port> {
        &mut self.portdict
    }
}
