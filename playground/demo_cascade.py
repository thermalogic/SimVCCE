
"""
General Object-oriented Abstraction of VC Cycle 
  
 The cascade Cycle
 
 lower condenser(Qout) -> upper evaporator(Qin)

 Majid Bahrami. ENSC461(S11)：Energy Conversion,Simon Fraser University,Canada
 
 Author: Cheng Maohua cmh@seu.edu.cn
"""

import sys
import os
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')


from vcc.vccobj import VCCycle
from vcc.utils import OutFiles, create_dictcycle_from_jsonfile

from platform import os

if __name__ == "__main__":
    curpath = os.path.abspath(os.path.dirname(__file__))
    
    # 1 the upper cycle 
    json_filename = curpath+'\\'+'./jsonmodel/vcr_cascade_upper.json'
    
    thedictcycle_upper = create_dictcycle_from_jsonfile(json_filename)
    cycle_upper = VCCycle(thedictcycle_upper)
    cycle_upper.simulator()
    OutFiles(cycle_upper)
    
    # 2 cascade: lower condenser(Qout) -> upper evaporator(Qin)
    json_filename_lower = curpath+'\\'+'./jsonmodel/vcr_cascade_lower.json'
    thedictcycle_lower = create_dictcycle_from_jsonfile(json_filename_lower)
    for device in thedictcycle_lower["components"]:
        if device["name"] == "Condenser":
           device["Qout"] = cycle_upper.Qin
           break 
    
    # 3 the lower cycle
    cycle_lower = VCCycle(thedictcycle_lower)
    cycle_lower.simulator()
    OutFiles(cycle_lower)

    # 4 the cascade cycle
    Wc = cycle_upper.Wc+cycle_lower.Wc
    cop = cycle_lower.Qin/Wc   
    
    print("\nThe Cascade Refrigeration Cycle")
    print("\tThe mass flow rate of the lower cycle(kg/s): {:.4f}".format(cycle_lower.conns.nodes[0][0].mdot))
    print("\tRefrigeration Capacity(kW): {: .2f}".format(cycle_lower.Qin))
    print("\tHeat Transfer Rate(kW): {: .2f}".format(cycle_upper.Qout))
    print("\tCompression Work(kW): {:.2f}".format(Wc))
    print("\tThe coefficient of performance: {:.2f}".format(cop))
   