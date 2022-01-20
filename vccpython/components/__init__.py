"""
General Object-oriented Abstraction of VC Cycle 

   Components Package  : port and devices
   
 Author: Cheng Maohua cmh@seu.edu.cn    
"""

from .port import Port
from .compressor import Compressor
from .condenser import Condenser
from .expansionvalve import ExpansionValve
from .evaporator import Evaporator


# ------------------------------------------------------------------------------
# compdict(jump table)
#  1: key:value-> Type String: class  name
#  2    add the new key:value to the dict after you add the new device class/type
# --------------------------------------------------------------------------------

compdict = {
    "COMPRESSOR": Compressor,
    "CONDENSER": Condenser,
    "EXPANSIONVALVE": ExpansionValve,
    "EVAPORATOR": Evaporator
}
