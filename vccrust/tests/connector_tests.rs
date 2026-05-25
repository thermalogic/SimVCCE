use simvcc::{Connector, CompSISO, Compressor, Condenser, UMComponent};
use std::collections::HashMap;
use std::rc::Rc;

const FLUID: &str = "R134a";

fn make_compressor_dict(name: &str, iport: &str, oport: &str) -> UMComponent {
    let mut dict = HashMap::new();
    dict.insert("name".to_string(), serde_json::Value::String(name.to_string()));
    dict.insert("classstr".to_string(), serde_json::Value::String("Compressor".to_string()));
    dict.insert("iPort".to_string(), serde_json::from_str(iport).unwrap());
    dict.insert("oPort".to_string(), serde_json::from_str(oport).unwrap());
    dict
}

#[test]
fn test_connector_new() {
    let con = Connector::new();
    assert_eq!(con.index, 0);
    assert!(con.nodes.is_empty());
}

#[test]
fn test_connector_default() {
    let con = Connector::default();
    assert_eq!(con.index, 0);
    assert!(con.nodes.is_empty());
}

#[test]
fn test_add_connector_creates_shared_node() {
    let mut comps: HashMap<String, Box<dyn CompSISO>> = HashMap::new();

    let comp0_dict = make_compressor_dict(
        "Comp0",
        r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#,
        r#"{"p": 0.6854}"#,
    );
    let comp1_dict = make_compressor_dict(
        "Comp1",
        r#"{}"#,
        r#"{}"#,
    );

    comps.insert("Comp0".to_string(), Box::new(Compressor::new(&comp0_dict, FLUID)) as Box<dyn CompSISO>);
    comps.insert("Comp1".to_string(), Box::new(Condenser::new(&comp1_dict, FLUID)) as Box<dyn CompSISO>);

    let mut con = Connector::new();
    let tconn = (("Comp0".to_string(), "oPort".to_string()), ("Comp1".to_string(), "iPort".to_string()));
    let result = con.add_connector(tconn, &mut comps);
    assert!(result.is_ok());
    assert_eq!(con.nodes.len(), 1);

    let port0 = comps.get("Comp0").unwrap().portdict().get("oPort").unwrap().clone();
    let port1 = comps.get("Comp1").unwrap().portdict().get("iPort").unwrap().clone();
    assert!(Rc::ptr_eq(&port0, &port1), "Ports should share the same Rc");
}

#[test]
fn test_add_connector_merges_values() {
    let mut comps: HashMap<String, Box<dyn CompSISO>> = HashMap::new();

    let comp0_dict = make_compressor_dict(
        "Comp0",
        r#"{"t": 0.0, "x": 1.0, "mdot": 0.08}"#,
        r#"{"p": 0.6854}"#,
    );
    let comp1_dict = make_compressor_dict(
        "Comp1",
        r#"{}"#,
        r#"{"t": 26.0, "x": 0.0}"#,
    );

    comps.insert("Comp0".to_string(), Box::new(Compressor::new(&comp0_dict, FLUID)) as Box<dyn CompSISO>);
    comps.insert("Comp1".to_string(), Box::new(Condenser::new(&comp1_dict, FLUID)) as Box<dyn CompSISO>);

    let mut con = Connector::new();
    let tconn = (("Comp0".to_string(), "oPort".to_string()), ("Comp1".to_string(), "iPort".to_string()));
    con.add_connector(tconn, &mut comps).unwrap();

    let node = &con.nodes[0];
    let node_ref = node.borrow();
    assert!(!node_ref.p.is_nan(), "p should be merged from Comp0.oPort");
}

#[test]
fn test_add_connector_missing_component() {
    let mut comps: HashMap<String, Box<dyn CompSISO>> = HashMap::new();
    let comp_dict = make_compressor_dict("Comp0", r#"{}"#, r#"{}"#);
    comps.insert("Comp0".to_string(), Box::new(Compressor::new(&comp_dict, FLUID)) as Box<dyn CompSISO>);

    let mut con = Connector::new();
    let tconn = (("Comp0".to_string(), "oPort".to_string()), ("NonExistent".to_string(), "iPort".to_string()));
    let result = con.add_connector(tconn, &mut comps);
    assert!(result.is_err());
}

#[test]
fn test_add_connector_missing_port() {
    let mut comps: HashMap<String, Box<dyn CompSISO>> = HashMap::new();
    let comp_dict = make_compressor_dict("Comp0", r#"{}"#, r#"{}"#);
    comps.insert("Comp0".to_string(), Box::new(Compressor::new(&comp_dict, FLUID)) as Box<dyn CompSISO>);
    let comp_dict2 = make_compressor_dict("Comp1", r#"{}"#, r#"{}"#);
    comps.insert("Comp1".to_string(), Box::new(Compressor::new(&comp_dict2, FLUID)) as Box<dyn CompSISO>);

    let mut con = Connector::new();
    let tconn = (("Comp0".to_string(), "nonExistentPort".to_string()), ("Comp1".to_string(), "iPort".to_string()));
    let result = con.add_connector(tconn, &mut comps);
    assert!(result.is_err());
}
