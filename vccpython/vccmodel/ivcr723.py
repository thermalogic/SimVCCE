cycle = {}
cycle["name"] = "The Ideal VCR Example 7.2-3"
cycle["components"] = [
    {
        "name": "Compressor",
        "devtype": "COMPRESSOR",
        "iPort": {"t": 0,  "x": 1, "mdot": 0.08},
        "oPort": {},
        "ef": 1.0
    },
    {
        "name": "Condenser",
        "devtype": "CONDENSER",
        "iPort": {},
        "oPort": {"t": 26,  "x": 0},

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
