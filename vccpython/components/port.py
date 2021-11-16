"""
 The port of device
"""
import CoolProp.CoolProp as cp


class Port:

    coolant = 'R134a'
    title = ('{:^6} \t{:<8} \t{:>8} \t{:>10} \t{:>10} \t{:^10} \t{:>10}'.format
             ("Index", "P(MPa)", "T(°C)", "H(kJ/kg)", "S(kJ/kg.K)",  "Quality", "MDOT(kg/s)"))
        
    def __init__(self, dictnode):
        """ create the node object"""
        self.coolant=Port.coolant
        self.index = None
        self.p = None
        self.t = None
        self.x = None
        self.mdot = None

        if('p' in dictnode):
            try:
                self.p = float(dictnode['p'])
            except:
                pass

        if ('t' in dictnode):
            try:
                self.t = float(dictnode['t'])
            except:
                pass

        if ('x' in dictnode):
            try:
                self.x = float(dictnode['x'])
            except:
                pass

        if ('mdot' in dictnode):
            try:
                self.mdot = float(dictnode['mdot'])
            except:
                pass

        self.h = None
        self.s = None
        self.stateok = False

        if self.t is not None and self.x is not None:
            self.tx()
        elif self.p is not None and self.x is not None:
            self.px()
        elif self.p is not None and self.t is not None:
            self.pt()
    def tx(self):
        try:
            self.p = cp.PropsSI('P', 'T', 273.15+self.t,
                                'Q', self.x, self.coolant)/1.0e6
            self.h = cp.PropsSI('H', 'T', 273.15+self.t,
                                'Q', self.x, self.coolant)/1000
            self.s = cp.PropsSI('S', 'T', 273.15+self.t,
                                'Q', self.x, self.coolant)/1000
            self.stateok = True
        except:
            self.stateok = False

    def px(self):
        try:
            self.t = cp.PropsSI('T', 'P', self.p*1.0e6,
                                'Q', self.x, self.coolant)-273.15
            self.h = cp.PropsSI('H', 'P', self.p*1.0e6,
                                'Q', self.x, self.coolant)/1000
            self.s = cp.PropsSI('S', 'P', self.p*1.0e6,
                                'Q', self.x, self.coolant)/1000
            print(self.t, self.h, self.s)

            self.stateok = True
        except:
            self.stateok = False

    def pt(self):
        try:
            self.h = cp.PropsSI('H', 'P', self.p*1.0e6, 'T',
                                self.t+273.15, self.coolant)/1000
            self.s = cp.PropsSI('S', 'P', self.p*1.0e6, 'T',
                                self.t+273.15, self.coolant)/1000
            self.x = cp.PropsSI('Q', 'P', self.p*1.0e6,
                                'H', self.h*1000, self.coolant)
            if self.x == -1:
                self.x = None
            self.stateok = True
        except:
            self.stateok = False

    def ps(self):
        try:
            if self.h is None:
                self.h = cp.PropsSI('H', 'P', self.p*1.0e6, 'S',
                                    self.s*1000, self.coolant)/1000
            if self.t is None:
                self.t = cp.PropsSI('T', 'P', self.p*1.0e6, 'S',
                                    self.s*1000, self.coolant)-273.15
            if self.x is None:
                self.x = cp.PropsSI('Q', 'P', self.p*1.0e6, 'S',
                                    self.s*1000, self.coolant)
                if self.x == -1:
                    self.x = None
            self.stateok = True
        except:
            self.stateok = False

    def ph(self):
        try:
            if self.s is None:
                self.s = cp.PropsSI('S', 'P', self.p*1.0e6, 'H',
                                    self.h*1000, self.coolant)/1000
            if self.t is None:
                self.t = cp.PropsSI('T', 'P', self.p*1.0e6, 'H',
                                    self.h*1000, self.coolant)-273.15
            if self.x is None:
                self.x = cp.PropsSI('Q', 'P', self.p*1.0e6, 'H',
                                    self.h*1000, self.coolant)
                if self.x == -1:
                    self.x = None
            self.stateok = True
        except:
            self.stateok = False

    def state(self):
        if self.stateok == False:
            if self.p is not None and self.s is not None:
                self.ps()
            elif self.p is not None and self.h is not None:
                self.ph()
            elif self.p is not None and self.t is not None:
                self.pt()

    def __str__(self):
        result = '{:^6}'.format(self.index)
        OutStrs = [{"fstr": '\t{:>7.4}', 'prop': self.p, "sstr": '\t{:>7}'},
                   {"fstr": '\t{:>8.2f}', 'prop': self.t, "sstr": '\t{:>8}'},
                   {"fstr": '\t{:>10.2f}', 'prop': self.h, "sstr": '\t{:>10}'},
                   {"fstr": '\t{:>8.3f}',  'prop': self.s, "sstr": '\t{:>8}'},
                   {"fstr": '\t{:>10.4f}', 'prop': self.x, "sstr": '\t{:>10}'},
                   {"fstr": '\t{:>8.2f}',  'prop': self.mdot, "sstr": '\t{:>8}'}
                   ]

        for item in OutStrs:
            try:
                result += item["fstr"].format(item["prop"])
            except:
                result += item["sstr"].format("")

        return result
