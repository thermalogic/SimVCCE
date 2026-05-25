//! Core modules: fundamental data structures for the simulation.
//!
//! - [`port`] — [`Port`] thermodynamic state at a connection point
//! - [`connector`] — [`Connector`] manages node sharing between component ports via `Rc<RefCell<Port>>`

pub mod port;
pub mod connector;

pub use port::{Port, NONE_INDEX};
pub use connector::Connector;
