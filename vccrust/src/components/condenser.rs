//! Condenser component: isobaric condensation process.
//!
//! Models an ideal condenser where pressure is conserved (p_in = p_out).
//! Calculates heat transfer rate: `Qout = mdot * (h_in - h_out)`.
//!
//! # Energy Category
//! `"QOUT"` — aggregated into cycle-level Qout.
//!
//! # Error Conditions
//! - `state()`: returns `Err` if both ports' p are NaN
//! - `balance()`: returns `Err` if both ports' mdot are NaN, or if either port's h is NaN

use crate::common::{CompSISO, PortDict, PortDictMut, SimulationError, UMComponent, to_string_with_precision};
use crate::components::siso_component::SISOComponent;

/// Condenser component for the vapor compression cycle.
///
/// Implements isobaric condensation: the output port's pressure equals the
/// input port's pressure. Heat transfer rate is calculated from the enthalpy
/// drop and mass flow rate.
pub struct Condenser {
    /// Shared SISO component fields and logic
    pub inner: SISOComponent,
    /// Heat transfer rate (kW)
    pub qc: f64,
}

impl Condenser {
    /// Creates a new Condenser from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        Condenser {
            inner: SISOComponent::new(dict_comp, fluid_name, "Condenser", "QOUT"),
            qc: 0.0,
        }
    }
}

impl CompSISO for Condenser {
    fn name(&self) -> &str {
        &self.inner.name
    }

    fn energy(&self) -> &str {
        &self.inner.energy
    }

    fn energy_value(&self) -> f64 {
        self.qc
    }

    fn set_port_address(&mut self) {
        self.inner.set_port_address();
    }

    /// Isobaric condensation: propagates pressure between ports.
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
            return Err(SimulationError::new("Condenser: both ports p are NaN"));
        }
        Ok(())
    }

    /// Mass and energy balance for condensation.
    ///
    /// - Mass: propagates mdot between ports
    /// - Energy: Qout = mdot * (h_in - h_out)
    ///
    /// # Errors
    /// - Returns `Err` if both ports' mdot are NaN
    /// - Returns `Err` if either port's h is NaN
    fn balance(&mut self) -> Result<(), SimulationError> {
        self.inner.propagate_mdot("Condenser")?;
        let (i_h, o_h) = self.inner.get_enthalpies("Condenser")?;
        self.qc = self.inner.mdot() * (i_h - o_h);
        Ok(())
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\nThe condenser Capacity(kW): {}\n",
            self.inner.name,
            self.inner.port_result_string(),
            to_string_with_precision(self.qc, 3)
        )
    }
}

impl PortDict for Condenser {
    fn portdict(&self) -> &std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict()
    }
}

impl PortDictMut for Condenser {
    fn portdict_mut(&mut self) -> &mut std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict_mut()
    }
}
