use simvcc::{ExpansionValve, CompSISO, UMComponent};
use std::collections::HashMap;

const FLUID: &str = "R134a";

fn make_dict(iport: &str, oport: &str) -> UMComponent {
    let mut dict = HashMap::new();
    dict.insert("name".to_string(), serde_json::Value::String("ExpansionValve".to_string()));
    dict.insert("classstr".to_string(), serde_json::Value::String("ExpansionValve".to_string()));
    dict.insert("iPort".to_string(), serde_json::from_str(iport).unwrap());
    dict.insert("oPort".to_string(), serde_json::from_str(oport).unwrap());
    dict
}

#[test]
fn test_expansion_valve_new() {
    let dict = make_dict(r#"{}"#, r#"{}"#);
    let comp = ExpansionValve::new(&dict, FLUID);
    assert_eq!(comp.name(), "ExpansionValve");
    assert_eq!(comp.energy(), "");
    assert_eq!(comp.energy_value(), 0.0);
}

#[test]
fn test_expansion_valve_state_propagate_enthalpy_from_input() {
    let dict = make_dict(r#"{"p": 0.6854, "t": 26.0, "x": 0.0}"#, r#"{}"#);
    let mut comp = ExpansionValve::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_ok());
    let o_h = comp.inner.o_port.borrow().h;
    assert!(!o_h.is_nan(), "oPort.h should be propagated from iPort");
}

#[test]
fn test_expansion_valve_state_propagate_enthalpy_from_output() {
    let dict = make_dict(r#"{}"#, r#"{"p": 0.2928, "h": 87.3}"#);
    let mut comp = ExpansionValve::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_ok());
    let i_h = comp.inner.i_port.borrow().h;
    assert!(!i_h.is_nan());
    assert!((i_h - 87.3).abs() < 0.1);
}

#[test]
fn test_expansion_valve_state_both_nan() {
    let dict = make_dict(r#"{}"#, r#"{}"#);
    let mut comp = ExpansionValve::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_err());
}

#[test]
fn test_expansion_valve_balance() {
    let dict = make_dict(r#"{"p": 0.6854, "t": 26.0, "x": 0.0, "mdot": 0.08}"#, r#"{}"#);
    let mut comp = ExpansionValve::new(&dict, FLUID);
    comp.state().unwrap();
    let result = comp.balance();
    assert!(result.is_ok());
}

#[test]
fn test_expansion_valve_balance_nan_mdot() {
    let dict = make_dict(r#"{"p": 0.6854, "t": 26.0, "x": 0.0}"#, r#"{}"#);
    let mut comp = ExpansionValve::new(&dict, FLUID);
    comp.state().unwrap();
    let result = comp.balance();
    assert!(result.is_err());
}

#[test]
fn test_expansion_valve_isenthalpic() {
    let dict = make_dict(r#"{"p": 0.6854, "t": 26.0, "x": 0.0}"#, r#"{}"#);
    let mut comp = ExpansionValve::new(&dict, FLUID);
    comp.state().unwrap();
    let i_h = comp.inner.i_port.borrow().h;
    let o_h = comp.inner.o_port.borrow().h;
    assert!(!i_h.is_nan());
    assert!(!o_h.is_nan());
    assert!((i_h - o_h).abs() < 1e-10, "Isenthalpic: h_in should equal h_out");
}

#[test]
fn test_expansion_valve_result_string() {
    let dict = make_dict(r#"{}"#, r#"{}"#);
    let comp = ExpansionValve::new(&dict, FLUID);
    let s = comp.result_string();
    assert!(s.contains("ExpansionValve"));
}
