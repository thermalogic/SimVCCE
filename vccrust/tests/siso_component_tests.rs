use simvcc::{SISOComponent, UMComponent};
use std::collections::HashMap;
use std::rc::Rc;

const FLUID: &str = "R134a";

fn make_comp_dict(name: &str, iport_json: &str, oport_json: &str) -> UMComponent {
    let mut dict = HashMap::new();
    dict.insert("name".to_string(), serde_json::Value::String(name.to_string()));
    dict.insert("classstr".to_string(), serde_json::Value::String("Compressor".to_string()));
    dict.insert("iPort".to_string(), serde_json::from_str(iport_json).unwrap());
    dict.insert("oPort".to_string(), serde_json::from_str(oport_json).unwrap());
    dict
}

#[test]
fn test_siso_component_new() {
    let dict = make_comp_dict("TestComp", r#"{"t": 0.0, "x": 1.0}"#, r#"{"p": 0.6854}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Default", "CompressionWork");
    assert_eq!(comp.name, "TestComp");
    assert_eq!(comp.energy, "CompressionWork");
    assert!(comp.portdict.contains_key("iPort"));
    assert!(comp.portdict.contains_key("oPort"));
}

#[test]
fn test_siso_component_new_default_name() {
    let dict = make_comp_dict("TestComp", r#"{}"#, r#"{}"#);
    let mut dict_no_name = dict.clone();
    dict_no_name.remove("name");
    let comp = SISOComponent::new(&dict_no_name, FLUID, "FallbackName", "QIN");
    assert_eq!(comp.name, "FallbackName");
}

#[test]
fn test_propagate_mdot_from_input() {
    let dict = make_comp_dict("Comp", r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#, r#"{"p": 0.6854}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    assert!(!comp.i_port.borrow().mdot.is_nan());
    assert!(comp.o_port.borrow().mdot.is_nan());

    let result = comp.propagate_mdot("Comp");
    assert!(result.is_ok());
    assert!(!comp.o_port.borrow().mdot.is_nan());
    assert_eq!(comp.o_port.borrow().mdot, 0.08);
}

#[test]
fn test_propagate_mdot_from_output() {
    let dict = make_comp_dict("Comp", r#"{"t": 0.0, "x": 1.0}"#, r#"{"p": 0.6854, "mdot": 0.05}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    let result = comp.propagate_mdot("Comp");
    assert!(result.is_ok());
    assert_eq!(comp.i_port.borrow().mdot, 0.05);
}

#[test]
fn test_propagate_mdot_both_nan() {
    let dict = make_comp_dict("Comp", r#"{"t": 0.0, "x": 1.0}"#, r#"{"p": 0.6854}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    let result = comp.propagate_mdot("Comp");
    assert!(result.is_err());
}

#[test]
fn test_get_enthalpies_ok() {
    let dict = make_comp_dict("Comp", r#"{"t": 0.0, "x": 1.0}"#, r#"{"p": 0.6854, "t": 26.0, "x": 0.0}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    let result = comp.get_enthalpies("Comp");
    assert!(result.is_ok());
    let (i_h, o_h) = result.unwrap();
    assert!(i_h > 0.0);
    assert!(o_h > 0.0);
}

#[test]
fn test_get_enthalpies_nan() {
    let dict = make_comp_dict("Comp", r#"{}"#, r#"{}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    let result = comp.get_enthalpies("Comp");
    assert!(result.is_err());
}

#[test]
fn test_mdot() {
    let dict = make_comp_dict("Comp", r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#, r#"{"p": 0.6854}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    assert_eq!(comp.mdot(), 0.08);
}

#[test]
fn test_set_port_address_no_change() {
    let dict = make_comp_dict("Comp", r#"{"t": 0.0, "x": 1.0}"#, r#"{"p": 0.6854}"#);
    let mut comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    let i_ptr = comp.i_port.clone();
    comp.set_port_address();
    assert!(Rc::ptr_eq(&comp.i_port, &i_ptr));
}

#[test]
fn test_port_result_string() {
    let dict = make_comp_dict("Comp", r#"{"t": 0.0, "x": 1.0}"#, r#"{"p": 0.6854, "t": 26.0, "x": 0.0}"#);
    let comp = SISOComponent::new(&dict, FLUID, "Comp", "CompressionWork");
    let s = comp.port_result_string();
    assert!(!s.is_empty());
    assert!(s.contains("Port"));
}
