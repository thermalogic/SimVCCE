use simvcc::{VCCycle, UMComponent, TupConnector};

fn make_demo_components() -> Vec<UMComponent> {
    let json_str = r#"[
        {
            "name": "Compressor",
            "classstr": "Compressor",
            "iPort": {"t": 0.0, "x": 1.0, "mdot": 0.08},
            "oPort": {"p": 0.6854}
        },
        {
            "name": "Condenser",
            "classstr": "Condenser",
            "iPort": {},
            "oPort": {"t": 26.0, "x": 0.0}
        },
        {
            "name": "ExpansionValve",
            "classstr": "ExpansionValve",
            "iPort": {},
            "oPort": {}
        },
        {
            "name": "Evaporator",
            "classstr": "Evaporator",
            "iPort": {},
            "oPort": {}
        }
    ]"#;
    let arr: Vec<UMComponent> = serde_json::from_str(json_str).unwrap();
    arr
}

fn make_demo_connectors() -> Vec<TupConnector> {
    vec![
        (("Compressor".to_string(), "oPort".to_string()), ("Condenser".to_string(), "iPort".to_string())),
        (("Condenser".to_string(), "oPort".to_string()), ("ExpansionValve".to_string(), "iPort".to_string())),
        (("ExpansionValve".to_string(), "oPort".to_string()), ("Evaporator".to_string(), "iPort".to_string())),
        (("Evaporator".to_string(), "oPort".to_string()), ("Compressor".to_string(), "iPort".to_string())),
    ]
}

#[test]
fn test_vccycle_new() {
    let comps = make_demo_components();
    let conns = make_demo_connectors();
    let cycle = VCCycle::new(comps, conns, "R134a");
    assert!(cycle.is_ok());
    let cycle = cycle.unwrap();
    assert_eq!(cycle.comps.len(), 4);
    assert_eq!(cycle.curcon.nodes.len(), 4);
}

#[test]
fn test_vccycle_new_unknown_classstr() {
    let json_str = r#"[
        {
            "name": "UnknownComp",
            "classstr": "UnknownType",
            "iPort": {},
            "oPort": {}
        }
    ]"#;
    let comps: Vec<UMComponent> = serde_json::from_str(json_str).unwrap();
    let conns: Vec<TupConnector> = vec![];
    let cycle = VCCycle::new(comps, conns, "R134a");
    assert!(cycle.is_ok());
    let cycle = cycle.unwrap();
    assert_eq!(cycle.comps.len(), 0);
}

#[test]
fn test_vccycle_simulator() {
    let comps = make_demo_components();
    let conns = make_demo_connectors();
    let mut cycle = VCCycle::new(comps, conns, "R134a").unwrap();
    cycle.simulator();

    assert!(cycle.wc > 0.0, "Wc should be positive, got {}", cycle.wc);
    assert!(cycle.qin > 0.0, "Qin should be positive, got {}", cycle.qin);
    assert!(cycle.qout > 0.0, "Qout should be positive, got {}", cycle.qout);
    assert!(cycle.cop > 0.0, "COP should be positive, got {}", cycle.cop);
    assert!(cycle.cop_hp > 0.0, "COP_hp should be positive, got {}", cycle.cop_hp);

    let energy_balance = (cycle.qout - cycle.qin - cycle.wc).abs();
    assert!(energy_balance < 0.01, "Energy balance: Qout - Qin - Wc = {}", energy_balance);

    assert!((cycle.cop - cycle.qin / cycle.wc).abs() < 1e-10);
    assert!((cycle.cop_hp - cycle.qout / cycle.wc).abs() < 1e-10);

    assert!(cycle.cop > 1.0 && cycle.cop < 15.0, "COP = {} seems unreasonable", cycle.cop);
}

#[test]
fn test_vccycle_result_str() {
    let comps = make_demo_components();
    let conns = make_demo_connectors();
    let mut cycle = VCCycle::new(comps, conns, "R134a").unwrap();
    cycle.simulator();
    let s = cycle.result_str();
    assert!(s.contains("Compression Work"));
    assert!(s.contains("Refrigeration Capacity"));
    assert!(s.contains("coefficient of performance"));
}

#[test]
fn test_vccycle_connector_error() {
    let comps = make_demo_components();
    let bad_conns = vec![
        (("NonExistent".to_string(), "oPort".to_string()), ("Condenser".to_string(), "iPort".to_string())),
    ];
    let cycle = VCCycle::new(comps, bad_conns, "R134a");
    assert!(cycle.is_err());
}
