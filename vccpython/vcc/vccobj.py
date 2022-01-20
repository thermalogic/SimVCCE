"""
General Object-oriented Abstraction of VC Cycle 

 class VCCycle: the Simulator class of VC Cycle  

 Author: Cheng Maohua cmh@seu.edu.cn
"""

import time
import getpass

from components.port import Port
from components import compdict
from .connector import Connector

class VCCycle:

    def __init__(self, dictcycle):
        """
          dictcycle={"name":namestring,
                     "refrigerant":refrigerantstring,
                     "components":[{component1},{component2},...],
                     "connectors":[((name1,port1),(name2,port2)),...]
                  }
          TO:     
             self.comps : dict of all component objects      
             self.conns : the connector object
        """
        self.name = dictcycle["name"]
        self.cycle_refrigerant=dictcycle["refrigerant"]
        Port.cycle_refrigerant = self.cycle_refrigerant

        dictcomps = dictcycle["components"]
        listconnectors = dictcycle["connectors"]

        # 1 convert dict to the dict of device objects: {device name:device obiect}
        self.comps = {}
        for curdev in dictcomps:
            self.comps[curdev['name']] = compdict[curdev['devtype']](curdev)

        self.conns = Connector()
        # 2 use the listconnectors to set the nodes value and alias between the item of nodes and the port of devices
        for tupconnector in listconnectors:
            self.conns.add_node(tupconnector, self.comps)

        self.Wc = None
        self.cop = None

    def __port_state(self):
        """ calculate the state of ports """
        # 1 the port state of devices: step2 state
        for key in self.comps:
            self.comps[key].state()

        # 2 the node state of connectors: step3 state
        for item in self.conns.nodes:
            if item[0].stateok == False:
                item[0].state()

    def __component_balance(self):
        for curdev in self.comps:
            self.comps[curdev].balance()

    def simulator(self):
        self.__port_state()
        self.__component_balance()

        self.Wc = 0
        self.Qin = 0
        self.Qout = 0

        for key in self.comps:
            if self.comps[key].energy == "CompressionWork":
                self.Wc += self.comps[key].Wc
            elif self.comps[key].energy == "QIN":
                self.Qin += self.comps[key].Qin
            elif self.comps[key].energy == "QOUT":
                self.Qout += self.comps[key].Qout


        self.cop = self.Qin / self.Wc
        self.cop_hp = self.Qout / self.Wc

    def __setformatstr(self, formatstr, result):
        result += formatstr.format('Compression Work(kW): ', self.Wc)
        result += formatstr.format('Refrigeration Capacity(kW): ', self.Qin)
        result += formatstr.format('\tCapacity(ton): ',self.Qin*60*(1/211))
        result += formatstr.format('The heat transfer rate(kW): ', self.Qout)
        result += formatstr.format('The coefficient of performance: ', self.cop)
        result += formatstr.format('The coefficient of performance(heat pump):', self.cop_hp)
        return result

    def __str__(self):
        str_curtime = time.strftime(
            "%Y/%m/%d %H:%M:%S", time.localtime(time.time()))
        result = "\nThe Vapor-Compression Cycle: {} ( {} by {})\n".format(
            self.name, str_curtime, getpass.getuser())
        result +="\nRefrigerant: {}\n".format(self.cycle_refrigerant)
        try:
            result = self.__setformatstr("{:>35} {:>5.2f}\n", result)
        except:
            result = self.__setformatstr("{} {}\n", result)
        return result
