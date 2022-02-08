"""
General Object-oriented Abstraction of VC Cycle 

 - OutFiles(cycle, outfilename=None)
 - create_dictcycle_from_jsonfile(filename):

 Author: Cheng Maohua cmh@seu.edu.cn
"""
import sys
import json
from components.port import Port


def OutFiles(cycle, outfilename=None):
    savedStdout = sys.stdout
    # redirect to the outfilename
    if outfilename is not None:
        datafile = open(outfilename, 'w', encoding='utf-8')
        sys.stdout = datafile

    # 1 output cycle performance
    print(cycle)

    # 2 output nodes
    print(Port.title)
    for item in cycle.conns.nodes:
        print(item)
    # 3 output devices
    for key in cycle.comps:
        print(cycle.comps[key])

    # return to sys.stdout
    if (outfilename != None):
        datafile.close()
        sys.stdout = savedStdout
