//! Port module: thermodynamic state of the working fluid at a connection point.
//!
//! A [`Port`] stores pressure, temperature, enthalpy, entropy, quality, and mass
//! flow rate. It can calculate missing properties from known pairs using CoolProp.
//!
//! # Node Sharing
//! When two components are connected, their ports share the same `Port` memory
//! (via `Rc<RefCell<Port>>`). This ensures state changes propagate automatically
//! between connected components — a key mechanism for the component calculation
//! order detection algorithm.
//!
//! # State Calculation Methods
//! - `tx()` — from temperature and quality
//! - `px()` — from pressure and quality (or pressure and enthalpy)
//! - `ps()` — from pressure and entropy
//! - `ph()` — from pressure and enthalpy
//! - `pt()` — from pressure and temperature

use std::collections::HashMap;
use std::ffi::CString;

/// Sentinel value indicating a port has not been assigned to any node.
pub const NONE_INDEX: usize = usize::MAX;

/// Thermodynamic state of the working fluid at a connection point (port).
///
/// A port stores pressure, temperature, enthalpy, entropy, quality, and mass
/// flow rate. It can calculate missing properties from known pairs using CoolProp.
#[derive(Debug, Clone)]
pub struct Port {
    /// Port name (optional identifier)
    pub name: String,
    /// Refrigerant fluid name (e.g., "R134a")
    pub fluid_name: String,
    /// Pressure (MPa)
    pub p: f64,
    /// Temperature (°C)
    pub t: f64,
    /// Enthalpy (kJ/kg)
    pub h: f64,
    /// Entropy (kJ/kg·K)
    pub s: f64,
    /// Quality (vapor mass fraction)
    pub x: f64,
    /// Mass flow rate (kg/s)
    pub mdot: f64,
    /// Whether the port state has been fully determined
    pub state_ok: bool,
    /// Node index in the connector's node list
    pub index: usize,
}

