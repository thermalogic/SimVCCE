//! Common module: core data structures, traits, and utility functions.
//!
//! This module defines:
//! - [`CompSISO`] — trait interface for Single-Input Single-Output components
//! - [`PortDict`] / [`PortDictMut`] — traits for accessing component port dictionaries
//! - [`AsAny`] — trait for runtime type downcasting
//! - Utility type aliases and helper functions
//!
//! [`Port`] and [`NONE_INDEX`] are re-exported from [`crate::core::port`].

// Re-export Port and NONE_INDEX from core::port for backward compatibility
pub use crate::core::{Port, NONE_INDEX};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Shared mutable reference to a Port, used for node sharing between components.
pub type PortRef = Rc<RefCell<Port>>;

/// Trait interface for Single-Input Single-Output (SISO) refrigeration cycle components.
///
/// Each component must implement:
/// - `set_port_address()` — update port pointers after connector node sharing
/// - `state()` — thermal process calculation (panics if input data is insufficient)
/// - `balance()` — energy and mass balance (panics if input data is insufficient)
/// - `result_string()` — formatted output of component results
/// - `name()` — component name
/// - `energy()` — energy category string ("CompressionWork", "QIN", "QOUT", or "")
///
/// # Panic Convention
/// `state()` and `balance()` **must panic** when required input data is not yet
/// available (e.g., NaN values). This is not an error — it signals to the
/// `component_simulator` that this component cannot be processed yet and should
/// be retried in a later iteration.
pub trait CompSISO: PortDict + PortDictMut + AsAny {
    /// Update port pointers to match the current portdict (after node sharing).
    fn set_port_address(&mut self);
    /// Perform thermal process calculation. Panics if input data is insufficient.
    fn state(&mut self);
    /// Perform energy and mass balance. Panics if input data is insufficient.
    fn balance(&mut self);
    /// Returns a formatted string of this component's results.
    fn result_string(&self) -> String;
    /// Returns the component name.
    fn name(&self) -> &str;
    /// Returns the energy category: "CompressionWork", "QIN", "QOUT", or "".
    fn energy(&self) -> &str;
}

/// Trait for read access to a component's port dictionary.
pub trait PortDict {
    /// Returns the map of port name → shared port reference.
    fn portdict(&self) -> &HashMap<String, PortRef>;
}

/// Trait for mutable access to a component's port dictionary.
pub trait PortDictMut {
    /// Returns the mutable map of port name → shared port reference.
    fn portdict_mut(&mut self) -> &mut HashMap<String, PortRef>;
}

/// Trait for runtime type downcasting.
///
/// Enables `component_simulator` to aggregate cycle results by downcasting
/// trait objects to concrete types (e.g., `Box<dyn CompSISO>` → `Compressor`).
pub trait AsAny {
    /// Returns a reference to `dyn Any` for downcasting.
    fn as_any(&self) -> &dyn std::any::Any;
    /// Returns a mutable reference to `dyn Any` for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Component configuration dictionary type (from JSON parsing).
pub type UMComponent = HashMap<String, serde_json::Value>;
/// Port configuration dictionary type (from JSON parsing).
pub type MPort = HashMap<String, serde_json::Value>;
/// Port identifier tuple: (component_name, port_name).
pub type TupPort = (String, String);
/// Connector specification tuple: ((comp0, port0), (comp1, port1)).
pub type TupConnector = (TupPort, TupPort);

/// Formats a floating-point value with specified precision, or "--" if NaN.
pub fn to_string_with_precision(value: f64, precision: usize) -> String {
    if !value.is_nan() {
        format!("{:.1$}", value, precision)
    } else {
        " -- ".to_string()
    }
}

/// Extracts a string value from a JSON Value, returning empty string if not a string.
pub fn any_to_string(val: &serde_json::Value) -> String {
    if let Some(s) = val.as_str() {
        s.to_string()
    } else {
        "".to_string()
    }
}
