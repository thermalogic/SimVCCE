
"""
The vapor-compression refrigeration cycle simulator for education in Python
  ExpansionValve: Throttling ：Isenthalpic expansion
"""
from .node import *

class ExpansionValve:

    energy = "None"
    devtype = "EXPANSIONVALVE"

    def __init__(self, dictDev, nodes):
        """ Initializes the ExpansionValve """
        self.name = dictDev['name']
        self.iNode = nodes[dictDev['iNode']]
        self.oNode = nodes[dictDev['oNode']]

    def state(self):
        """
          Throttling：Isenthalpic expansion
        """
        self.oNode.h=self.iNode.h
    
    def balance(self):
        """ mass and energy balance  """
        if self.iNode.mdot is not None:
            self.oNode.mdot = self.iNode.mdot
        elif self.oNode.mdot is not None:
            self.iNode.mdot = self.oNode.mdot

 
    def __str__(self):
        result = '\n' + self.name
        result += '\n' + Node.title
        result += '\n' + self.iNode.__str__()
        result += '\n' + self.oNode.__str__()
        return result
