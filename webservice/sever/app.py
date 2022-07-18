from flask import Flask, request, jsonify
from simvcce.vcc.vccobj import VCCycle

app = Flask(__name__)

cycles  = [{"Hi": "Please use the POST request with JSON"}]


@app.route("/")
def hello_world():
    return "<p>Hello, Here is the SimVCCE service, the service route is /simvcce</p>"
    
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
        cycles[0]=simvcce
        result = jsonify(simvcce)
        return result, 201
    return {"error": "Request must be JSON"}, 415

if __name__ == '__main__':
    app.run()
