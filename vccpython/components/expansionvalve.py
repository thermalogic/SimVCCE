"""
General Object-oriented Abstraction of VC Cycle 
  ExpansionValve:
 
 Author: Cheng Maohua cmh@seu.edu.cn   
"""
from .device_siso import Device_SISO

class ExpansionValve(Device_SISO):

    energy = "None"
    devtype = "EXPANSIONVALVE"

    def state(self):
        """ ideal Isenthalpic expansion """
        if self.iPort.h is not None and self.oPort.h is None:
           self.oPort.h = self.iPort.h
        elif self.oPort.h is not None and self.iPort.h is None:
            self.iPort.h = self.oPort.h
     
    