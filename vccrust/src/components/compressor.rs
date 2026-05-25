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

use crate::common::{CompSISO, PortDict, PortDictMut, SimulationError, UMComponent, to_string_with_precision};
use crate::components::siso_component::SISOComponent;

/// Compressor component for the vapor compression cycle.
///
/// Implements isentropic compression: the output port's entropy equals the
/// input port's entropy. Compression work is calculated from the enthalpy
/// difference and mass flow rate.
pub struct Compressor {
    /// Shared SISO component fields and logic
    pub inner: SISOComponent,
    /// Compression work (kW)
    pub wc: f64,
}

impl Compressor {
    /// Creates a new Compressor from a JSON component configuration.
    pub fn new(dict_comp: &UMComponent, fluid_name: &str) -> Self {
        Compressor {
            inner: SISOComponent::new(dict_comp, fluid_name, "Compressor", "CompressionWork"),
            wc: 0.0,
        }
    }
}

impl CompSISO for Compressor {
    fn name(&self) -> &str {
        &self.inner.name
    }

    fn energy(&self) -> &str {
        &self.inner.energy
    }

    fn energy_value(&self) -> f64 {
        self.wc
    }

    fn set_port_address(&mut self) {
        self.inner.set_port_address();
    }

    /// Isentropic compression: sets oPort.s = iPort.s.
    ///
    /// # Errors
    /// Returns `Err` if iPort.s is NaN (input entropy not yet determined).
    fn state(&mut self) -> Result<(), SimulationError> {
        let i_s = self.inner.i_port.borrow().s;
        if i_s.is_nan() {
            return Err(SimulationError::new("Compressor: iPort.s is NaN"));
        }
        self.inner.o_port.borrow_mut().s = i_s;
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
        self.inner.propagate_mdot("Compressor")?;
        let (i_h, o_h) = self.inner.get_enthalpies("Compressor")?;
        self.wc = self.inner.mdot() * (o_h - i_h);
        Ok(())
    }

    fn result_string(&self) -> String {
        format!(
            "\n{}\n{}\nThe compressor Work(kW): {}\n",
            self.inner.name,
            self.inner.port_result_string(),
            to_string_with_precision(self.wc, 3)
        )
    }
}

impl PortDict for Compressor {
    fn portdict(&self) -> &std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict()
    }
}

impl PortDictMut for Compressor {
    fn portdict_mut(&mut self) -> &mut std::collections::HashMap<String, crate::common::PortRef> {
        self.inner.portdict_mut()
    }
}
