"""

class Connector

"""

from .port import *

class Connector:

    def __init__(self):
        self.nodes=[]
        self.index=0
       
    def getnodevalue(self, node,port):
        """ 
           the node is the connector of two ports ,so the node may get values from all of two ports
            the values is the union set of the not-none values within two ports 
        """
        if node[0].index is None and port.index is not None:
            node[0].index = port.index
        if node[0].p is None and port.p is not None:
            node[0].p = port.p
        if node[0].t is None and port.t is not None:
            node[0].t = port.t
        if node[0].h is None and port.h is not None:
            node[0].h = port.h
        if node[0].s is None and port.s is not None:
            node[0].s = port.s
        if node[0].x is None and port.x is not None:
            node[0].x = port.x
        if node[0].mdot is None and port.mdot is not None:
            node[0].mdot = port.mdot

    def AddConnector(self, tupConnector,comps):
        comp0, port0 =tupConnector[0] 
        comp1, port1 =tupConnector[1]
        # 1 get the index of port in nodes
        comps[comp0].portdict[port0][0].index = self.index
        # 2 add port0 into nodes
        self.nodes.append(comps[comp0].portdict[port0])
        # 3 join port1 info into  nodes[self.index]
        self.getnodevalue(self.nodes[self.index],comps[comp1].portdict[port1][0])
        # 4 send back nodes[self.index] to port1
        comps[comp1].portdict[port1][0] =self.nodes[self.index][0]

        self.index += 1
