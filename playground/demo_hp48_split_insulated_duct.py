
"""
General Object-oriented Abstraction of VC Cycle 

The heat pump system:

* heat -> evaporator

* condenser -> heat -> the air flowing through the insulated duct

 Author: Cheng Maohua cmh@seu.edu.cn
"""

import sys
import os
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')


from vcc.vccobj import VCCycle
from vcc.utils import OutFiles, create_dictcycle_from_jsonfile
from components.port import Port
# new components
from ncomponents.insulated_duct import Insulated_Duct

from platform import os

if __name__ == "__main__":
    curpath = os.path.abspath(os.path.dirname(__file__))
    
    # 1 VC heat pump 
    json_filename = curpath+'\\'+'./jsonmodel/vchp_ohio_48.json'
    
    thedictcycle = create_dictcycle_from_jsonfile(json_filename)
    cycle = VCCycle(thedictcycle)
    cycle.simulator()
    OutFiles(cycle)
    
    # 2 the insulated duct
    dictduct = {"in": {"p": 100.0, "t": 15.0, "vdot": 8.0},
                "out": {"p": 95.0},
                "Qin": cycle.Qout   # the condenser -> heat -> the insulated duct
                }
    duct = Insulated_Duct(dictduct)
    duct.get_outt()
    print(duct)
