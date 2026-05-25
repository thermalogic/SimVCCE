//! Components module: refrigeration cycle component implementations.
//!
//! Each component implements the [`CompSISO`](crate::common::CompSISO) trait
//! and follows the error convention: `state()` and `balance()` return
//! `Err(SimulationError)` when required input data is not yet available,
//! signaling to the `component_simulator` that the component should be
//! retried later.
//!
//! The [`SISOComponent`] struct encapsulates the shared fields and logic
//! (port dictionary, name, energy category, port address updates, mdot
//! propagation) common to all SISO components. Individual components
//! embed this struct and only implement their specific `state()` and
//! `balance()` logic.

pub mod compressor;
pub mod condenser;
pub mod evaporator;
pub mod expansionvalve;
pub mod siso_component;

pub use compressor::Compressor;
pub use condenser::Condenser;
pub use evaporator::Evaporator;
pub use expansionvalve::ExpansionValve;
pub use siso_component::SISOComponent;
