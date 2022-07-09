"""
General Object-oriented Abstraction of VC Cycle 

   Components Package  : port and devices
   
 Author: Cheng Maohua cmh@seu.edu.cn    
"""

from .compressor import Compressor
from .condenser import Condenser
from .expansionvalve import ExpansionValve
from .evaporator import Evaporator
# add the new device class
from .flashchamber import FlashChamber
from .mixingchamber import MixingChamber


# ------------------------------------------------------------------------------
# compdict
#  typedev: class
#     Note: add  typedev: class to the dict after you add the new device class
# --------------------------------------------------------------------------------

compdict = {
    Compressor.devtype: Compressor,
    Condenser.devtype: Condenser,
    ExpansionValve.devtype: ExpansionValve,
    Evaporator.devtype: Evaporator,
    #  add the new device class
    FlashChamber.devtype:  FlashChamber,
    MixingChamber.devtype:  MixingChamber
}

