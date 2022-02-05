"""
MixingChamber
                         ↑ oPort 
                   ┌─────┴─────┐
                   │           │
                 → │           │ 
          iPort0   │           │
                   └─────┬─────┘
                         ↑ iPort1 
 
 json example:
  {
            "name": "mixingchamber",
            "devtype": "MIXING_CHAMBER",
            "iPort0": {
                "x": 1.0
            },
            "iPort1": {
            },
            "oPort": {
                "p": 0.6
            }
        }
"""

from components.port import Port


class MixingChamber:
    energy = "none"
    devtype = "MIXING_CHAMBER"

    def __init__(self, dictDev):
        """
        Initializes Merge_Two2one
        """
        self.name = dictDev['name']
        self.iPort0 = Port(dictDev['iPort0'])
        self.iPort1 = Port(dictDev['iPort1'])
        self.oPort = Port(dictDev['oPort'])

    def state(self):
        if self.iPort0.p is not None:
            self.oPort.p = self.iPort0.p
            self.iPort1.p = self.iPort0.p
        elif self.iPort1.p is not None:
            self.iPort0.p = self.iPort1.p
            self.oPort.p = self.iPort1.p
        elif self.oPort.p is not None:
            self.iPort0.p = self.oPort1.p
            self.iPort1.p = self.oPort1.p

    def balance(self):
        self.oPort.mdot = self.iPort0.mdot+self.iPort1.mdot
        self.oPort.h = (self.iPort0.mdot*self.iPort0.h +
                           self.iPort1.mdot*self.iPort1.h)/self.oPort.mdot

    def __str__(self):
        result = '\n' + self.name
        result += '\n' + " PORT " + Port.title
        result += '\n' + " iPort0 "+self.iPort0.__str__()
        result += '\n' + " iPort1 " + self.iPort1.__str__()
        result += '\n' + " oPort " + self.oPort.__str__()
        return result
