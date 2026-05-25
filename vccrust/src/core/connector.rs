//! Connector module: manages node sharing between component ports.
//!
//! The Connector is the key mechanism for **node sharing** — when two components
//! are connected, their ports share the same `Port` memory (via `Rc<RefCell<Port>>`).
//! This ensures that state changes in one component's port are immediately visible
//! to the connected component's port, maintaining consistent thermodynamic state
//! across the entire cycle.
//!
//! # Node Sharing Process
//! 1. Port0 is added as a new node in the node list
//! 2. Port1's known property values are merged into the node
//! 3. Port1's reference in the component's portdict is replaced with the node reference
//! 4. Now both components reference the same `Port` object — any modification
//!    by one component is instantly visible to the other

use crate::common::{PortRef, TupConnector, NONE_INDEX};
use std::collections::HashMap;

/// Manages shared nodes between connected component ports.
///
/// Each node in the `nodes` vector is a `Rc<RefCell<Port>>` that is shared by
/// exactly two component ports. Memory is managed automatically by `Rc` reference
/// counting — when all references are dropped, the memory is freed.
pub struct Connector {
    /// Current node index (used during connector construction)
    pub index: usize,
    /// Vector of shared node references. Each node is shared by two connected ports.
    pub nodes: Vec<PortRef>,
}

impl Connector {
    /// Creates a new empty Connector.
    pub fn new() -> Self {
        Connector {
            index: 0,
            nodes: Vec::new(),
        }
    }

    /// Merges property values from a port into the current node.
    ///
    /// For each property (p, t, h, s, x, mdot), if the node's value is NaN
    /// and the port's value is not, the port's value is copied to the node.
    fn get_node_value(&mut self, port: &PortRef) {
        let port_data = port.borrow();
        let mut node = self.nodes[self.index].borrow_mut();

        if node.index == NONE_INDEX && port_data.index != NONE_INDEX {
            node.index = port_data.index;
        }
        if node.p.is_nan() && !port_data.p.is_nan() {
            node.p = port_data.p;
        }
        if node.t.is_nan() && !port_data.t.is_nan() {
            node.t = port_data.t;
        }
        if node.h.is_nan() && !port_data.h.is_nan() {
            node.h = port_data.h;
        }
        if node.s.is_nan() && !port_data.s.is_nan() {
            node.s = port_data.s;
        }
        if node.x.is_nan() && !port_data.x.is_nan() {
            node.x = port_data.x;
        }
        if node.mdot.is_nan() && !port_data.mdot.is_nan() {
            node.mdot = port_data.mdot;
        }
    }

    /// Connects two component ports by creating a shared node.
    ///
    /// # Arguments
    /// * `tconn` - Tuple specifying ((comp0, port0), (comp1, port1))
    /// * `comps` - Mutable reference to the component HashMap
    ///
    /// # Process
    /// 1. Get references to both ports from their respective components
    /// 2. Set port0's index to the new node index
    /// 3. Add port0's reference as a new node
    /// 4. Merge port1's known values into the node
    /// 5. Replace port1's reference in its component with the node reference
    /// 6. Re-set port addresses in the component (update i_port/o_port fields)
    pub fn add_connector(
        &mut self,
        tconn: TupConnector,
        comps: &mut HashMap<String, Box<dyn crate::common::CompSISO>>,
    ) {
        let ((comp0, port0), (comp1, port1)) = tconn;

        // 1 get the index of port in Nodes
        self.index = self.nodes.len();

        let port0_ref = comps.get_mut(&comp0).unwrap().portdict().get(&port0).unwrap().clone();
        let port1_ref = comps.get_mut(&comp1).unwrap().portdict().get(&port1).unwrap().clone();

        port0_ref.borrow_mut().index = self.index;
        // 2 init the Node[index] using the port0
        self.nodes.push(port0_ref);
        // 3 join the port1 info to the Node[index]
        self.get_node_value(&port1_ref);
        // 4 change the address of port1 to the Node[index]
        comps.get_mut(&comp1).unwrap().portdict_mut().insert(port1, self.nodes[self.index].clone());
        comps.get_mut(&comp1).unwrap().set_port_address();
    }
}

impl Default for Connector {
    fn default() -> Self {
        Self::new()
    }
}
