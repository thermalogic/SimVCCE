
"""
General Object-oriented Abstraction of VC Cycle 

The ideal vaporcompression refrigeration cycle:
Assume
* the evaporator pressure is maintained constant at 0.12MPa
* the mass flow rate of refrigerant is 1 kg/s.

refrigerants: R12,R134a,R22

Condenser pressures: 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,0.1, 1.4 MPa

Calculate the COP of the refrigeration cycle

* Plot the COPs against the condenser pressure
* Save the results to the csv file

 Author: Cheng Maohua cmh@seu.edu.cn
"""
import os
import sys
curpath = os.path.abspath(os.path.dirname(__file__))
sys.path.append(curpath+'/../vccpython/')

from vcc.vccobj import VCCycle
from vcc.utils import OutFiles, create_dictcycle_from_jsonfile
from components.port import Port


import csv
import matplotlib.pyplot as plt


def var_csv(Coolants, x, y, FileName):
    with open(FileName, 'w', newline='') as csvfile:
        fieldnames = ['Condenser_Pressure']
        for ant in Coolants:
            fieldnames.append("COP("+ant+")")

        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()

        # rows
        for i in range(len(x)):
            rowdict = {}
            rowdict['Condenser_Pressure'] = "{:.2f}".format(x[i])
            for j in range(len(Coolants)):
                rowdict["COP("+Coolants[j]+")"] = "{:.2f}".format(y[j][i])
            writer.writerow(rowdict)


def plot_vars(Coolants, x, y):
    plt.title('Variance of Condenser Pressure on Coefficient of Performance')
    plt.xlabel('Pressure(MPa)')
    plt.ylabel('COP')
    for i in range(len(Coolants)):
        plt.plot(x, y[i], "-s", label=Coolants[i])
    plt.legend(loc="best")
    plt.show()


if __name__ == "__main__":
    curpath = os.path.abspath(os.path.dirname(__file__))
    json_filename = curpath+'\\'+'./jsonmodel/ivcrvar.json'
    ResultFilePath = curpath+'/result/'
    # the base data of cycle
    thedictcycle = create_dictcycle_from_jsonfile(json_filename)

    # vars
    refrigerants = ["R12", "R134a", "R22"]
    vars = {"name": "Condenser",
            "oPort": {"p": [0.4, 0.5, 0.6, 0.7, 0.8, 1.0, 1.4]}
            }

    results = []
    for cant in refrigerants:
        # var coolant
        newdictcycle = thedictcycle.copy()
        newdictcycle["refrigerant"] = cant
        i = 0
        cur_result = []
        for var in vars["oPort"]["p"]:
            # var condenser pressures
            for device in newdictcycle["components"]:
                if device["name"] == vars["name"]:
                    device["oPort"]["p"] = var

             # the simulator
            cycle = VCCycle(newdictcycle)
            cycle.simulator()
            cur_result.append(cycle.cop)
            i += 1
            # output to console
            OutFiles(cycle)
            # output to the file
            ResultFileName = ResultFilePath + \
                thedictcycle['name']+"_"+cant+"_"+str(i)+"_"
            OutFiles(cycle, ResultFileName + '.txt')
        # after the analysis
        results.append(cur_result)

    # save to csv
    CSVFileName = ResultFilePath + thedictcycle['name']+".csv"
    var_csv(refrigerants, vars["oPort"]["p"], results, CSVFileName)
    # plot
    plot_vars(refrigerants, vars["oPort"]["p"], results)
