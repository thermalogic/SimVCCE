
"""
General Object-oriented Abstraction of VC Cycle 

Author: Cheng Maohua cmh@seu.edu.cn
"""

import sys
import os
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')


from vcc.vccobj import VCCycle
from vcc.utils import OutFiles, create_dictcycle_from_jsonfile

if __name__ == "__main__":
    curpath = os.path.abspath(os.path.dirname(__file__))
    
    json_filename = curpath+'\\'+'./jsonmodel/vcr_cascade_upper.json'
    
    thedictcycle = create_dictcycle_from_jsonfile(json_filename)
    cycle = VCCycle(thedictcycle)
    cycle.simulator()
    OutFiles(cycle)
    