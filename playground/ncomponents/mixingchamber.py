"""
MixingChamber
                         ↑ oPort 
                   ┌─────┴─────┐
                   │           │
                 → │           │  oPortV
          iPort0   │           │
                   └─────┬─────┘
                         ↑ iPort 
 
 json object example:
  {
            "name": "mixingchamber",
            "devtype": "MIXING_CHAMBER",
            "iPort0": {
                "x": 1
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
        self.iPort0 = [Port(dictDev['iPort0'])]
        self.iPort1 = [Port(dictDev['iPort1'])]
        self.oPort = [Port(dictDev['oPort'])]

        # map the name of port to the port obj
        self.portdict = {
            "iPort0": self.iPort0,
            "iPort1": self.iPort1,
            "oPort": self.oPort
        }

    def state(self):
        if self.iPort0[0].p is not None:
            self.oPort[0].p = self.iPort0[0].p
            self.iPort1[0].p = self.iPort0[0].p
        elif self.iPort1[0].p is not None:
            self.iPort0[0].p = self.iPort1[0].p
            self.oPort[0].p = self.iPort1[0].p
        elif self.oPort[0].p is not None:
            self.iPort0[0].p = self.oPort1[0].p
            self.iPort1[0].p = self.oPort1[0].p
           
    def balance(self):
        self.oPort[0].mdot = self.iPort0[0].mdot+self.iPort1[0].mdot
        self.oPort[0].h = (self.iPort0[0].mdot*self.iPort0[0].h +
                           self.iPort1[0].mdot*self.iPort1[0].h)/self.oPort[0].mdot
    
   
    def __str__(self):
        result = '\n' + self.name
        result += '\n' + " PORT " + Port.title
        result += '\n' + " iPort0 "+self.iPort0[0].__str__()
        result += '\n' + " iPort1 " + self.iPort1[0].__str__()
        result += '\n' + " oPort " + self.oPort[0].__str__()
        return result
