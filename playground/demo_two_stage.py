
"""
General Object-oriented Abstraction of VC Cycle 

EXAMPLE 11–4: Multistage Compression Refrigeration: Page704-705
Thermodynamics: An Engineering Approach, 5th Edition 
Yunus A. Cengel, Michael A. Boles, McGraw-Hill

Author: Cheng Maohua cmh@seu.edu.cn
"""
import sys
import os
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')


from ncomponents.flashchamber import FlashChamber
from ncomponents.mixingchamber import MixingChamber

from components import compdict
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles, create_dictcycle_from_jsonfile


if __name__ == "__main__":
    # add new the components to compdict
    compdict["FLASH_CHAMBER"] = FlashChamber
    compdict["MIXING_CHAMBER"] = MixingChamber

    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'

    json_filename = curpath+'\\'+'./jsonmodel/vcr_two_stage_11_4.json'

    thedictcycle = create_dictcycle_from_jsonfile(json_filename)
    cycle = VCCycle(thedictcycle)
    cycle.simulator()
    OutFiles(cycle)
    ResultFileName = ResultFilePath+thedictcycle['name']
    OutFiles(cycle, ResultFileName + '.txt')
