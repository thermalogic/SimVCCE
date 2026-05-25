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

use crate::common::{CompSISO, Port, PortRef, UMComponent, to_string_with_precision, any_to_string, PortDictMut, PortDict};
use std::collections::HashMap;
use std::rc::Rc;

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
    /// Input port reference
    pub i_port: PortRef,
    /// Output port reference
    pub o_port: PortRef,
    /// Port dictionary: {"iPort" → ref, "oPort" → ref}
    pub portdict: HashMap<String, PortRef>,
    /// Refrigeration capacity (kW)
    pub qe: f64,
}

impl Evaporator {
    /// Creates a new Evaporator from a JSON component configuration.
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

        let i_port = Rc::new(std::cell::RefCell::new(Port::new(&i_port_data, fluid_name)));
        let o_port = Rc::new(std::cell::RefCell::new(Port::new(&o_port_data, fluid_name)));

        let mut portdict = HashMap::new();
        portdict.insert("iPort".to_string(), i_port.clone());
        portdict.insert("oPort".to_string(), o_port.clone());

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

    fn set_port_address(&mut self) {
        let i_port = self.portdict.get("iPort").unwrap();
        if !Rc::ptr_eq(&self.i_port, i_port) {
            self.i_port = i_port.clone();
        }
        let o_port = self.portdict.get("oPort").unwrap();
        if !Rc::ptr_eq(&self.o_port, o_port) {
            self.o_port = o_port.clone();
        }
    }

    /// Isobaric evaporation: propagates pressure between ports.
    ///
    /// If one port has a valid pressure and the other doesn't, copies it.
    ///
    /// # Panics
    /// Panics if both ports' p are NaN (no pressure information available).
    fn state(&mut self) {
        let i_p = self.i_port.borrow().p;
        let o_p = self.o_port.borrow().p;
        if !o_p.is_nan() && i_p.is_nan() {
            self.i_port.borrow_mut().p = o_p;
        } else if !i_p.is_nan() && o_p.is_nan() {
            self.o_port.borrow_mut().p = i_p;
        } else if i_p.is_nan() && o_p.is_nan() {
            panic!("Evaporator: both ports p are NaN");
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
        let i_mdot = self.i_port.borrow().mdot;
        let o_mdot = self.o_port.borrow().mdot;
        if i_mdot.is_nan() && o_mdot.is_nan() {
            panic!("Evaporator: mdot is NaN");
        }
        if !i_mdot.is_nan() {
            self.o_port.borrow_mut().mdot = i_mdot;
        } else if !o_mdot.is_nan() {
            self.i_port.borrow_mut().mdot = o_mdot;
        }
        let i_h = self.i_port.borrow().h;
        let o_h = self.o_port.borrow().h;
        if i_h.is_nan() || o_h.is_nan() {
            panic!("Evaporator: h is NaN");
        }
        let mdot = self.i_port.borrow().mdot;
        self.qe = mdot * (o_h - i_h);
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\n{}\n{}\nThe Refrigeration Capacity(kW): {}\n",
            self.name,
            Port::TITLE,
            self.i_port.borrow().result_string(),
            self.o_port.borrow().result_string(),
            to_string_with_precision(self.qe, 3)
        )
    }
}

impl PortDict for Evaporator {
    fn portdict(&self) -> &HashMap<String, PortRef> {
        &self.portdict
    }
}

impl PortDictMut for Evaporator {
    fn portdict_mut(&mut self) -> &mut HashMap<String, PortRef> {
        &mut self.portdict
    }
}
