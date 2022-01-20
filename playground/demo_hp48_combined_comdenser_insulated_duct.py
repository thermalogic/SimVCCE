
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

from vcc.utils import OutFiles, create_dictcycle_from_jsonfile
from vcc.vccobj import VCCycle
from components import compdict
from ncomponents.condenser_insulated_duct import CondenserInsulatedDcut


if __name__ == "__main__":
    print(compdict)
    compdict["CONDENSER_INSULATED_DUCT"] = CondenserInsulatedDcut
    print(compdict)
    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'

    # 1 VC heat pump 
    json_filename = curpath+'\\'+'./jsonmodel/vchp_ohio_48_combined.json'
 
    thedictcycle = create_dictcycle_from_jsonfile(json_filename)
    cycle = VCCycle(thedictcycle)
    cycle.simulator()
    OutFiles(cycle)
    ResultFileName = ResultFilePath+thedictcycle['name']
    OutFiles(cycle, ResultFileName + '.txt')

   
   