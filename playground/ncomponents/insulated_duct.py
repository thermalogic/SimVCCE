"""
 The Insulated Duct
                        Qin kw
                        ↓ 
                   ┌──────────┐
                   │          │
      oPortAir   ← ┤          │← iPortAir
                   │          │
                   └──────────┘
  dictport={"p":100.0,"t":15.0,"vdot":8.0}
     p - kPa   t -  °C  vdot - m^3/min   mdot - kg/s
"""
 
from .airport import *

class Insulated_Duct:
    """ the air flowing through the insulated duct"""
    def __init__(self, dduct):
        self.iPort = AirPort(dduct["in"])
        self.oPort = AirPort(dduct["out"])
        self.Qin = dduct["Qin"]

    def get_outt(self):
        self.oPort.t = self.iPort.t + self.Qin / \
            (self.iPort.mdot*AirPort.cp)

    def __str__(self):
        result = '\n' + "Insulated Duct(Air):"
        result += '\n\t' + "Qin(kW): {:.2f}".format(self.Qin)
        result += '\n\t' + "vdot(m^3/min): {:.2f}".format(self.iPort.vdot)
        result += '\n\t' + "mdot(kg/s): {:.4f}".format(self.iPort.mdot)
        result += '\n\t' + "inlet:"+self.iPort.__str__()
        result += '\n\t' + "outlet:"+self.oPort.__str__()
        return result


if __name__ == "__main__":
    dictduct = {"in": {"p": 100.0, "t": 15.0, "vdot": 8.0},
                "out": {"p": 95.0},
                "Qin": 3.69
                }
    duct = Insulated_Duct(dictduct)
    duct.get_outt()
    print(duct)
