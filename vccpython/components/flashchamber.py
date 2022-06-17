
"""
class FlashChamber
                         ↓   iPort 
                   ┌─────┴─────┐
                   │           │
                   │           │→  oPortV
                   │────────── │
                   └─────┬─────┘
                         ↓  oPortL 
json example:
      {
            "name": "flashchamber",
            "devtype": "FLASH_CHAMBER",
            "iPort": {
                "p": 0.6
            },
            "oPortL": {
                "x": 0.0
            },
            "oPortV": {  "x": 1.0
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
        self.iPort = Port(dictDev['iPort'])
        self.oPortV = Port(dictDev['oPortV'])
        self.oPortL = Port(dictDev['oPortL'])

    def state(self):
        if self.iPort.p is not None:
            self.oPortV.p = self.iPort.p
            self.oPortL.p = self.oPortL.p
            
        elif self.oPortL.p is not None:
            self.oPortV.p = self.oPortV.p
            self.iPort.p = self.iPort.p
       
        elif self.oPortV.p is not None:
            self.oPortL.p = self.oPortL.p
            self.iPort.p = self.iPort.p
      
    def balance(self):
        """flash chamber """
        oPortV_fdot =self.iPort.x
        oPortL_fdot =1.0-self.iPort.x
        self.oPortV.mdot = self.iPort.mdot*oPortV_fdot
        self.oPortL.mdot = self.iPort.mdot*oPortL_fdot
    
       
    def __str__(self):
        result = '\n' + self.name
        result += '\n' + "  PORT  " + Port.title
        result += '\n' + "  iPort "+self.iPort.__str__()
        result += '\n' + " oPortV " + self.oPortV.__str__()
        result += '\n' + " oPortL " + self.oPortL.__str__()
        return result
