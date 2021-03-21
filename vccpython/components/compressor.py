
from .port import *
import CoolProp.CoolProp as cp


class Compressor:
    """ compression of the refrigerant"""
    energy = "CompressionWork"
    devtype = "COMPRESSOR"

    def __init__(self, dictDev):
        """
        Initializes 
        """
        self.name = dictDev['name']
        self.iPort = [Port(dictDev['iPort'])]
        self.oPort = [Port(dictDev['oPort'])]
        try:
            self.ef = float(dictDev['ef'])
        except:
            self.ef = None

        self.Wc = None
        # map the name of port to the port obj
        self.portdict = {
            "iPort": self.iPort,
            "oPort": self.oPort
        }

    def state(self):
        """
        compression of the refrigeran
        """
        if self.ef == 1.0:
            self.oPort[0].s = self.iPort[0].s
        self.isos = self.iPort[0].s

    def balance(self):
        """  mass and energy balance    """
        # mass balance
        if self.iPort[0].mdot is not None:
            self.oPort[0].mdot = self.iPort[0].mdot
        elif self.oPort[0].mdot is not None:
            self.iPort[0].mdot = self.oPort[0].mdot
        # energy
        if self.ef != 1.0 or self.ef is None:
            self.isoh = cp.PropsSI('H', 'P', self.oPort[0].p*1.0e6, 'S',
                                   self.isos*1000, 'R134a')/1000
            if self.ef is None:
                self.ef = (self.isoh-self.iPort[0].h) / \
                    (self.oPort[0].h-self.iPort[0].h)
            elif self.ef < 1.0 and self.ef > 0.0:
                self.oPort[0].h = (
                    self.isoh-self.iPort[0].h)/self.ef + self.iPort[0].h
                self.oPort[0].state()
        # wc
        self.Wc = self.iPort[0].mdot * (self.oPort[0].h - self.iPort[0].h)

    def __str__(self):
        result = '\n' + self.name
        result += '\n' +" PORTS "+Port.title
        result += '\n' +" iPort "+ self.iPort[0].__str__()
        result += '\n' +" oPort "+ self.oPort[0].__str__()
        try:
            result += '\nThe compressor efficiency(%): \t{:>.2f}'.format(
                self.ef*100.0)
        except:
            pass
        result += '\nWc(kW): \t{:>.2f}'.format(self.Wc)
        return result
