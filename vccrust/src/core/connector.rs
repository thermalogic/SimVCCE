//! Connector module

use crate::common::{Port, TupConnector, NONE_INDEX};
use std::collections::HashMap;

pub struct Connector {
    pub index: usize,
    pub nodes: Vec<*mut Port>,
}

impl Connector {
    pub fn new() -> Self {
        Connector {
            index: 0,
            nodes: Vec::new(),
        }
    }

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

impl Drop for Connector {
    fn drop(&mut self) {
        unsafe {
            for &node in &self.nodes {
                let _ = Box::from_raw(node);
            }
        }
    }
}
