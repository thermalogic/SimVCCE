use simvcc::{Evaporator, CompSISO, UMComponent};
use std::collections::HashMap;

const FLUID: &str = "R134a";

fn make_dict(iport: &str, oport: &str) -> UMComponent {
    let mut dict = HashMap::new();
    dict.insert("name".to_string(), serde_json::Value::String("Evaporator".to_string()));
    dict.insert("classstr".to_string(), serde_json::Value::String("Evaporator".to_string()));
    dict.insert("iPort".to_string(), serde_json::from_str(iport).unwrap());
    dict.insert("oPort".to_string(), serde_json::from_str(oport).unwrap());
    dict
}

#[test]
fn test_evaporator_new() {
    let dict = make_dict(r#"{}"#, r#"{}"#);
    let comp = Evaporator::new(&dict, FLUID);
    assert_eq!(comp.name(), "Evaporator");
    assert_eq!(comp.energy(), "QIN");
}

#[test]
fn test_evaporator_state_propagate_pressure_from_input() {
    let dict = make_dict(r#"{"p": 0.2928}"#, r#"{}"#);
    let mut comp = Evaporator::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_ok());
    let o_p = comp.inner.o_port.borrow().p;
    assert!(!o_p.is_nan());
    assert!((o_p - 0.2928).abs() < 1e-6);
}

#[test]
fn test_evaporator_state_propagate_pressure_from_output() {
    let dict = make_dict(r#"{}"#, r#"{"t": 0.0, "x": 1.0}"#);
    let mut comp = Evaporator::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_ok());
    let i_p = comp.inner.i_port.borrow().p;
    let o_p = comp.inner.o_port.borrow().p;
    assert!(!i_p.is_nan());
    assert!((i_p - o_p).abs() < 1e-10, "Isobaric: p_in should equal p_out");
}

#[test]
fn test_evaporator_state_both_nan() {
    let dict = make_dict(r#"{}"#, r#"{}"#);
    let mut comp = Evaporator::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_err());
}

#[test]
fn test_evaporator_balance() {
    let dict = make_dict(r#"{"p": 0.2928, "mdot": 0.08}"#, r#"{"t": 0.0, "x": 1.0}"#);
    let mut comp = Evaporator::new(&dict, FLUID);
    comp.state().unwrap();
    comp.inner.i_port.borrow_mut().state();
    comp.inner.o_port.borrow_mut().state();
    if comp.inner.i_port.borrow().h.is_nan() {
        comp.inner.i_port.borrow_mut().h = 87.3;
        comp.inner.i_port.borrow_mut().state();
    }
    let result = comp.balance();
    assert!(result.is_ok());
    assert!(comp.qe > 0.0, "Evaporator Qin should be positive, got {}", comp.qe);
}

#[test]
fn test_evaporator_result_string() {
    let dict = make_dict(r#"{}"#, r#"{}"#);
    let comp = Evaporator::new(&dict, FLUID);
    let s = comp.result_string();
    assert!(s.contains("Evaporator"));
    assert!(s.contains("Refrigeration Capacity"));
}
