
"""
General Object-oriented Abstraction of VC Cycle 

 Compressor:
      if ef=1.0, Isentropic compression 
 
 Author: Cheng Maohua cmh@seu.edu.cn   
"""
from .port import *
from phyprops.prop_coolprop import *


class Compressor:
    """ compression of the refrigerant"""
    energy = "CompressionWork"
    devtype = "COMPRESSOR"

    def __init__(self, dictDev):
        """
        Initializes 
        """
        self.name = dictDev['name']
        self.iPort = [Port(dictDev['iPort'])]
        self.oPort = [Port(dictDev['oPort'])]
        try:
            self.ef = float(dictDev['ef'])
        except:
            self.ef = None

        if ("win" in dictDev):
            self.Wc = float(dictDev["win"])
        else:
            self.Wc = None
        # map the port's name(str) to the obj
        self.portdict = {
            "iPort": self.iPort,
            "oPort": self.oPort
        }

    def state(self):
        """
            if ef=1.0, Isentropic compression 
        """
        self.isos = self.iPort[0].s
        if self.ef == 1.0:
            self.oPort[0].s = self.iPort[0].s
        elif self.ef != 1.0:
            # add code to obtain the oPort state
            pass

    def balance(self):
        """  mass and energy balance    """
        # ef
        if self.ef is None:
            # add code to obtain the ef
            pass
        # energy balance
        if (self.Wc is not None):
            self.iPort[0].mdot = self.Wc/(self.oPort[0].h - self.iPort[0].h)
        # mass balance
        if self.iPort[0].mdot is not None:
            self.oPort[0].mdot = self.iPort[0].mdot
        elif self.oPort[0].mdot is not None:
            self.iPort[0].mdot = self.oPort[0].mdot
        # wc
        if (self.Wc is None):
            self.Wc = self.iPort[0].mdot * (self.oPort[0].h - self.iPort[0].h)

    def __str__(self):
        result = '\n' + self.name
        result += '\n' + " PORTS "+Port.title
        result += '\n' + " iPort " + self.iPort[0].__str__()
        result += '\n' + " oPort " + self.oPort[0].__str__()
        try:
            result += '\nThe compressor efficiency(%): \t{:>.2f}'.format(
                self.ef*100.0)
        except:
            pass
        result += '\nWc(kW): \t{:>.2f}'.format(self.Wc)
        return result
