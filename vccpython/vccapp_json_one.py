
"""
General Object-oriented Abstraction of VC Cycle 

   python vccapp_json_one.py jsonfilename_of_cycle  

Author: Cheng Maohua cmh@seu.edu.cn
"""
import os
import json

from vcc.vccobj import VCCycle
from vcc.utils import OutFiles

if __name__ == "__main__":
    import sys
    jsonname=sys.argv[1]

    curpath = os.path.abspath(os.path.dirname(__file__))
    json_filename = curpath+'\\'+'jsonmodel/'+jsonname
    with open(json_filename, 'r') as f:
        thedictcycle = json.loads(f.read())
  
    cycle = VCCycle(thedictcycle)
    cycle.simulator()
    OutFiles(cycle)
   
    ResultFileName = curpath+'/result/'+thedictcycle['name']
    OutFiles(cycle, ResultFileName + '.txt')
