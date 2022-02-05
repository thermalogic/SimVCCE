
"""
General Object-oriented Abstraction of VC Cycle 
   Condenser: heat rejection
 
 Author: Cheng Maohua cmh@seu.edu.cn
"""
from .device_siso import Device_SISO

class Condenser(Device_SISO):

    energy = "QOUT"
    devtype = "CONDENSER"

    def __init__(self, dictDev):
        """ Initializes the condenser """
        super().__init__(dictDev)
        if ("Qout" in dictDev):
            self.Qout = float(dictDev["Qout"])
        else:
            self.Qout = None
      
    def state(self):
        """ ideal  Isobaric """
        if self.oPort.p is not None and self.iPort.p is None:
            self.iPort.p = self.oPort.p
        elif self.iPort.p is not None and self.oPort.p is None:
            self.oPort.p = self.iPort.p
    
    
    def balance(self):
        """ mass and energy balance of the condenser  """
        if self.Qout is not None:
            self.iPort.mdot = self.Qout/(self.iPort.h-self.oPort.h)
   
        super().mass_balance()
        
        if self.Qout is None:
            self.Qout = self.iPort.mdot*(self.iPort.h-self.oPort.h)

    def __str__(self):
        result = super().__str__()
        result += f'\nQout(kW): \t{self.Qout:{">.2f" if type(self.Qout) is float else ""}}'
        return result
