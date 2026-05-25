use simvcc::{Condenser, CompSISO, UMComponent};
use std::collections::HashMap;

const FLUID: &str = "R134a";

fn make_dict(iport: &str, oport: &str) -> UMComponent {
    let mut dict = HashMap::new();
    dict.insert("name".to_string(), serde_json::Value::String("Condenser".to_string()));
    dict.insert("classstr".to_string(), serde_json::Value::String("Condenser".to_string()));
    dict.insert("iPort".to_string(), serde_json::from_str(iport).unwrap());
    dict.insert("oPort".to_string(), serde_json::from_str(oport).unwrap());
    dict
}

#[test]
fn test_condenser_new() {
    let dict = make_dict(r#"{}"#, r#"{"t": 26.0, "x": 0.0}"#);
    let comp = Condenser::new(&dict, FLUID);
    assert_eq!(comp.name(), "Condenser");
    assert_eq!(comp.energy(), "QOUT");
}

#[test]
fn test_condenser_state_propagate_pressure_from_output() {
    let dict = make_dict(r#"{}"#, r#"{"t": 26.0, "x": 0.0}"#);
    let mut comp = Condenser::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_ok());
    let i_p = comp.inner.i_port.borrow().p;
    let o_p = comp.inner.o_port.borrow().p;
    assert!(!i_p.is_nan());
    assert!((i_p - o_p).abs() < 1e-10, "Isobaric: p_in should equal p_out");
}

#[test]
fn test_condenser_state_propagate_pressure_from_input() {
    let dict = make_dict(r#"{"p": 0.6854}"#, r#"{}"#);
    let mut comp = Condenser::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_ok());
    let o_p = comp.inner.o_port.borrow().p;
    assert!(!o_p.is_nan());
    assert!((o_p - 0.6854).abs() < 1e-6);
}

#[test]
fn test_condenser_state_both_nan() {
    let dict = make_dict(r#"{}"#, r#"{}"#);
    let mut comp = Condenser::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_err());
}

#[test]
fn test_condenser_balance() {
    let dict = make_dict(r#"{"p": 0.6854, "mdot": 0.08}"#, r#"{"t": 26.0, "x": 0.0}"#);
    let mut comp = Condenser::new(&dict, FLUID);
    comp.state().unwrap();
    comp.inner.i_port.borrow_mut().state();
    comp.inner.o_port.borrow_mut().state();
    if comp.inner.i_port.borrow().h.is_nan() {
        comp.inner.i_port.borrow_mut().h = 420.0;
        comp.inner.i_port.borrow_mut().state();
    }
    let result = comp.balance();
    assert!(result.is_ok());
    assert!(comp.qc > 0.0, "Condenser Qout should be positive, got {}", comp.qc);
}

#[test]
fn test_condenser_result_string() {
    let dict = make_dict(r#"{}"#, r#"{"t": 26.0, "x": 0.0}"#);
    let comp = Condenser::new(&dict, FLUID);
    let s = comp.result_string();
    assert!(s.contains("Condenser"));
    assert!(s.contains("Capacity"));
}
