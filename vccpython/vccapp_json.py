
"""
General Object-oriented Abstraction  of VC Cycle 

Yunus A. Cengel, Michael A. Boles, Thermodynamics: An Engineering Approach, 8th Edition,McGraw-Hill, 2015.


The Simulator of VC Cycle 
  * Input :the json file of the cycle model
  * Output: text file
Run: 
   python vccapp_json.py

 Author: Cheng Maohua cmh@seu.edu.cn

"""
import os
import glob
import json
from simvcce.vcc.vccobj import VCCycle
from simvcce.vcc.utils import OutFiles

if __name__ == "__main__":
    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'

    json_filenames_str = curpath+'/jsonmodel/*.json'
    json_filenames = glob.glob(json_filenames_str)

    for i in range(len(json_filenames)):
        with open(json_filenames[i], 'r') as f:
            thedictcycle = json.loads(f.read())

        # the simulator
        cycle = VCCycle(thedictcycle)
        cycle.simulator()
        # output to console
        OutFiles(cycle)
        # output to the file
        ResultFileName = ResultFilePath+thedictcycle['name']
        OutFiles(cycle, ResultFileName + '.txt')
