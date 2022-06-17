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
    w = 12
    title = f'{"Index":^6s} {"P(MPa)":^{w}s} {"T(°C)":^{w}s} {"H(kJ/kg)":^{w}s} {"S(kJ/kg.K)":^{w}s} {"Quality":^{w}s} {"MDOT(kg/s)":^{w}s}'

    pairs = {('p', 't'): {'h': pt_h, 's': pt_s},
             ('t', 'x'): {'p': tx_p, 'h': tx_h, 's': tx_s},
             ('p', 'x'): {'t': px_t, 'h': px_h, 's': px_s},
             ('p', 's'): {'h': ps_h, 't': ps_t, 'x': ps_x},
             ('p', 'h'): {'s': ph_s, 't': ph_t, 'x': ph_x}}

    def __init__(self, dictnode):
        """ create the node object"""
        self.index = None
        if ("refrigerant" in dictnode):
           self.refrigerant = dictnode["refrigerant"]
        else:
           self.refrigerant = Port.cycle_refrigerant
        self.p = None
        self.t = None
        self.h = None
        self.s = None
        self.x = None
        self.mdot = None
        # update the instance attributes with  dictnode
        self.__dict__.update(dictnode)

        # step1 state : input values
        self.stateok = False
        self.state()

    def state(self):
        if self.stateok == False:
            for pair, keyfun in Port.pairs.items():
                v0 = self.__dict__[pair[0]]
                v1 = self.__dict__[pair[1]]
                if v0 is not None and v1 is not None:
                    stateok = True
                    # loop to get all props of (v0,v1)
                    for key, fun in keyfun.items():
                        if self.__dict__[key] is None:
                            try:
                                self.__dict__[key] = fun(v0, v1, self.refrigerant)
                            except:
                                stateok = False
                    # end loop to get all props of (v0,v1)
                    self.stateok = stateok
                    break  # exit for pair,keyfun

    def __str__(self):
        result = f'{self.index:^6d}' if type(self.index) is int else f'{3*"-":^6s}'

        out_strs = {self.p: '.3f',
                   self.t: '.2f',
                   self.h: '.2f',
                   self.s: '.3f',
                   self.x: '.3f',
                   self.mdot: '.4f'}
        for value, fstr in out_strs.items():
            result += f' {value:^{Port.w}{fstr}}' if type(value) is float else f'{5*"-":>10s}'
        return result
