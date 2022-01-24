cycle = {}
cycle["name"] = "The Ideal VCR Example 11-1"
cycle["refrigerant"] = "R134a"
cycle["components"] = [
    {
        "name": "Compressor",
        "devtype": "COMPRESSOR",
        "iPort": {"p": 0.14,  "x": 1, "mdot": 0.05},
        "oPort": {},
        "ef": 1.0
    },
    {
        "name": "Condenser",
        "devtype": "CONDENSER",
        "iPort": {},
        "oPort": {"p": 0.8,  "x": 0},

    },
    {
        "name": "ExpansionValve",
        "devtype": "EXPANSIONVALVE",
        "iPort": {},
        "oPort": {},
    },
    {
        "name": "Evaporator",
        "devtype": "EVAPORATOR",
        "iPort": {},
        "oPort": {},
    }
]

cycle["connectors"] = [
    (("Compressor", "oPort"), ("Condenser", "iPort")),
    (("Condenser", "oPort"), ("ExpansionValve", "iPort")),
    (("ExpansionValve", "oPort"), ("Evaporator", "iPort")),
    (("Evaporator", "oPort"), ("Compressor", "iPort"))]
