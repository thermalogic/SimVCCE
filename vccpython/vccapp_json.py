
"""
General Object-oriented Abstraction  of VC Cycle 
The Simulator of VC Cycle 
  * Input :the json file of the cycle model
  * Output: text file
Run: 
   python vccapp_json.py

 Author: Cheng Maohua cmh@seu.edu.cn

"""
import glob
from vcc.vccobj import VCCycle
from vcc.utils import OutFiles, create_dictcycle_from_jsonfile
from platform import os


if __name__ == "__main__":
    curpath = os.path.abspath(os.path.dirname(__file__))
    json_filenames_str = curpath+'\\'+'./jsonmodel/*.json'
    #json_filenames_str=curpath+'\\'+'./jsonmodel/ivcr724.json'
    json_filenames = glob.glob(json_filenames_str)
    ResultFilePath = curpath+'/result/'

    for i in range(len(json_filenames)):
        thedictcycle = create_dictcycle_from_jsonfile(json_filenames[i])
        # the simulator
        cycle = VCCycle(thedictcycle)
        cycle.simulator()
        # output to console
        OutFiles(cycle)
        # output to the file
        ResultFileName = ResultFilePath+thedictcycle['name']
        OutFiles(cycle, ResultFileName + '.txt')

