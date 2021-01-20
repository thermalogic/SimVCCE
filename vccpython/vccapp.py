
"""
The vapor-compression refrigeration cycle simulator for education in Python
 
  * Input :vcr cycle dict model

  * Output: text file

Run: 
   python vccapp.py
"""
from vcc.vccobj import VCRCycle
from vcc.utils import OutFiles
from vccmodel import cycles
from platform import os

if __name__ == "__main__":

    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultPath = curpath+'/result/'

    for curcycle in cycles:
        ResultFileName = ResultPath+curcycle.cycle['name']

        cycle = VCRCycle(curcycle.cycle)
        cycle.simulator()
        # output to text
        OutFiles(cycle)
        OutFiles(cycle, ResultFileName + '.txt')
