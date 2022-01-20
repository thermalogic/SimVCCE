"""
AirPort

"""
import CoolProp.CoolProp as cp

class AirPort:
    R = 8.314472/29
    cp = None

    def __init__(self, dictport):
        """ create the port object"""
        self.p = float(dictport['p'])
        self.t = None
        self.vdot = None
        self.mdot = None
        self.cp = None
        if ("t" in dictport):
            self.t = float(dictport['t'])
        if ("vdot" in dictport):
            self.vdot = float(dictport['vdot'])
            self.mdot = self.p*self.vdot / \
                (AirPort.R*(self.t+273.15)*60)  # PV = nRT
        if (self.p is not None and self.t is not None):
            self.cp = cp.PropsSI("CPMASS", "P", self.p*1000,
                                 "T", self.t+273.15, "AIR")/1000
            AirPort.cp = self.cp
    
    def __str__(self):
        result = "(p kPa,t °C): {:.1f} {:.1f}".format(self.p, self.t)
        return result
