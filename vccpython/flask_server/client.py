import requests
import json
api_url = "http://127.0.0.1:5000/simvcce"
response = requests.get(api_url)
j=response.json()
print(j)
print(response.status_code)

headers = {"Content-Type": "application/json"}
todo = {"name": "China", "capital": "Baijing", "area": 211100}
response = requests.post(api_url, data=json.dumps(todo), headers=headers)
#response = requests.put(api_url, json=todo)
j=response.json()
print(j)
print(response.status_code)
