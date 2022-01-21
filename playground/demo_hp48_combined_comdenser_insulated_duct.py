
"""
General Object-oriented Abstraction of VC Cycle 

The heat pump system:

* heat -> evaporator

* condenser -> heat -> the air flowing through the insulated duct

Author: Cheng Maohua cmh@seu.edu.cn
"""

from ncomponents.condenser_insulated_duct import CondenserInsulatedDcut
from components import compdict
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles, create_dictcycle_from_jsonfile
import sys
import os
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')


if __name__ == "__main__":
    # add new the components to compdict
    compdict["CONDENSER_INSULATED_DUCT"] = CondenserInsulatedDcut

    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'

    json_filename = curpath+'\\'+'./jsonmodel/vchp_ohio_48_combined.json'

    thedictcycle = create_dictcycle_from_jsonfile(json_filename)
    cycle = VCCycle(thedictcycle)
    cycle.simulator()
    OutFiles(cycle)
    ResultFileName = ResultFilePath+thedictcycle['name']
    OutFiles(cycle, ResultFileName + '.txt')
