from flask import Flask, request, jsonify
from simvcce.vcc.vccobj import VCCycle

def OutDict(cycle):
    result_items = {'CompressionWork(kW)': cycle.Wc,
                    'RefrigerationCapacity(kW)': cycle.Qin,
                    'Capacity(ton)': cycle.Qin*60*(1/211),
                    'The heat transfer rate(kW)': cycle.Qout,
                    'The coefficient of performance': cycle.cop,
                    'The coefficient of performance(heat pump)': cycle.cop_hp}
    return result_items
app = Flask(__name__)

cycles  = []
thedictcycle={}

@app.get("/simvcce")
def get_simvcce():
    return jsonify(cycles)

@app.post("/simvcce")
def add_simvcce():
    if request.is_json:
        thedictcycle = request.get_json()
        cycle = VCCycle(thedictcycle)
        cycle.simulator()
        #  devices
        dict_devs={}
        for key in cycle.comps.keys():
          dict_devs[key] =dict(cycle.comps[key])
        # cycle
        simvcce = dict_devs | dict(cycle)
        cycles.append(simvcce)
        result = jsonify(simvcce)
        return result, 201
    return {"error": "Request must be JSON"}, 415