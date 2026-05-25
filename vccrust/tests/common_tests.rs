use simvcc::{to_string_with_precision, any_to_string, SimulationError};

#[test]
fn test_to_string_with_precision_normal() {
    assert_eq!(to_string_with_precision(3.14159, 2), "3.14");
    assert_eq!(to_string_with_precision(3.14159, 3), "3.142");
    assert_eq!(to_string_with_precision(0.0, 3), "0.000");
    assert_eq!(to_string_with_precision(-1.5, 1), "-1.5");
}

#[test]
fn test_to_string_with_precision_nan() {
    assert_eq!(to_string_with_precision(f64::NAN, 3), " -- ");
}

#[test]
fn test_any_to_string_from_str() {
    let val = serde_json::Value::String("hello".to_string());
    assert_eq!(any_to_string(&val), "hello");
}

#[test]
fn test_any_to_string_from_number() {
    let val = serde_json::Value::Number(42.into());
    assert_eq!(any_to_string(&val), "");
}

#[test]
fn test_any_to_string_from_null() {
    assert_eq!(any_to_string(&serde_json::Value::Null), "");
}

#[test]
fn test_simulation_error_new() {
    let err = SimulationError::new("test error");
    assert_eq!(err.message, "test error");
}

#[test]
fn test_simulation_error_display() {
    let err = SimulationError::new("missing data");
    assert_eq!(format!("{}", err), "SimulationError: missing data");
}

#[test]
fn test_simulation_error_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(SimulationError::new("test"));
    assert_eq!(err.to_string(), "SimulationError: test");
}
