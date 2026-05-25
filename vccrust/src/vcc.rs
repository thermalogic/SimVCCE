//! VCCycle - Main vapor compression cycle module

use crate::common::{CompSISO, TupConnector, UMComponent, to_string_with_precision, Port};
use crate::core::Connector;
use crate::components::{Compressor, Condenser, Evaporator, ExpansionValve};
use std::collections::HashMap;

pub struct VCCycle {
    pub curcon: Connector,
    pub comps: HashMap<String, Box<dyn CompSISO>>,
    pub comp_order: Vec<String>,
    pub wc: f64,
    pub qin: f64,
    pub cop: f64,
}

impl VCCycle {
    pub fn new(dict_comps: Vec<UMComponent>, vec_connectors: Vec<TupConnector>, comp_order: Vec<String>) -> Self {
        let mut comps = HashMap::new();

        for item in dict_comps {
            let class_str = item.get("classstr")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let name = item.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("").to_string();

            match class_str {
                "Compressor" => {
                    comps.insert(name, Box::new(Compressor::new(&item)) as Box<dyn CompSISO>);
                }
                "Condenser" => {
                    comps.insert(name, Box::new(Condenser::new(&item)) as Box<dyn CompSISO>);
                }
                "Evaporator" => {
                    comps.insert(name, Box::new(Evaporator::new(&item)) as Box<dyn CompSISO>);
                }
                "ExpansionValve" => {
                    comps.insert(name, Box::new(ExpansionValve::new(&item)) as Box<dyn CompSISO>);
                }
                _ => {}
            }
        }

        let mut curcon = Connector::new();
        for tconn in vec_connectors {
            curcon.add_connector(tconn, &mut comps);
        }

        VCCycle {
            curcon,
            comps,
            comp_order,
            wc: 0.0,
            qin: 0.0,
            cop: 0.0,
        }
    }

    pub fn state(&mut self) {
        // 1 state by process - in comp order
        for name in &self.comp_order {
            if let Some(comp) = self.comps.get_mut(name) {
                comp.state();
            }
        }
        // 2 state of port/node
        for &node in &self.curcon.nodes {
            unsafe {
                (*node).state();
            }
        }
    }

    pub fn balance(&mut self) {
        // In comp order
        for name in &self.comp_order {
            if let Some(comp) = self.comps.get_mut(name) {
                comp.balance();
                if comp.energy() == "CompressorWork" {
                    if let Some(compressor) = comp.as_any().downcast_ref::<Compressor>() {
                        self.wc = compressor.wc;
                    }
                } else if comp.energy() == "RefrigerationCapacity" {
                    if let Some(evaporator) = comp.as_any().downcast_ref::<Evaporator>() {
                        self.qin = evaporator.qe;
                    }
                }
            }
        }
        self.cop = self.qin / self.wc;
    }

    pub fn resultstr(&self) -> String {
        format!(
            "\n --- The Cycle --- \
             \n\tCompression Work(kW): {} \
             \n\tRefrigeration Capacity(kW): {} \
             \n\tThe coefficient of performance: {} \n",
            to_string_with_precision(self.wc, 3),
            to_string_with_precision(self.qin, 3),
            to_string_with_precision(self.cop, 3)
        )
    }

    pub fn outdevresultstr(&self) {
        // 按C++的顺序输出：Evaporator, ExpansionValve, Condenser, Compressor
        let order = ["Evaporator", "ExpansionValve", "Condenser", "Compressor"];
        for name in order {
            if let Some(comp) = self.comps.get(name) {
                println!("{}", comp.resultstring());
            }
        }
        println!("\n{}", Port::TITLE);
        for &node in &self.curcon.nodes {
            unsafe {
                println!("{}", (*node).resultstring());
            }
        }
    }

    pub fn outresults(&self) {
        println!("{}", self.resultstr());
        self.outdevresultstr();
    }
}
