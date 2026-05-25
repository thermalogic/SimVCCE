//! Condenser component

use crate::common::{CompSISO, Port, UMComponent, to_string_with_precision, any_to_string, PortDictMut, PortDict};
use std::collections::HashMap;

pub struct Condenser {
    pub name: String,
    pub energy: String,
    pub i_port: *mut Port,
    pub o_port: *mut Port,
    pub portdict: HashMap<String, *mut Port>,
    pub qc: f64,
}

impl Condenser {
    pub fn new(dict_comp: &UMComponent) -> Self {
        let name = any_to_string(dict_comp.get("name").unwrap());
        
        // Parse iPort
        let i_port_data: HashMap<String, f64> = dict_comp
            .get("iPort")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
                    .collect()
            })
            .unwrap_or_default();
        
        // Parse oPort
        let o_port_data: HashMap<String, f64> = dict_comp
            .get("oPort")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
                    .collect()
            })
            .unwrap_or_default();

        let i_port = Box::into_raw(Box::new(Port::new(&i_port_data)));
        let o_port = Box::into_raw(Box::new(Port::new(&o_port_data)));

        let mut portdict = HashMap::new();
        portdict.insert("iPort".to_string(), i_port);
        portdict.insert("oPort".to_string(), o_port);

        Condenser {
            name,
            energy: "CondenserCapacity".to_string(),
            i_port,
            o_port,
            portdict,
            qc: 0.0,
        }
    }
}

impl CompSISO for Condenser {
    fn name(&self) -> &str {
        &self.name
    }

    fn energy(&self) -> &str {
        &self.energy
    }

    fn setportaddress(&mut self) {
        if self.i_port != *self.portdict.get("iPort").unwrap() {
            self.i_port = *self.portdict.get("iPort").unwrap();
        }
        if self.o_port != *self.portdict.get("oPort").unwrap() {
            self.o_port = *self.portdict.get("oPort").unwrap();
        }
    }

    fn state(&mut self) {
        unsafe {
            (*self.i_port).p = (*self.o_port).p;
        }
    }

    fn balance(&mut self) {
        unsafe {
            if !(*self.i_port).mdot.is_nan() {
                (*self.o_port).mdot = (*self.i_port).mdot;
            } else if !(*self.o_port).mdot.is_nan() {
                (*self.i_port).mdot = (*self.o_port).mdot;
            }
            self.qc = (*self.i_port).mdot * ((*self.i_port).h - (*self.o_port).h);
        }
    }

    fn resultstring(&self) -> String {
        unsafe {
            format!(
                "\n{}\n{}\n{}\n{}\nThe condenser Capacity(kW): {}\n",
                self.name,
                Port::TITLE,
                (*self.i_port).resultstring(),
                (*self.o_port).resultstring(),
                to_string_with_precision(self.qc, 3)
            )
        }
    }
}

impl PortDict for Condenser {
    fn portdict(&self) -> &HashMap<String, *mut Port> {
        &self.portdict
    }
}

impl PortDictMut for Condenser {
    fn portdict_mut(&mut self) -> &mut HashMap<String, *mut Port> {
        &mut self.portdict
    }
}


