//! Connector module: manages node sharing between component ports.
//!
//! The Connector is the key mechanism for **node sharing** — when two components
//! are connected, their ports share the same `Port` memory (raw pointer). This
//! ensures that state changes in one component's port are immediately visible
//! to the connected component's port, maintaining consistent thermodynamic state
//! across the entire cycle.
//!
//! # Node Sharing Process
//! 1. Port0 is added as a new node in the node list
//! 2. Port1's known property values are merged into the node
//! 3. Port1's pointer in the component's portdict is replaced with the node pointer
//! 4. Now both components reference the same `Port` object — any modification
//!    by one component is instantly visible to the other

use crate::common::{Port, TupConnector, NONE_INDEX};
use std::collections::HashMap;

/// Manages shared nodes between connected component ports.
///
/// Each node in the `nodes` vector is a `*mut Port` that is shared by exactly
/// two component ports. Memory ownership is managed here — the `Drop` impl
/// deallocates all node memory via `Box::from_raw`.
pub struct Connector {
    /// Current node index (used during connector construction)
    pub index: usize,
    /// Vector of shared node pointers. Each node is shared by two connected ports.
    pub nodes: Vec<*mut Port>,
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
    fn getnodevalue(&mut self, port: *mut Port) {
        unsafe {
            if (*self.nodes[self.index]).index == NONE_INDEX && (*port).index != NONE_INDEX {
                (*self.nodes[self.index]).index = (*port).index;
            }
            if (*self.nodes[self.index]).p.is_nan() && !(*port).p.is_nan() {
                (*self.nodes[self.index]).p = (*port).p;
            }
            if (*self.nodes[self.index]).t.is_nan() && !(*port).t.is_nan() {
                (*self.nodes[self.index]).t = (*port).t;
            }
            if (*self.nodes[self.index]).h.is_nan() && !(*port).h.is_nan() {
                (*self.nodes[self.index]).h = (*port).h;
            }
            if (*self.nodes[self.index]).s.is_nan() && !(*port).s.is_nan() {
                (*self.nodes[self.index]).s = (*port).s;
            }
            if (*self.nodes[self.index]).x.is_nan() && !(*port).x.is_nan() {
                (*self.nodes[self.index]).x = (*port).x;
            }
            if (*self.nodes[self.index]).mdot.is_nan() && !(*port).mdot.is_nan() {
                (*self.nodes[self.index]).mdot = (*port).mdot;
            }
        }
    }

    /// Connects two component ports by creating a shared node.
    ///
    /// # Arguments
    /// * `tconn` - Tuple specifying ((comp0, port0), (comp1, port1))
    /// * `comps` - Mutable reference to the component HashMap
    ///
    /// # Process
    /// 1. Get pointers to both ports from their respective components
    /// 2. Set port0's index to the new node index
    /// 3. Add port0's pointer as a new node
    /// 4. Merge port1's known values into the node
    /// 5. Replace port1's pointer in its component with the node pointer
    /// 6. Re-set port addresses in the component (update i_port/o_port fields)
    pub fn add_connector(
        &mut self,
        tconn: TupConnector,
        comps: &mut HashMap<String, Box<dyn crate::common::CompSISO>>,
    ) {
        let ((comp0, port0), (comp1, port1)) = tconn;

        // 1 get the index of port in Nodes
        self.index = self.nodes.len();
        unsafe {
            let port0_ptr = *comps.get_mut(&comp0).unwrap().portdict().get(&port0).unwrap();
            let port1_ptr = *comps.get_mut(&comp1).unwrap().portdict().get(&port1).unwrap();

            (*port0_ptr).index = self.index;
            // 2 init the Node[index] using the port0
            self.nodes.push(port0_ptr);
            // 3 join the port1 info to the Node[index]
            self.getnodevalue(port1_ptr);
            // 4 change the address of port1 to the Node[index]
            comps.get_mut(&comp1).unwrap().portdict_mut().insert(port1, self.nodes[self.index]);
            comps.get_mut(&comp1).unwrap().setportaddress();
        }
    }
}

impl Default for Connector {
    fn default() -> Self {
        Self::new()
    }
}

/// Deallocates all node memory.
///
/// Since the Connector owns all shared node memory (created via `Box::into_raw`),
/// it is responsible for freeing it. This is done safely by converting each
/// raw pointer back to a `Box` and letting it drop.
impl Drop for Connector {
    fn drop(&mut self) {
        unsafe {
            for &node in &self.nodes {
                let _ = Box::from_raw(node);
            }
        }
    }
}
