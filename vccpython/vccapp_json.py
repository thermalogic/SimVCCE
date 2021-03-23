
"""
General Object-oriented Abstraction  of VP Cycle 
The Simulator of VC Cycle 
  * Input :the json file of the cycle model
  * Output: text file
Run: 
   python vccapp_json.py
"""
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles
from platform import os

# json
import json
def create_dictcycle_from_jsonfile(filename):
    """ create dict cycle from json file"""
    with open(filename, 'r') as f:
        dictcycle = json.loads(f.read())

    # convection json to the tuple in python
    for itemtuples in dictcycle["connectors"]:
        for i in range(len(itemtuples)):
            itemtuples[i] = (itemtuples[i]["devname"], itemtuples[i]["port"])
    return dictcycle

curpath = os.path.abspath(os.path.dirname(__file__))
rankinefilename = curpath+'\\'+'./jsonmodel/ivcr723.json'
thedictcycle = create_dictcycle_from_jsonfile(rankinefilename)
# the simulator
cycle = VCCycle(thedictcycle)
cycle.simulator()
# output to console
OutFiles(cycle)
# output to the file
ResultFilePath = curpath+'/result/'
ResultFileName = ResultFilePath+thedictcycle['name']
OutFiles(cycle, ResultFileName + '.txt')
