import requests
import json
import pprint

api_url = "http://127.0.0.1:5000/simvcce"

print("\nrequests.post\n")
headers = {"Content-Type": "application/json"}

json_filename ='./jsonmodel/ivcr_11_1.json'
with open(json_filename, 'r') as f:
    todo=json.loads(f.read())
    response = requests.post(api_url, data=json.dumps(todo), headers=headers)
    result = response.json()
    pprint.pprint(result)
    print(response.status_code)

# get
print("\nrequests.get\n")
response = requests.get(api_url)
pprint.pprint(response.json())
