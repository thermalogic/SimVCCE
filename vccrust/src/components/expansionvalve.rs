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

use crate::common::{CompSISO, PortDict, PortDictMut, SimulationError, UMComponent};
use crate::components::siso_component::SISOComponent;

/// Expansion valve component for the vapor compression cycle.
///
/// Implements isenthalpic throttling: the output port's enthalpy equals the
/// input port's enthalpy. Only mass balance is performed (no energy calculation).
pub struct ExpansionValve {
    /// Shared SISO component fields and logic
    pub inner: SISOComponent,
}

impl ExpansionValve {
    /// Creates a new ExpansionValve from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        ExpansionValve {
            inner: SISOComponent::new(dict_comp, fluid_name, "ExpansionValve", ""),
        }
    }
}

impl CompSISO for ExpansionValve {
    fn name(&self) -> &str {
        &self.inner.name
    }

    fn energy(&self) -> &str {
        &self.inner.energy
    }

    fn energy_value(&self) -> f64 {
        0.0
    }

    fn set_port_address(&mut self) {
        self.inner.set_port_address();
    }

    /// Isenthalpic throttling: propagates enthalpy between ports.
    ///
    /// If one port has a valid enthalpy and the other doesn't, copies it.
    ///
    /// # Errors
    /// Returns `Err` if both ports' h are NaN (no enthalpy information available).
    fn state(&mut self) -> Result<(), SimulationError> {
        let i_h = self.inner.i_port.borrow().h;
        let o_h = self.inner.o_port.borrow().h;
        if !i_h.is_nan() && o_h.is_nan() {
            self.inner.o_port.borrow_mut().h = i_h;
        } else if !o_h.is_nan() && i_h.is_nan() {
            self.inner.i_port.borrow_mut().h = o_h;
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
        self.inner.propagate_mdot("ExpansionValve")?;
        Ok(())
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\n",
            self.inner.name,
            self.inner.port_result_string()
        )
    }
}

impl PortDict for ExpansionValve {
    fn portdict(&self) -> &std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict()
    }
}

impl PortDictMut for ExpansionValve {
    fn portdict_mut(&mut self) -> &mut std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict_mut()
    }
}
