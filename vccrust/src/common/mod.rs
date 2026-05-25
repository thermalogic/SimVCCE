//! Common module: core data structures, traits, and utility functions.
//!
//! This module defines:
//! - [`Port`] — thermodynamic state of the working fluid at a connection point
//! - [`CompSISO`] — trait interface for Single-Input Single-Output components
//! - [`PortDict`] / [`PortDictMut`] — traits for accessing component port dictionaries
//! - [`AsAny`] — trait for runtime type downcasting
//! - CoolProp integration via the `coolprop-sys` crate for thermodynamic property calculations
//! - Utility type aliases and helper functions

use std::collections::HashMap;
use std::ffi::CString;

/// Sentinel value indicating a port has not been assigned to any node.
pub const NONE_INDEX: usize = usize::MAX;

/// Thermodynamic state of the working fluid at a connection point (port).
///
/// A port stores pressure, temperature, enthalpy, entropy, quality, and mass
/// flow rate. It can calculate missing properties from known pairs using CoolProp.
///
/// # Node Sharing
/// When two components are connected, their ports share the same `Port` memory
/// (via raw pointer). This ensures state changes propagate automatically between
/// connected components — a key mechanism for the component calculation order
/// detection algorithm.
///
/// # State Calculation Methods
/// - `tx()` — from temperature and quality
/// - `px()` — from pressure and quality (or pressure and enthalpy)
/// - `ps()` — from pressure and entropy
/// - `ph()` — from pressure and enthalpy
/// - `pt()` — from pressure and temperature
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
    pub stateok: bool,
    /// Node index in the connector's node list
    pub index: usize,
}

impl Port {
    /// Creates a new Port from a dictionary of property key-value pairs.
    ///
    /// Initializes all properties to NaN, then sets provided values.
    /// If enough property pairs are given (e.g., t+x, p+x, p+t),
    /// automatically calculates the remaining properties.
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

    /// Calls CoolProp's `PropsSI` function via the `coolprop-sys` crate.
    fn propssi(&self, output: &str, name1: &str, prop1: f64, name2: &str, prop2: f64) -> f64 {
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
            self.p = self.propssi("P", "T", t_k, "Q", self.x) / 1.0e6;
            self.h = self.propssi("H", "T", t_k, "Q", self.x) / 1000.0;
            self.s = self.propssi("S", "T", t_k, "Q", self.x) / 1000.0;
            self.stateok = true;
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

    /// Calculates properties from pressure and entropy.
    ///
    /// Sets h, t, x from P and S.
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

    /// Calculates properties from pressure and enthalpy.
    ///
    /// Sets s, t, x from P and H.
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

    /// Calculates properties from pressure and temperature.
    ///
    /// Sets s, h, x from P and T.
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

    /// Attempts to calculate the port state from available property pairs.
    ///
    /// Tries in order: ps → ph → pt. Only called when `stateok` is false.
    /// This is used by `component_simulator` to resolve node states after
    /// a component's `state()` call provides new property values.
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

    /// Returns a formatted string of this port's thermodynamic state.
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

    /// Column header for port result output.
    pub const TITLE: &'static str = "Port   	P(MPa)   T(C)  H(kJ/kg)	  S(kJ/kg.K)  Quality MDOT(kg/s)";
}

/// Trait interface for Single-Input Single-Output (SISO) refrigeration cycle components.
///
/// Each component must implement:
/// - `setportaddress()` — update port pointers after connector node sharing
/// - `state()` — thermal process calculation (panics if input data is insufficient)
/// - `balance()` — energy and mass balance (panics if input data is insufficient)
/// - `resultstring()` — formatted output of component results
/// - `name()` — component name
/// - `energy()` — energy category string ("CompressionWork", "QIN", "QOUT", or "")
///
/// # Panic Convention
/// `state()` and `balance()` **must panic** when required input data is not yet
/// available (e.g., NaN values). This is not an error — it signals to the
/// `component_simulator` that this component cannot be processed yet and should
/// be retried in a later iteration.
pub trait CompSISO: PortDict + PortDictMut + AsAny {
    /// Update port pointers to match the current portdict (after node sharing).
    fn setportaddress(&mut self);
    /// Perform thermal process calculation. Panics if input data is insufficient.
    fn state(&mut self);
    /// Perform energy and mass balance. Panics if input data is insufficient.
    fn balance(&mut self);
    /// Returns a formatted string of this component's results.
    fn resultstring(&self) -> String;
    /// Returns the component name.
    fn name(&self) -> &str;
    /// Returns the energy category: "CompressionWork", "QIN", "QOUT", or "".
    fn energy(&self) -> &str;
}

/// Trait for read access to a component's port dictionary.
pub trait PortDict {
    /// Returns the map of port name → port pointer.
    fn portdict(&self) -> &HashMap<String, *mut Port>;
}

/// Trait for mutable access to a component's port dictionary.
pub trait PortDictMut {
    /// Returns the mutable map of port name → port pointer.
    fn portdict_mut(&mut self) -> &mut HashMap<String, *mut Port>;
}

/// Trait for runtime type downcasting.
///
/// Enables `component_simulator` to aggregate cycle results by downcasting
/// trait objects to concrete types (e.g., `Box<dyn CompSISO>` → `Compressor`).
pub trait AsAny {
    /// Returns a reference to `dyn Any` for downcasting.
    fn as_any(&self) -> &dyn std::any::Any;
    /// Returns a mutable reference to `dyn Any` for downcasting.
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

/// Component configuration dictionary type (from JSON parsing).
pub type UMComponent = HashMap<String, serde_json::Value>;
/// Port configuration dictionary type (from JSON parsing).
pub type MPort = HashMap<String, serde_json::Value>;
/// Port identifier tuple: (component_name, port_name).
pub type TupPort = (String, String);
/// Connector specification tuple: ((comp0, port0), (comp1, port1)).
pub type TupConnector = (TupPort, TupPort);

/// Formats a floating-point value with specified precision, or "--" if NaN.
pub fn to_string_with_precision(value: f64, precision: usize) -> String {
    if !value.is_nan() {
        format!("{:.1$}", value, precision)
    } else {
        " -- ".to_string()
    }
}

/// Converts a string slice to an owned String.
pub fn copy_string(s: &str) -> String {
    s.to_string()
}

/// Extracts a string value from a JSON Value, returning empty string if not a string.
pub fn any_to_string(val: &serde_json::Value) -> String {
    if let Some(s) = val.as_str() {
        s.to_string()
    } else {
        "".to_string()
    }
}
