"""
 General Object-oriented Abstraction of vcr Cycle 

RefrigerationCycle: the Simulator class of VCR Cycle  


"""

import time
from platform import os
import getpass

from components.port import Port
from components.connector import Connector
from components import compdict


class VCCycle:

    def __init__(self, dictcycle):
        """
          dictcycle={"name":namestring,
                     "components":[{component1},{component2},...],
                     "connectors":[((name1,port1),(name2,port2)),...]
                  }
          TO:     
             self.comps : dict of all component objects      
             self.conns : the connector object
        """
        self.name = dictcycle["name"]
        dictcomps = dictcycle["components"]
        listconnectors = dictcycle["connectors"]

        # 1 convert dict to the dict of device objects: {device name:device obiect}
        self.DevNum = len(dictcomps)
        self.comps = {}
        for curdev in dictcomps:
            self.comps[curdev['name']] = compdict[curdev['devtype']](curdev)

        self.conns = Connector()
        # 2 use the listconnectors to set the nodes value and alias between the item of nodes and the port of devices
        for tupconnector in listconnectors:
            self.conns.add_node(tupconnector, self.comps)

        self.Wc = None
        self.Qlow =None
        self.cop = None

    def port_state(self):
        """ calculate the state of ports """
        # 1 the port state of devices
        for key in self.comps:
            self.comps[key].state()

        # 2 the node state of connectors
        for item in self.conns.nodes:
            if item[0].stateok == False:
                item[0].state()

    def component_balance(self):
        for curdev in self.comps:
            self.comps[curdev].balance()

    def simulator(self):
        self.port_state()
        self.component_balance()

        self.Wc = 0
        self.Qin = 0
        self.Qout = 0

        for key in self.comps:
            if self.comps[key].energy == "CompressionWork":
                self.Wc += self.comps[key].Wc
            elif self.comps[key].energy == "QIN":
                self.Qin += self.comps[key].Qin

        self.cop = self.Qin / self.Wc
        self.Qin = self.Qin*60*(1/211)

    def __setformatstr(self, formatstr, result):
        result += formatstr.format('Compression Work(kW): ', self.Wc)
        result += formatstr.format('Refrigeration Capacity(ton): ', self.Qin)
        result += formatstr.format('The coefficient of performance: ', self.cop)
        return result

    def __str__(self):
        str_curtime = time.strftime(
            "%Y/%m/%d %H:%M:%S", time.localtime(time.time()))
        result = "\nRefrigeration Cycle: {} (Time: {} by {} on {})\n".format(
            self.name, str_curtime, getpass.getuser(), os.popen('hostname').read())
        try:
            result = self.__setformatstr("{:>35} {:>5.2f}\n", result)
        except:
            result = self.__setformatstr("{} {}\n", result)
        return result
