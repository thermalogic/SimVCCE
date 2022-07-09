
"""
General Object-oriented Abstraction of VC Cycle 

 Compressor:
      if ef=1.0, Isentropic compression 
 
 Author: Cheng Maohua cmh@seu.edu.cn   
"""
from .device_siso import Device_SISO


class Compressor(Device_SISO):
    """ compression of the refrigerant"""
    energy = "CompressionWork"
    devtype = "COMPRESSOR"

    def __init__(self, dictDev):
        """  Initializes  """
        super().__init__(dictDev)
        if ("ef" in dictDev):
            try:
                self.ef = float(dictDev['ef'])
            except:
                self.ef = None
        else:
            self.ef = 1.0
        # add your code here for the input Wc
        pass

    def state(self):
        """
            if ef=1.0, Isentropic compression 
        """
        if self.ef == 1.0:
            self.oPort.s = self.iPort.s
         # ef
        if self.ef is None or self.ef != 1.0:
            pass  # add your code here to get the oPort state

    def balance(self):
        """  mass and energy balance    """

        # add your code here to get the mass flow rate
        pass

        # mass balance
        super().mass_balance()
        # energy balance

        # add your code here when Wc is known
        pass
        self.Wc = self.iPort.mdot * (self.oPort.h - self.iPort.h)

    def __str__(self):
        result = super().__str__()
        result += f'\nThe compressor efficiency(%): \t{self.ef:{">.2%" if self.ef is not None else ""}}'
        result += f'\nWc(kW): \t{self.Wc:{">.2f" if type(self.Wc) is float else ""}}'
        return result

    def __iter__(self):
        """ the dict of the object """
        dictobj = {'name': self.name,
                   'iPort': dict(self.iPort),
                   'oPort': dict(self.oPort),
                   'ef': self.ef,
                   'wc': self.Wc
                   }

        for key, value in dictobj.items():
            yield (key, value)    
