
"""
General Object-oriented Abstraction of VC Cycle 

Compressor:
 
 Author: Cheng Maohua cmh@seu.edu.cn   
"""
from .device_siso import Device_SISO
from phyprops.prop_coolprop import *

class Compressor(Device_SISO):
    """ compression of the refrigerant"""
    energy = "CompressionWork"
    devtype = "COMPRESSOR"

    def __init__(self, dictDev):
        """  Initializes Compressor """
        super().__init__(dictDev)
        if ("ef" in dictDev):
            try:
                self.ef = float(dictDev['ef'])
            except:
                self.ef = None
        else:
            self.ef = 1.0

        if ("Qout" in dictDev):
            self.Qout = float(dictDev["Qout"])
        else:
            self.Qout = 0.0

        if ("Wc" in dictDev):
            self.Wc = float(dictDev["Wc"])
        else:
            self.Wc = 0.0

    def state(self):
        """   if ef=1.0, Isentropic compression   """
        if self.ef == 1.0:
            self.oPort.s = self.iPort.s
        elif self.ef is None or self.ef != 1.0:
            isos = self.iPort.s
            isoh = ps_h(self.oPort.p, isos, self.oPort.refrigerant)
            if self.ef is None:
                self.ef = (isoh-self.iPort.h) / \
                    (self.oPort.h-self.iPort.h)
            elif self.ef < 1.0 and self.ef > 0.0:
                self.oPort.h = (isoh-self.iPort.h) / \
                    self.ef + self.iPort.h

    def balance(self):
        """  mass and energy balance    """
        # energy balance
        if (self.Wc !=0.0):
            self.iPort.mdot = (self.Wc-self.Qout)/(self.oPort.h - self.iPort.h)
        # mass balance
        super().mass_balance()
        # wc
        if (self.Wc == 0.0):
            self.Wc = self.iPort.mdot * \
                (self.oPort.h - self.iPort.h)+self.Qout

    def __str__(self):
        result = super().__str__()
        result += f'\nThe compressor efficiency(%): \t{self.ef:{">.2%" if self.ef is not None else ""}}'
        if (self.Qout != 0.0):
            result += f'\nThe loss heat from the surroundings(kW): \t{self.Qout:{">.2f" if type(self.Qout) is float else ""}}'
        result += f'\nCompression Work(kW): \t{self.Wc:{">.2f" if type(self.Wc) is float else ""}}'
        return result
