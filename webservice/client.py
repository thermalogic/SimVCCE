import requests
import pprint

api_url = "http://127.0.0.1:5000/simvcce"
json_filename = './jsonmodel/ivcr_11_1.json'

# post
print("requests.post\n")

headers = {"Content-Type": "application/json"}

with open(json_filename, 'r') as f:
    json_cycle=f.read()

response = requests.post(api_url, data=json_cycle, headers=headers)
result = response.json()

pprint.pprint(result)
#print(response.status_code)

# get
#print("\nrequests.get\n")
#response = requests.get(api_url)
#pprint.pprint(response.json())
