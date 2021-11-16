"""

class Connector

"""

from .port import *

class Connector:

    def __init__(self):
        self.nodes=[]
        self.curindex=0
       
    def combined_node_value(self, node, port):
        """ 
           the node is the connector of two ports ,so the node may get values from all of two ports
            the values is the union set of the non-none values within two ports 
        """
        for key in node[0].__dict__.keys():
            nodevalue = getattr(node[0], key)
            portvalue = getattr(port[0], key)
            if nodevalue is None and portvalue is not None:
               setattr(node[0], key, portvalue)
   
    def add_node(self, tupConnector,comps):
        """ node : (("comp0", "port0"), ("comp1", "port1"))"""
        comp0, port0 =tupConnector[0] 
        comp1, port1 =tupConnector[1]
        
        # 1 get the port [] list: list is mutable!
        comp_port0 = comps[comp0].portdict[port0]
        comp_port1 = comps[comp1].portdict[port1]

        # 2 get the index of port in nodes
        comp_port0[0].index = self.curindex
        
        # 3 create the new node with comp_port0 
        self.nodes.append(comp_port0)
        
        # 4 join comp_port1 info into  nodes[self.curindex]
        curnode = self.nodes[self.curindex]
        self.combined_node_value(curnode, comp_port1)
        # 5 set the pointer of comp_port1[0] to nodes[self.curindex][0]
        comp_port1[0] = curnode[0]
        
        self.curindex += 1
