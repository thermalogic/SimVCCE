
"""
General Object-oriented Abstraction of VC Cycle 
   Condenser: Isobaric heat rejection
 
 Author: Cheng Maohua cmh@seu.edu.cn
"""
from .port import *

class Condenser:

    energy = "QOUT"
    devtype = "CONDENSER"

    def __init__(self, dictDev):
        """ Initializes the condenser """
        self.name = dictDev['name']
        self.iPort = [Port(dictDev['iPort'])]
        self.oPort = [Port(dictDev['oPort'])]
        # map the port's name(str) to the obj
        self.portdict = {
            "iPort": self.iPort,
            "oPort": self.oPort
        }

    def state(self):
        """ Isobaric """
        if self.oPort[0].p is not None:
           self.iPort[0].p = self.oPort[0].p
        elif self.iPort[0].p is not None:
           self.oPort[0].p = self.iPort[0].p
  

    def balance(self):
        """ mass and energy balance of the condenser  """
        if self.iPort[0].mdot is not None:
            self.oPort[0].mdot = self.iPort[0].mdot
        elif self.oPort[0].mdot is not None:
            self.iPort[0].mdot = self.oPort[0].mdot

        self.Qout = self.iPort[0].mdot*(self.iPort[0].h-self.oPort[0].h)

    def __str__(self):
        result = '\n' + self.name
        result += '\n' +" PORTS "+Port.title
        result += '\n' +" iPort "+ self.iPort[0].__str__()
        result += '\n' +" oPort "+ self.oPort[0].__str__()
        result += '\nQout(kW): \t{:>.2f}'.format(self.Qout)
        return result
