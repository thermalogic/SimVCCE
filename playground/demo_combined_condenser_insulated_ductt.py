
"""
General Object-oriented Abstraction of VC Cycle 

ncomponents.condenser_insulated_duct

Author: Cheng Maohua cmh@seu.edu.cn

"""

import sys
import os
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')

from ncomponents.condenser_insulated_duct import *

if __name__ == "__main__":
    dict_cdduct = {
        "name": "condenser_duct",
        "devtype": "CONDENSER_INSULATED_DUCT",
        "iPort": {"p": 1.0, "t": 60.0, "mdot": 0.0184},
        "oPort": {"p": 1.0, "t": 30},
        "iPortAir": {"p": 100.0, "t": 15.0, "vdot": 8.0},
        "oPortAir": {"p": 95.0}
    }
    cdduct = CondenserInsulatedDcut(dict_cdduct)
    cdduct.state()
    cdduct.balance()
    print(cdduct)
