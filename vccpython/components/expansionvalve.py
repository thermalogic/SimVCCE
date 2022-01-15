from .port import *


class ExpansionValve:

    energy = "None"
    devtype = "EXPANSIONVALVE"

    def __init__(self, dictDev):
        """ Initializes the ExpansionValve """
        self.name = dictDev['name']
        self.iPort = [Port(dictDev['iPort'])]
        self.oPort = [Port(dictDev['oPort'])]
        # map the port's name(str) to the obj
        self.portdict = {
            "iPort": self.iPort,
            "oPort": self.oPort
        }

    def state(self):
        self.oPort[0].h = self.iPort[0].h

    def balance(self):
        """ mass and energy balance  """
        if self.iPort[0].mdot is not None:
            self.oPort[0].mdot = self.iPort[0].mdot
        elif self.oPort[0].mdot is not None:
            self.iPort[0].mdot = self.oPort[0].mdot

    def __str__(self):
        result = '\n' + self.name
        result += '\n' +" PORTS "+Port.title
        result += '\n' +" iPort "+ self.iPort[0].__str__()
        result += '\n' +" oPort "+ self.oPort[0].__str__()
        return result
