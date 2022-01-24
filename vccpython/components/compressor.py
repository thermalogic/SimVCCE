
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
        if ("ef" in dictDev):
            try:
                self.ef = float(dictDev['ef'])
            except:
                self.ef = None
        else:
            self.ef = 1.0
        # add your code here for the input Wc

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
         # ef
        if self.ef is None or self.ef != 1.0:
            pass  # add your code here to get the oPort state


    def balance(self):
        """  mass and energy balance    """
            
        # mass balance
        # add your code here to get the mass flow rate


        if self.iPort[0].mdot is None and self.oPort[0].mdot is None:
            raise ValueError("mdot not none")
        if self.iPort[0].mdot is not None:
            self.oPort[0].mdot = self.iPort[0].mdot
        elif self.oPort[0].mdot is not None:
            self.iPort[0].mdot = self.oPort[0].mdot

        # energy balance
        # add your code here when Wc is known

        self.Wc = self.iPort[0].mdot * (self.oPort[0].h - self.iPort[0].h)

    def __str__(self):
        result = '\n' + self.name
        result += '\n' + " PORTS "+Port.title
        result += '\n' + " iPort " + self.iPort[0].__str__()
        result += '\n' + " oPort " + self.oPort[0].__str__()
        try:
            result += '\nThe compressor efficiency(%): \t{:>.2f}'.format(
                self.ef*100.0)
            result += '\nWc(kW): \t{:>.2f}'.format(self.Wc)
        except:
            pass
        return result
