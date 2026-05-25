use simvcc::{Port, NONE_INDEX};
use std::collections::HashMap;

const FLUID: &str = "R134a";

fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn test_port_new_empty() {
    let port = Port::new(&HashMap::new(), FLUID);
    assert!(port.p.is_nan());
    assert!(port.t.is_nan());
    assert!(port.h.is_nan());
    assert!(port.s.is_nan());
    assert!(port.x.is_nan());
    assert!(port.mdot.is_nan());
    assert!(!port.state_ok);
}

#[test]
fn test_port_new_with_mdot_only() {
    let mut data = HashMap::new();
    data.insert("mdot".to_string(), 0.08);
    let port = Port::new(&data, FLUID);
    assert!(port.p.is_nan());
    assert!(!port.mdot.is_nan());
    assert_eq!(port.mdot, 0.08);
    assert!(!port.state_ok);
}

#[test]
fn test_port_new_tx() {
    let mut data = HashMap::new();
    data.insert("t".to_string(), 0.0);
    data.insert("x".to_string(), 1.0);
    let port = Port::new(&data, FLUID);
    assert!(port.state_ok);
    assert!(approx_eq(port.p, 0.2928, 0.01), "p = {}", port.p);
    assert!(port.h > 300.0 && port.h < 500.0, "h = {}", port.h);
    assert!(port.s > 1.0 && port.s < 2.5, "s = {}", port.s);
}

#[test]
fn test_port_new_px_saturated_vapor() {
    let mut data = HashMap::new();
    data.insert("p".to_string(), 0.2928);
    data.insert("x".to_string(), 1.0);
    let port = Port::new(&data, FLUID);
    assert!(port.state_ok);
    assert!(approx_eq(port.t, 0.0, 0.5), "t = {}", port.t);
    assert!(port.h > 300.0, "h = {}", port.h);
}

#[test]
fn test_port_new_px_subcooled_with_h() {
    let mut ref_data = HashMap::new();
    ref_data.insert("p".to_string(), 0.6854);
    ref_data.insert("x".to_string(), 0.0);
    let ref_port = Port::new(&ref_data, FLUID);
    let h_sat_liq = ref_port.h;

    let mut data = HashMap::new();
    data.insert("p".to_string(), 0.6854);
    data.insert("h".to_string(), h_sat_liq);
    let mut port = Port::new(&data, FLUID);
    if !port.state_ok {
        port.state();
    }
    assert!(port.state_ok);
    assert!(!port.t.is_nan(), "t should be calculated");
    assert!(!port.s.is_nan(), "s should be calculated");
}

#[test]
fn test_port_new_pt() {
    let mut data = HashMap::new();
    data.insert("p".to_string(), 0.6854);
    data.insert("t".to_string(), 26.0);
    let port = Port::new(&data, FLUID);
    assert!(port.state_ok);
    assert!(!port.h.is_nan(), "h should be calculated");
    assert!(port.h > 0.0, "h = {}", port.h);
}

#[test]
fn test_port_ps() {
    let mut data = HashMap::new();
    data.insert("p".to_string(), 0.2928);
    data.insert("x".to_string(), 1.0);
    let port1 = Port::new(&data, FLUID);

    let mut data2 = HashMap::new();
    data2.insert("p".to_string(), 0.2928);
    data2.insert("s".to_string(), port1.s);
    let mut port2 = Port::new(&data2, FLUID);
    if !port2.state_ok {
        port2.state();
    }
    assert!(port2.state_ok);
    assert!(approx_eq(port2.h, port1.h, 0.1), "h = {} vs {}", port2.h, port1.h);
}

#[test]
fn test_port_ph() {
    let mut data = HashMap::new();
    data.insert("p".to_string(), 0.2928);
    data.insert("x".to_string(), 1.0);
    let port1 = Port::new(&data, FLUID);

    let mut data2 = HashMap::new();
    data2.insert("p".to_string(), 0.2928);
    data2.insert("h".to_string(), port1.h);
    let mut port2 = Port::new(&data2, FLUID);
    if !port2.state_ok {
        port2.state();
    }
    assert!(port2.state_ok);
    assert!(approx_eq(port2.s, port1.s, 0.001), "s = {} vs {}", port2.s, port1.s);
}

#[test]
fn test_port_state_resolves_from_partial() {
    let mut data = HashMap::new();
    data.insert("p".to_string(), 0.2928);
    let mut port = Port::new(&data, FLUID);
    assert!(!port.state_ok);
    port.state();
    assert!(!port.state_ok);

    port.x = 1.0;
    port.state();
    assert!(port.state_ok);
    assert!(port.h > 0.0);
}

#[test]
fn test_port_result_string_format() {
    let mut data = HashMap::new();
    data.insert("t".to_string(), 0.0);
    data.insert("x".to_string(), 1.0);
    let port = Port::new(&data, FLUID);
    let s = port.result_string();
    assert!(!s.is_empty());
    assert!(!s.contains("--"));
}

#[test]
fn test_port_result_string_nan_quality() {
    let mut data = HashMap::new();
    data.insert("p".to_string(), 0.6854);
    data.insert("t".to_string(), 50.0);
    let port = Port::new(&data, FLUID);
    let s = port.result_string();
    assert!(s.contains("--"));
}

#[test]
fn test_none_index() {
    assert_eq!(NONE_INDEX, usize::MAX);
}
