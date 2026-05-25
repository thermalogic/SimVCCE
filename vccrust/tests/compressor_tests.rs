use simvcc::{Compressor, CompSISO, UMComponent};
use std::collections::HashMap;

const FLUID: &str = "R134a";

fn make_dict(iport: &str, oport: &str) -> UMComponent {
    let mut dict = HashMap::new();
    dict.insert("name".to_string(), serde_json::Value::String("Compressor".to_string()));
    dict.insert("classstr".to_string(), serde_json::Value::String("Compressor".to_string()));
    dict.insert("iPort".to_string(), serde_json::from_str(iport).unwrap());
    dict.insert("oPort".to_string(), serde_json::from_str(oport).unwrap());
    dict
}

#[test]
fn test_compressor_new() {
    let dict = make_dict(r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#, r#"{"p": 0.6854}"#);
    let comp = Compressor::new(&dict, FLUID);
    assert_eq!(comp.name(), "Compressor");
    assert_eq!(comp.energy(), "CompressionWork");
    assert_eq!(comp.energy_value(), 0.0);
}

#[test]
fn test_compressor_state_ok() {
    let dict = make_dict(r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#, r#"{"p": 0.6854}"#);
    let mut comp = Compressor::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_ok());
    let i_s = comp.inner.i_port.borrow().s;
    let o_s = comp.inner.o_port.borrow().s;
    assert!(!i_s.is_nan());
    assert!(!o_s.is_nan());
    assert!((i_s - o_s).abs() < 1e-10, "Isentropic: s_in should equal s_out");
}

#[test]
fn test_compressor_state_nan_entropy() {
    let dict = make_dict(r#"{}"#, r#"{"p": 0.6854}"#);
    let mut comp = Compressor::new(&dict, FLUID);
    let result = comp.state();
    assert!(result.is_err());
}

#[test]
fn test_compressor_balance() {
    let dict = make_dict(r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#, r#"{"p": 0.6854}"#);
    let mut comp = Compressor::new(&dict, FLUID);
    comp.state().unwrap();
    comp.inner.o_port.borrow_mut().state();
    let result = comp.balance();
    assert!(result.is_ok());
    assert!(comp.wc > 0.0, "Compression work should be positive, got {}", comp.wc);
}

#[test]
fn test_compressor_balance_nan_mdot() {
    let dict = make_dict(r#"{"t": 0.0, "x": 1.0}"#, r#"{"p": 0.6854}"#);
    let mut comp = Compressor::new(&dict, FLUID);
    comp.state().unwrap();
    comp.inner.o_port.borrow_mut().state();
    let result = comp.balance();
    assert!(result.is_err());
}

#[test]
fn test_compressor_result_string() {
    let dict = make_dict(r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#, r#"{"p": 0.6854}"#);
    let comp = Compressor::new(&dict, FLUID);
    let s = comp.result_string();
    assert!(s.contains("Compressor"));
    assert!(s.contains("Work"));
}
