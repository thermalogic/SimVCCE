
"""
General Object-oriented Abstraction of VC Cycle 

EXAMPLE 11–5: Multistage Compression Refrigeration: Page627-628

Yunus A. Cengel, Michael A. Boles. Thermodynamics: An Engineering Approach, 8th Edition, McGraw-Hill,2015

Author: Cheng Maohua cmh@seu.edu.cn
"""
import sys
import os
import json
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')


from ncomponents.flashchamber import FlashChamber
from ncomponents.mixingchamber import MixingChamber

from components import compdict
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles


if __name__ == "__main__":
    # add the newc omponents to compdict
    compdict[FlashChamber.devtype] = FlashChamber
    compdict[MixingChamber.devtype] = MixingChamber

    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'

    json_filename = curpath+'\\'+'./jsonmodel/vcr_two_stage_11_5.json'
    with open(json_filename, 'r') as f:
        thedictcycle = json.loads(f.read())
  
    cycle = VCCycle(thedictcycle)
    cycle.simulator()
    OutFiles(cycle)
    ResultFileName = ResultFilePath+thedictcycle['name']
    OutFiles(cycle, ResultFileName + '.txt')
