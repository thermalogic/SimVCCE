
"""
General Object-oriented Abstraction  of VC Cycle 

The Simulator of Ideal VCR 11-1 Page612

Yunus A. Cengel, Michael A. Boles, Thermodynamics: An Engineering Approach, 8th Edition,McGraw-Hill, 2015.

  * Input : The cycle dict model: ivcr_11_1.py
  * Output: text file

Author: Cheng Maohua cmh@seu.edu.cn
"""
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles
from vccmodel.ivcr_11_1 import cycle
from platform import os

if __name__ == "__main__":

    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'
    ResultFileName = ResultFilePath+cycle['name'] + '.txt'

    thecycle = VCCycle(cycle)
    thecycle.simulator()
    # output to console
    OutFiles(thecycle)
    # output to text file
    OutFiles(thecycle, ResultFileName)

      
