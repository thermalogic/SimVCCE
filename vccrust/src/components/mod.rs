//! Components module: refrigeration cycle component implementations.
//!
//! Each component implements the [`CompSISO`](crate::common::CompSISO) trait
//! and follows the error convention: `state()` and `balance()` return
//! `Err(SimulationError)` when required input data is not yet available,
//! signaling to the `component_simulator` that the component should be
//! retried later.

pub mod compressor;
pub mod condenser;
pub mod evaporator;
pub mod expansionvalve;

pub use compressor::Compressor;
pub use condenser::Condenser;
pub use evaporator::Evaporator;
pub use expansionvalve::ExpansionValve;
