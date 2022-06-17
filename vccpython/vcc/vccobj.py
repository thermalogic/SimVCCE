"""
General Object-oriented Abstraction of VC Cycle 

 class VCCycle: the Simulator class of VC Cycle  

 Author: Cheng Maohua cmh@seu.edu.cn
"""

from time import time, localtime, strftime
from getpass import getuser

from components import compdict
from components.port import Port
from .connector import Connector


class VCCycle:

    def __init__(self, dictcycle):
        """
          dictcycle={"name":namestring,
                     "refrigerant":refrigerantstring,
                     "components":[{component1},{component2},...],
                     "connectors":{"name1.port1":"name2.port2",...}
                  }
          TO:     
             self.comps : dict of all component objects      
             self.conns : the connector object
        """
        self.name = dictcycle["name"]
        self.cycle_refrigerant = dictcycle["refrigerant"]
        Port.cycle_refrigerant = self.cycle_refrigerant

        # 1 convert dict to the dict of device objects: {device name:device obiect}
        self.comps = {}
        for curdev in dictcycle["components"]:
            self.comps[curdev['name']] = compdict[curdev['devtype']](curdev)

        # 2 set the nodes value and alias between the item of nodes and the port of devices
        self.conns = Connector(dictcycle["connectors"], self.comps)
    
    def __component_simulator(self):
        """ calculate the state of ports """
        # the ports state of device
        for key in self.comps:
            self.comps[key].state()

        # the nodes state of connectors
        for item in self.conns.nodes:
            if item.stateok == False:
                item.state()

        for curdev in self.comps:
            self.comps[curdev].balance()
    
    def simulator(self):
        self.__component_simulator()

        self.Wc = 0.0
        self.Qin = 0.0
        self.Qout = 0.0

        for key in self.comps:
            if self.comps[key].energy == "CompressionWork":
                self.Wc += self.comps[key].Wc
            elif self.comps[key].energy == "QIN":
                self.Qin += self.comps[key].Qin
            elif self.comps[key].energy == "QOUT":
                self.Qout += self.comps[key].Qout

        self.cop = self.Qin / self.Wc
        self.cop_hp = self.Qout / self.Wc

    def __str__(self):
        curtime = strftime("%Y/%m/%d %H:%M:%S", localtime(time()))
        result = f"\nThe Vapor-Compression Cycle: {self.name} ({curtime} by {getuser()})\n"
        result += f"\nRefrigerant: {self.cycle_refrigerant}\n"
        
        rusult_items = {'Compression Work(kW): ': self.Wc,
                        'Refrigeration Capacity(kW): ': self.Qin,
                        '\tCapacity(ton): ': self.Qin*60*(1/211),
                        'The heat transfer rate(kW): ': self.Qout,
                        'The coefficient of performance: ': self.cop,
                        'The coefficient of performance(heat pump):': self.cop_hp}
        for name, value in rusult_items.items():
            result += f'{name:>35} {value:{">5.2f" if type(value) is float else ""}}\n'
        return result
