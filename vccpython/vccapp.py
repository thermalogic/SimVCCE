
"""
General Object-oriented Abstraction  of VC Cycle 

The Simulator of VC Cycle 
  * Input :VC cycle dict model
  * Output: text file
Run: 
   python vccapp.py

 Author: Cheng Maohua cmh@seu.edu.cn
"""
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles
from vccmodel import cycles
from platform import os

if __name__ == "__main__":

    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'

    for curcycle in cycles:
        ResultFileName = ResultFilePath+curcycle.cycle['name'] + '.txt'

        cycle = VCCycle(curcycle.cycle)
        cycle.simulator()
        # output to text
        OutFiles(cycle)
        OutFiles(cycle, ResultFileName)

      
