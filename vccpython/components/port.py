"""
General Object-oriented Abstraction of VC Cycle 
  The port of device：
       dictNode: {"p","t","x","mdot","refrigerant"}
       the default refrigerant  is Port.cycle_refrigerant

 Author: Cheng Maohua cmh@seu.edu.cn
"""
from phyprops.prop_coolprop import *

class Port:
    cycle_refrigerant = 'R134a'
    
    title = ('{:^6} \t{:<8} \t{:>8} \t{:>10} \t{:>10} \t{:^10} \t{:>10}'.format
             ("Index", "P(MPa)", "T(°C)", "H(kJ/kg)", "S(kJ/kg.K)",  "Quality", "MDOT(kg/s)"))

    def __init__(self, dictnode):
        """ create the node object"""
        self.index = None
        if ("refrigerant" in dictnode):
           self.refrigerant = dictnode["refrigerant"]
        else:
           self.refrigerant = Port.cycle_refrigerant
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
        # step1 state : input values
        if self.t is not None and self.x is not None:
            self.tx()
        elif self.p is not None and self.x is not None:
            self.px()
        elif self.p is not None and self.t is not None:
            self.pt()
  
    def tx(self):
        try:
            self.p = tx_p(self.t, self.x, self.refrigerant)
            self.h = tx_h(self.t, self.x, self.refrigerant)
            self.s = tx_s(self.t, self.x, self.refrigerant)
            self.stateok = True
        except:
            self.stateok = False

    def px(self):
        try:
            self.t = px_t(self.p, self.x, self.refrigerant)
            self.h = px_h(self.p, self.x, self.refrigerant)
            self.s = px_s(self.p, self.x, self.refrigerant)
            self.stateok = True
        except:
            self.stateok = False

    def pt(self):
        try:
            self.h = pt_h(self.p, self.t, self.refrigerant)
            self.s = pt_s(self.p, self.t, self.refrigerant)
            self.stateok = True
        except:
            self.stateok = False

    def ps(self):
        try:
            if self.h is None:
                self.h = ps_h(self.p, self.s, self.refrigerant)
            if self.t is None:
                self.t = ps_t(self.p, self.s, self.refrigerant)
            if self.x is None:
                self.x = ps_x(self.p, self.s, self.refrigerant)
            self.stateok = True
        except:
            self.stateok = False

    def ph(self):
        try:
            if self.s is None:
                self.s = ph_s(self.p, self.h, self.refrigerant)
            if self.t is None:
                self.t = ph_t(self.p, self.h, self.refrigerant)
            if self.x is None:
                self.x = ph_x(self.p, self.h, self.refrigerant)
            self.stateok = True
        except:
            self.stateok = False

    def state(self):
        """ step3 state: after obtain the new parameter pairs """
        if self.stateok == False:
            if self.p is not None and self.s is not None:
                self.ps()
            elif self.p is not None and self.h is not None:
                self.ph()
            elif self.p is not None and self.t is not None:
                self.pt()

    def __str__(self):
        try:
           result = '{:^6}'.format(self.index)
        except:
            result ="-"
        OutStrs = [{"fstr": '\t{:>7.4}', 'prop': self.p, "sstr": '\t{:>7}'},
                   {"fstr": '\t{:>8.2f}', 'prop': self.t, "sstr": '\t{:>8}'},
                   {"fstr": '\t{:>10.2f}', 'prop': self.h, "sstr": '\t{:>10}'},
                   {"fstr": '\t{:>8.3f}',  'prop': self.s, "sstr": '\t{:>8}'},
                   {"fstr": '\t{:>10.4f}', 'prop': self.x, "sstr": '\t{:>10}'},
                   {"fstr": '\t{:>8.4f}',  'prop': self.mdot, "sstr": '\t{:>8}'}
                   ]

        for item in OutStrs:
            try:
                result += item["fstr"].format(item["prop"])
            except:
                result += item["sstr"].format("")

        return result
