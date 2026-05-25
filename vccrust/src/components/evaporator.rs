//! Evaporator component: isobaric evaporation process.
//!
//! Models an ideal evaporator where pressure is conserved (p_in = p_out).
//! Calculates refrigeration capacity: `Qin = mdot * (h_out - h_in)`.
//!
//! # Energy Category
//! `"QIN"` — aggregated into cycle-level Qin.
//!
//! # Error Conditions
//! - `state()`: returns `Err` if both ports' p are NaN
//! - `balance()`: returns `Err` if both ports' mdot are NaN, or if either port's h is NaN

use crate::common::{CompSISO, PortDict, PortDictMut, SimulationError, UMComponent, to_string_with_precision};
use crate::components::siso_component::SISOComponent;

/// Evaporator component for the vapor compression cycle.
///
/// Implements isobaric evaporation: the output port's pressure equals the
/// input port's pressure. Refrigeration capacity is calculated from the
/// enthalpy increase and mass flow rate.
pub struct Evaporator {
    /// Shared SISO component fields and logic
    pub inner: SISOComponent,
    /// Refrigeration capacity (kW)
    pub qe: f64,
}

impl Evaporator {
    /// Creates a new Evaporator from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        Evaporator {
            inner: SISOComponent::new(dict_comp, fluid_name, "Evaporator", "QIN"),
            qe: 0.0,
        }
    }
}

impl CompSISO for Evaporator {
    fn name(&self) -> &str {
        &self.inner.name
    }

    fn energy(&self) -> &str {
        &self.inner.energy
    }

    fn energy_value(&self) -> f64 {
        self.qe
    }

    fn set_port_address(&mut self) {
        self.inner.set_port_address();
    }

    /// Isobaric evaporation: propagates pressure between ports.
    ///
    /// If one port has a valid pressure and the other doesn't, copies it.
    ///
    /// # Errors
    /// Returns `Err` if both ports' p are NaN (no pressure information available).
    fn state(&mut self) -> Result<(), SimulationError> {
        let i_p = self.inner.i_port.borrow().p;
        let o_p = self.inner.o_port.borrow().p;
        if !o_p.is_nan() && i_p.is_nan() {
            self.inner.i_port.borrow_mut().p = o_p;
        } else if !i_p.is_nan() && o_p.is_nan() {
            self.inner.o_port.borrow_mut().p = i_p;
        } else if i_p.is_nan() && o_p.is_nan() {
            return Err(SimulationError::new("Evaporator: both ports p are NaN"));
        }
        Ok(())
    }

    /// Mass and energy balance for evaporation.
    ///
    /// - Mass: propagates mdot between ports
    /// - Energy: Qin = mdot * (h_out - h_in)
    ///
    /// # Errors
    /// - Returns `Err` if both ports' mdot are NaN
    /// - Returns `Err` if either port's h is NaN
    fn balance(&mut self) -> Result<(), SimulationError> {
        self.inner.propagate_mdot("Evaporator")?;
        let (i_h, o_h) = self.inner.get_enthalpies("Evaporator")?;
        self.qe = self.inner.mdot() * (o_h - i_h);
        Ok(())
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\nThe Refrigeration Capacity(kW): {}\n",
            self.inner.name,
            self.inner.port_result_string(),
            to_string_with_precision(self.qe, 3)
        )
    }
}

impl PortDict for Evaporator {
    fn portdict(&self) -> &std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict()
    }
}

impl PortDictMut for Evaporator {
    fn portdict_mut(&mut self) -> &mut std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict_mut()
    }
}
