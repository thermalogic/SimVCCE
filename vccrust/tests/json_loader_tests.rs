use simvcc::JSONLoader;

#[test]
fn test_json_loader_new() {
    let _loader = JSONLoader::new();
}

#[test]
fn test_json_loader_default() {
    let _loader = JSONLoader::default();
}

#[test]
fn test_load_file_not_found() {
    let loader = JSONLoader::new();
    let result = loader.load_file("nonexistent_file.json");
    assert!(result.is_err());
}

#[test]
fn test_load_file_valid_json() {
    let loader = JSONLoader::new();
    let result = loader.load_file("jsonmodel/demovcc.json");
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.get("components").is_some());
    assert!(json.get("connectors").is_some());
    assert_eq!(json.get("refrigerant").unwrap().as_str(), Some("R134a"));
}

#[test]
fn test_create_cycle_from_json() {
    let loader = JSONLoader::new();
    let json = loader.load_file("jsonmodel/demovcc.json").unwrap();
    let result = loader.create_cycle(&json);
    assert!(result.is_ok());
    let cycle = result.unwrap();
    assert_eq!(cycle.comps.len(), 4);
    assert_eq!(cycle.curcon.nodes.len(), 4);
}

#[test]
fn test_create_cycle_and_simulate() {
    let loader = JSONLoader::new();
    let json = loader.load_file("jsonmodel/demovcc.json").unwrap();
    let mut cycle = loader.create_cycle(&json).unwrap();
    cycle.simulator();
    assert!(cycle.wc > 0.0);
    assert!(cycle.qin > 0.0);
    assert!(cycle.cop > 0.0);
}

#[test]
fn test_create_cycle_invalid_json() {
    let loader = JSONLoader::new();
    let json = serde_json::json!({
        "components": [],
        "connectors": {}
    });
    let result = loader.create_cycle(&json);
    assert!(result.is_ok());
    let cycle = result.unwrap();
    assert_eq!(cycle.comps.len(), 0);
}

#[test]
fn test_create_cycle_default_refrigerant() {
    let loader = JSONLoader::new();
    let json = serde_json::json!({
        "components": [],
        "connectors": {}
    });
    let result = loader.create_cycle(&json);
    assert!(result.is_ok());
}
