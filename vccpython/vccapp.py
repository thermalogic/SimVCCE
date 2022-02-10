
"""
General Object-oriented Abstraction  of VC Cycle 

The Simulator of Ideal VCR 11-1 Page612

Yunus A. Cengel, Michael A. Boles, Thermodynamics: An Engineering Approach, 8th Edition,McGraw-Hill, 2015.

  * Input : The cycle dict model: ivcr_11_1.py
  * Output: text file

Author: Cheng Maohua cmh@seu.edu.cn
"""
from vccmodel.ivcr_11_1 import cycle
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles
import os

if __name__ == "__main__":
    thecycle = VCCycle(cycle)
    thecycle.simulator()
    # output to console
    OutFiles(thecycle)

    # output to text file
    curpath = os.path.abspath(os.path.dirname(__file__))
    ResultFilePath = curpath+'/result/'
    ResultFileName = ResultFilePath+cycle['name'] + '.txt'
    OutFiles(thecycle, ResultFileName)
