"""
General Object-oriented Abstraction of VC Cycle 

The base class for single inlet and outlet device

 Author: Cheng Maohua cmh@seu.edu.cn   
"""
from .port import *

class Device_SISO:
    """ The base class for single inlet and outlet device"""
    energy = ""
    devtype = "SISO"

    def __init__(self, dictDev):
        """   Initializes   """
        self.name = dictDev['name']
        self.iPort = Port(dictDev['iPort'])
        self.oPort = Port(dictDev['oPort'])
     
    def state(self):
        raise Exception("override state() for the specified device")

    def mass_balance(self):
        if self.iPort.mdot is None and self.oPort.mdot is None:
            raise ValueError("mdot is None")
        if self.iPort.mdot is not None:
            self.oPort.mdot = self.iPort.mdot
        elif self.oPort.mdot is not None:
            self.iPort.mdot = self.oPort.mdot

    def balance(self):
        """  mass and energy balance    """
        self.mass_balance()

    def __str__(self):
        result = '\n' + self.name
        result += '\n' + " PORT  " + Port.title
        result += '\n' + " iPort " + self.iPort.__str__()
        result += '\n' + " oPort " + self.oPort.__str__()
        return result
