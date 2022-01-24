
"""
class FlashChamber
                         ↓   iPort 
                   ┌─────┴─────┐
                   │           │
                   │           │→  oPortV
                   │────────── │
                   └─────┬─────┘
                         ↓      oPortL 
json object example:
      {
            "name": "flashchamber",
            "devtype": "FLASH_CHAMBER",
            "iPort": {
                "p": 0.6
            },
            "oPortL": {
                "x": 0.0
            },
            "oPortV": {
            }
        },
     
"""
from components.port import Port

class FlashChamber:
    energy = "none"
    devtype = "FLASH_CHAMBER"

    def __init__(self, dictDev):
        """
        Initializes lashChamber
        """
        self.name = dictDev['name']
        self.iPort = [Port(dictDev['iPort'])]
        self.oPortV = [Port(dictDev['oPortV'])]
        self.oPortL = [Port(dictDev['oPortL'])]

        # map the name of port to the port obj
        self.portdict = {
            "iPort": self.iPort,
            "oPortV": self.oPortV,
            "oPortL": self.oPortL
        }

    def state(self):
        if self.iPort[0].p is not None:
            self.oPortV[0].p = self.iPort[0].p
            self.oPortL[0].p = self.oPortL[0].p
            
        elif self.oPortL[0].p is not None:
            self.oPortV[0].p = self.oPortV[0].p
            self.iPort[0].p = self.iPort[0].p
       
        elif self.oPortV[0].p is not None:
            self.oPortL[0].p = self.oPortL[0].p
            self.iPort[0].p = self.iPort[0].p
      
    def balance(self):
        """flash chamber """
        oPortV_fdot =self.iPort[0].x
        oPortL_fdot =1.0-self.iPort[0].x
        self.oPortV[0].mdot = self.iPort[0].mdot*oPortV_fdot
        self.oPortL[0].mdot = self.iPort[0].mdot*oPortL_fdot
    
       
    def __str__(self):
        result = '\n' + self.name
        result += '\n' + " PORT " + Port.title
        result += '\n' + " iPort "+self.iPort[0].__str__()
        result += '\n' + " oPortV " + self.oPortV[0].__str__()
        result += '\n' + " oPortL " + self.oPortL[0].__str__()
        return result
