
"""
General Object-oriented Abstraction of VC Cycle 

Evaporator:
    heat addition

Author: Cheng Maohua cmh@seu.edu.cn    
"""
from .device_siso import Device_SISO

class Evaporator(Device_SISO):

    energy = "QIN"
    devtype = "EVAPORATOR"

    def __init__(self, dictDev):
        """ Initializes the Evaporator """
        super().__init__(dictDev)
        self.Qin=None

    def state(self):
        """ ideal Isobaric """
        if self.oPort.p is not None and self.iPort.p is None:
            self.iPort.p = self.oPort.p
        elif self.iPort.p is not None and self.oPort.p is None:
            self.oPort.p = self.iPort.p

    def balance(self):
        """ mass and energy balance  """

        # mass balance
        super().mass_balance()

        # energy balance
        self.Qin = self.iPort.mdot * (self.oPort.h - self.iPort.h)

    def __str__(self):
        result = super().__str__()
        result += f'\nQin(kW): \t{self.Qin:{">.2f" if type(self.Qin) is float  else ""}}'
        return result
    
    def __iter__(self):
        """ the dict of the object """
        dictobj = {'name': self.name,
                   'iPort': dict(self.iPort),
                   'oPort': dict(self.oPort),
                   'Qin': self.Qin
                    }

        for key, value in dictobj.items():
            yield (key, value)    
