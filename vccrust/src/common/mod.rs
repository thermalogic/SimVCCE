use std::ffi::{CString};
use std::os::raw::c_char;
use std::collections::{HashMap};

#[link(name = "CoolProp", kind = "dylib")]
extern "system" {
    fn PropsSI(
        output: *const c_char,
        name1: *const c_char,
        prop1: f64,
        name2: *const c_char,
        prop2: f64,
        fluid: *const c_char,
    ) -> f64;
}

pub const NONE_INDEX: usize = usize::MAX;

#[derive(Debug, Clone)]
pub struct Port {
    pub name: String,
    pub fluid_name: String,
    pub p: f64,
    pub t: f64,
    pub h: f64,
    pub s: f64,
    pub x: f64,
    pub mdot: f64,
    pub stateok: bool,
    pub index: usize,
}

impl Port {
    pub fn new(curm_port: &HashMap<String, f64>) -> Self {
        let mut port = Port {
            name: "".to_string(),
            fluid_name: "R134a".to_string(),
            p: f64::NAN,
            t: f64::NAN,
            h: f64::NAN,
            s: f64::NAN,
            x: f64::NAN,
            mdot: f64::NAN,
            stateok: false,
            index: NONE_INDEX,
        };

        if let Some(&p_val) = curm_port.get("p") {
            port.p = p_val;
        }
        if let Some(&t_val) = curm_port.get("t") {
            port.t = t_val;
        }
        if let Some(&x_val) = curm_port.get("x") {
            port.x = x_val;
        }
        if let Some(&mdot_val) = curm_port.get("mdot") {
            port.mdot = mdot_val;
        }
        if let Some(&h_val) = curm_port.get("h") {
            port.h = h_val;
        }
        if let Some(&s_val) = curm_port.get("s") {
            port.s = s_val;
        }

        if !port.t.is_nan() && !port.x.is_nan() {
            port.tx();
        } else if !port.p.is_nan() && !port.x.is_nan() {
            port.px();
        } else if !port.p.is_nan() && !port.t.is_nan() {
            port.pt();
        }

        port
    }

    fn propssi(&self, output: &str, name1: &str, prop1: f64, name2: &str, prop2: f64) -> f64 {
        let c_output = CString::new(output).unwrap();
        let c_name1 = CString::new(name1).unwrap();
        let c_name2 = CString::new(name2).unwrap();
        let c_fluid = CString::new(self.fluid_name.as_str()).unwrap();

        unsafe {
            PropsSI(
                c_output.as_ptr(),
                c_name1.as_ptr(),
                prop1,
                c_name2.as_ptr(),
                prop2,
                c_fluid.as_ptr(),
            )
        }
    }

    pub fn tx(&mut self) {
        if !self.t.is_nan() && !self.x.is_nan() {
            let t_k = self.t + 273.15;
            self.p = self.propssi("P", "T", t_k, "Q", self.x) / 1.0e6;
            self.h = self.propssi("H", "T", t_k, "Q", self.x) / 1000.0;
            self.s = self.propssi("S", "T", t_k, "Q", self.x) / 1000.0;
            self.stateok = true;
        }
    }

