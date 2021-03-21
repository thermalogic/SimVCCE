"""
General Object-oriented Abstraction of vcr Cycle 

 - OutFiles(cycle, outfilename=None)

"""
import sys
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
    for item in cycle.curcon.nodes:
        print(item[0])
    # 3 output devices
    for key in cycle.comps:
        print(cycle.comps[key])
    
    # return to sys.stdout
    if (outfilename != None):
        datafile.close()
        sys.stdout = savedStdout
