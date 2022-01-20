"""
The combined condenser and insulated duct

                         ↓   iPort refrigerant
                   ┌─────┼─────┐
                   │  ←┌─┼─┐   │
      oPortAir   ← ┤ Q │ Z │ Q │← iPortAir
                   │  ←└─┼─┘   │
                   └─────┼─────┘
                         ↓      oPort refrigerant
  
 port air :{"p":100.0,"t":15.0,"vdot":8.0}
     p - kPa   t -  °C  vdot - m^3/min   mdot - kg/s
"""

import CoolProp.CoolProp as cp
from components.port import Port
from .airport import *

class CondenserInsulatedDcut:

    energy = "QOUT"
    devtype = "CONDENSER_INSULATED_DUCT"

    def __init__(self, dictDev):
        """ Initializes the condenser """
        self.name = dictDev['name']
        self.iPort = [Port(dictDev['iPort'])]
        self.oPort = [Port(dictDev['oPort'])]
        self.iPortAir = AirPort(dictDev["iPortAir"])
        self.oPortAir = AirPort(dictDev["oPortAir"])
     
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
        # 
        if self.iPort[0].mdot is not None:
            self.oPort[0].mdot = self.iPort[0].mdot
        elif self.oPort[0].mdot is not None:
            self.iPort[0].mdot = self.oPort[0].mdot
        self.Qout = self.iPort[0].mdot*(self.iPort[0].h-self.oPort[0].h)
        # air flows
        self.oPortAir.t = self.iPortAir.t + self.Qout / \
            (self.iPortAir.mdot*AirPort.cp)


    def __str__(self):
        result = '\n' + self.name
        result += '\n' + " PORTS "+Port.title
        result += '\n' + " iPort " + self.iPort[0].__str__()
        result += '\n' + " oPort " + self.oPort[0].__str__()
        result += '\nQout(kW): \t{:>.2f}'.format(self.Qout)
        result += '\n' + "Insulated Duct(Air):"
        result += '\n\t' + "vdot(m^3/min): {:.2f}".format(self.iPortAir.vdot)
        result += '\n\t' + "mdot(kg/s): {:.4f}".format(self.iPortAir.mdot)
        result += '\n\t' + "inlet:"+self.iPortAir.__str__()
        result += '\n\t' + "outlet:"+self.oPortAir.__str__()
        return result

if __name__ == "__main__":
    dict_cdduct=    {
        "name": "Condenser_duct",
        "devtype": "CONDENSER_INSULATED_DUCTs",
        "iPort": {"p":1.0,"t":60.0,"mdot":0.0184},
        "oPort": {"p": 1.0, "t": 30},
        "iPortAir": {"p": 100.0, "t": 15.0, "vdot": 8.0},
        "oPortAir": {"p": 95.0}
    }
    cdduct = CondenserInsulatedDcut(dict_cdduct)
    cdduct.state()
    cdduct.balance()
    print(cdduct)

