from flask import Flask, request, jsonify

app = Flask(__name__)

simvcces = []

@app.get("/simvcce")
def get_simvcce():
    return jsonify(simvcces)


@app.post("/simvcce")
def add_simvcce():
    if request.is_json:
        simvcce = request.get_json()
        simvcces.append(simvcce)
        result = {"cop": 11}
        return result, 201
    return {"error": "Request must be JSON"}, 415