    pub fn px(&mut self) {
        if !self.p.is_nan() {
            let p_pa = self.p * 1.0e6;
            if !self.x.is_nan() {
                self.t = self.propssi("T", "P", p_pa, "Q", self.x) - 273.15;
                self.h = self.propssi("H", "P", p_pa, "Q", self.x) / 1000.0;
                self.s = self.propssi("S", "P", p_pa, "Q", self.x) / 1000.0;
            } else if !self.h.is_nan() {
                let h_jkg = self.h * 1000.0;
                self.t = self.propssi("T", "P", p_pa, "H", h_jkg) - 273.15;
                self.s = self.propssi("S", "P", p_pa, "H", h_jkg) / 1000.0;
                let x_val = self.propssi("Q", "P", p_pa, "H", h_jkg);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.stateok = true;
        }
    }

    pub fn ps(&mut self) {
        if !self.p.is_nan() && !self.s.is_nan() {
            let p_pa = self.p * 1.0e6;
            let s_jkgk = self.s * 1000.0;
            if self.h.is_nan() {
                self.h = self.propssi("H", "P", p_pa, "S", s_jkgk) / 1000.0;
            }
            if self.t.is_nan() {
                self.t = self.propssi("T", "P", p_pa, "S", s_jkgk) - 273.15;
            }
            if self.x.is_nan() {
                let x_val = self.propssi("Q", "P", p_pa, "S", s_jkgk);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.stateok = true;
        }
    }

    pub fn ph(&mut self) {
        if !self.p.is_nan() && !self.h.is_nan() {
            let p_pa = self.p * 1.0e6;
            let h_jkg = self.h * 1000.0;
            if self.s.is_nan() {
                self.s = self.propssi("S", "P", p_pa, "H", h_jkg) / 1000.0;
            }
            if self.t.is_nan() {
                self.t = self.propssi("T", "P", p_pa, "H", h_jkg) - 273.15;
            }
            if self.x.is_nan() {
                let x_val = self.propssi("Q", "P", p_pa, "H", h_jkg);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.stateok = true;
        }
    }

    pub fn pt(&mut self) {
        if !self.p.is_nan() && !self.t.is_nan() {
            let p_pa = self.p * 1.0e6;
            let t_k = self.t + 273.15;
            if self.s.is_nan() {
                self.s = self.propssi("S", "P", p_pa, "T", t_k) / 1000.0;
            }
            if self.h.is_nan() {
                self.h = self.propssi("H", "P", p_pa, "T", t_k) / 1000.0;
            }
            if self.x.is_nan() {
                let x_val = self.propssi("Q", "P", p_pa, "T", t_k);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.stateok = true;
        }
    }

    pub fn state(&mut self) {
        if !self.stateok {
            if !self.p.is_nan() && !self.s.is_nan() {
                self.ps();
            } else if !self.p.is_nan() && !self.h.is_nan() {
                self.ph();
            } else if !self.p.is_nan() && !self.t.is_nan() {
                self.pt();
            }
        }
    }

    pub fn resultstring(&self) -> String {
        let x_str = if self.x.is_nan() {
            "   --".to_string()
        } else {
            format!("{:6.3}", self.x)
        };
        format!(
            "{}\t{:7.3}\t{:7.3}\t{:8.3}\t   {:7.3}\t{}\t{:7.3}",
            self.index, self.p, self.t, self.h, self.s, x_str, self.mdot
        )
    }

    pub const TITLE: &'static str = "Port   	P(MPa)   T(C)  H(kJ/kg)	  S(kJ/kg.K)  Quality MDOT(kg/s)";
}

pub trait CompSISO: PortDict + PortDictMut + AsAny {
    fn setportaddress(&mut self);
    fn state(&mut self);
    fn balance(&mut self);
    fn resultstring(&self) -> String;
    fn name(&self) -> &str;
    fn energy(&self) -> &str;
}

pub trait PortDict {
    fn portdict(&self) -> &HashMap<String, *mut Port>;
}

pub trait PortDictMut {
    fn portdict_mut(&mut self) -> &mut HashMap<String, *mut Port>;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
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

pub type UMComponent = HashMap<String, serde_json::Value>;
pub type MPort = HashMap<String, serde_json::Value>;
pub type TupPort = (String, String);
pub type TupConnector = (TupPort, TupPort);

pub fn to_string_with_precision(value: f64, precision: usize) -> String {
    if !value.is_nan() {
        format!("{:.1$}", value, precision)
    } else {
        " -- ".to_string()
    }
}

pub fn copy_string(s: &str) -> String {
    s.to_string()
}

pub fn any_to_string(val: &serde_json::Value) -> String {
    if let Some(s) = val.as_str() {
        s.to_string()
    } else {
        "".to_string()
    }
}
