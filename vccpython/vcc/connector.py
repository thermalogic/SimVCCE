"""
General Object-oriented Abstraction of VC Cycle 
  
  class Connector

 Author: Cheng Maohua cmh@seu.edu.cn
"""

from components.port import *


class Connector:

    def __init__(self):
        self.nodes = []
        self.curindex = 0

    def __combined_node_value(self, node, port):
        """ 
           the node is the connector of two ports ,so the node may get values from all of two ports
            the values is the union set of the non-none values within two ports 
        """
        for key, portvalue in port.__dict__.items():
            nodevalue = node.__dict__[key]
            if portvalue is not None and nodevalue is None:
                node.__dict__[key] = portvalue
      
    def add_node(self, portpairs, comps):
        """ node : ("comp0.port0", "comp1.port1")"""
        comp0, port0 = portpairs[0].split(".")
        comp1, port1 = portpairs[1].split(".")

        # 1 get the index of port in nodes
        comps[comp0].__dict__[port0].index = self.curindex
        # 2 create the new node with port0
        self.nodes.append(comps[comp0].__dict__[port0])  # port0

        # 3 merge comp1's port1 info into  nodes[self.curindex]
        self.__combined_node_value(self.nodes[self.curindex], comps[comp1].__dict__[port1])
        # 4 set port0 and port1 as the alias of  nodes[self.curindex]
        comps[comp0].__dict__[port0]= self.nodes[self.curindex]
        comps[comp1].__dict__[port1] = self.nodes[self.curindex]
        
        self.curindex += 1