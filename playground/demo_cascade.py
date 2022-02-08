
"""
General Object-oriented Abstraction of VC Cycle 

Yunus A. Cengel, Michael A. Boles,Thermodynamics: An Engineering Approach, 8th Edition, McGraw-Hill,2015

EXAMPLE 11–4: The Cascade Refrigeration Cycle,Page625-626

The cascade Cycle
 
lower condenser(Qout) -> upper evaporator(Qin)

 Author: Cheng Maohua cmh@seu.edu.cn
"""
import sys
import os
import json
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')

from vcc.utils import OutFiles
from vcc.vccobj import VCCycle

if __name__ == "__main__":
    curpath = os.path.abspath(os.path.dirname(__file__))

    # 1 the upper cycle
    json_filename = curpath+'\\'+'./jsonmodel/vcr_cascade_upper_11_4.json'

    with open(json_filename, 'r') as f:
        thedictcycle_upper = json.loads(f.read())

    cycle_upper = VCCycle(thedictcycle_upper)
    cycle_upper.simulator()
    OutFiles(cycle_upper)

    # 2 cascade: lower condenser(Qout) -> upper evaporator(Qin)
    json_filename = curpath+'\\'+'./jsonmodel/vcr_cascade_lower_11_4.json'
    with open(json_filename, 'r') as f:
        thedictcycle_lower = json.loads(f.read())

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
    print(f"\tThe mass flow rate of the lower cycle(kg/s): {cycle_lower.conns.nodes[0].mdot:.4f}")
    print(f"\tRefrigeration Capacity(kW): {cycle_lower.Qin:.2f}")
    print(f"\tHeat Transfer Rate(kW): {cycle_upper.Qout:.2f}")
    print(f"\tCompression Work(kW): {Wc:.2f}")
    print(f"\tThe coefficient of performance: {cop:.2f}")