impl Port {
    /// Creates a new Port from a dictionary of property key-value pairs.
    ///
    /// Initializes all properties to NaN, then sets provided values.
    /// If enough property pairs are given (e.g., t+x, p+x, p+t),
    /// automatically calculates the remaining properties.
    pub fn new(curm_port: &HashMap<String, f64>, fluid_name: &str) -> Self {
        let mut port = Port {
            name: "".to_string(),
            fluid_name: fluid_name.to_string(),
            p: f64::NAN,
            t: f64::NAN,
            h: f64::NAN,
            s: f64::NAN,
            x: f64::NAN,
            mdot: f64::NAN,
            state_ok: false,
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

    /// Calls CoolProp's `PropsSI` function via the `coolprop-sys` crate.
    fn prop_si(&self, output: &str, name1: &str, prop1: f64, name2: &str, prop2: f64) -> f64 {
        let c_output = CString::new(output).unwrap();
        let c_name1 = CString::new(name1).unwrap();
        let c_name2 = CString::new(name2).unwrap();
        let c_fluid = CString::new(self.fluid_name.as_str()).unwrap();

        let coolprop = coolprop_sys::COOLPROP.lock().unwrap();
        unsafe {
            (coolprop.PropsSI)(
                c_output.as_ptr(),
                c_name1.as_ptr(),
                prop1,
                c_name2.as_ptr(),
                prop2,
                c_fluid.as_ptr(),
            )
        }
    }

    /// Calculates properties from temperature and quality.
    ///
    /// Sets p, h, s from T (°C) and x. Temperature is converted to K internally.
    pub fn tx(&mut self) {
        if !self.t.is_nan() && !self.x.is_nan() {
            let t_k = self.t + 273.15;
            self.p = self.prop_si("P", "T", t_k, "Q", self.x) / 1.0e6;
            self.h = self.prop_si("H", "T", t_k, "Q", self.x) / 1000.0;
            self.s = self.prop_si("S", "T", t_k, "Q", self.x) / 1000.0;
            self.state_ok = true;
        }
    }

    /// Calculates properties from pressure and quality (or pressure and enthalpy).
    ///
    /// If x is known: sets t, h, s from P and x.
    /// If h is known (but x is not): sets t, s, x from P and H.
    pub fn px(&mut self) {
        if !self.p.is_nan() {
            let p_pa = self.p * 1.0e6;
            if !self.x.is_nan() {
                self.t = self.prop_si("T", "P", p_pa, "Q", self.x) - 273.15;
                self.h = self.prop_si("H", "P", p_pa, "Q", self.x) / 1000.0;
                self.s = self.prop_si("S", "P", p_pa, "Q", self.x) / 1000.0;
            } else if !self.h.is_nan() {
                let h_jkg = self.h * 1000.0;
                self.t = self.prop_si("T", "P", p_pa, "H", h_jkg) - 273.15;
                self.s = self.prop_si("S", "P", p_pa, "H", h_jkg) / 1000.0;
                let x_val = self.prop_si("Q", "P", p_pa, "H", h_jkg);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.state_ok = true;
        }
    }

    /// Calculates properties from pressure and entropy.
    ///
    /// Sets h, t, x from P and S.
    pub fn ps(&mut self) {
        if !self.p.is_nan() && !self.s.is_nan() {
            let p_pa = self.p * 1.0e6;
            let s_jkgk = self.s * 1000.0;
            if self.h.is_nan() {
                self.h = self.prop_si("H", "P", p_pa, "S", s_jkgk) / 1000.0;
            }
            if self.t.is_nan() {
                self.t = self.prop_si("T", "P", p_pa, "S", s_jkgk) - 273.15;
            }
            if self.x.is_nan() {
                let x_val = self.prop_si("Q", "P", p_pa, "S", s_jkgk);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.state_ok = true;
        }
    }

    /// Calculates properties from pressure and enthalpy.
    ///
    /// Sets s, t, x from P and H.
    pub fn ph(&mut self) {
        if !self.p.is_nan() && !self.h.is_nan() {
            let p_pa = self.p * 1.0e6;
            let h_jkg = self.h * 1000.0;
            if self.s.is_nan() {
                self.s = self.prop_si("S", "P", p_pa, "H", h_jkg) / 1000.0;
            }
            if self.t.is_nan() {
                self.t = self.prop_si("T", "P", p_pa, "H", h_jkg) - 273.15;
            }
            if self.x.is_nan() {
                let x_val = self.prop_si("Q", "P", p_pa, "H", h_jkg);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.state_ok = true;
        }
    }

    /// Calculates properties from pressure and temperature.
    ///
    /// Sets s, h, x from P and T.
    pub fn pt(&mut self) {
        if !self.p.is_nan() && !self.t.is_nan() {
            let p_pa = self.p * 1.0e6;
            let t_k = self.t + 273.15;
            if self.s.is_nan() {
                self.s = self.prop_si("S", "P", p_pa, "T", t_k) / 1000.0;
            }
            if self.h.is_nan() {
                self.h = self.prop_si("H", "P", p_pa, "T", t_k) / 1000.0;
            }
            if self.x.is_nan() {
                let x_val = self.prop_si("Q", "P", p_pa, "T", t_k);
                self.x = if x_val == -1.0 { f64::NAN } else { x_val };
            }
            self.state_ok = true;
        }
    }

    /// Attempts to calculate the port state from available property pairs.
    ///
    /// Tries in order: ps → ph → pt. Only called when `state_ok` is false.
    /// This is used by `component_simulator` to resolve node states after
    /// a component's `state()` call provides new property values.
    pub fn state(&mut self) {
        if !self.state_ok {
            if !self.p.is_nan() && !self.s.is_nan() {
                self.ps();
            } else if !self.p.is_nan() && !self.h.is_nan() {
                self.ph();
            } else if !self.p.is_nan() && !self.t.is_nan() {
                self.pt();
            }
        }
    }

    /// Returns a formatted string of this port's thermodynamic state.
    pub fn result_string(&self) -> String {
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

    /// Column header for port result output.
    pub const TITLE: &'static str = "Port   	P(MPa)   T(C)  H(kJ/kg)	  S(kJ/kg.K)  Quality MDOT(kg/s)";
}
