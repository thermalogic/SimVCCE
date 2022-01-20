
"""

ncomponents.insulated_duct

"""

import sys
import os
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../')

from ncomponents.insulated_duct import Insulated_Duct

if __name__ == "__main__":
    dictduct = {"in": {"p": 100.0, "t": 15.0, "vdot": 8.0},
                "out": {"p": 95.0},
                "Qin": 3.69
                }
    duct = Insulated_Duct(dictduct)
    duct.get_outt()
    print(duct)

    