
"""
General Object-oriented Abstraction of VC Cycle 

The ideal vaporcompression refrigeration cycle:
Assume
* the evaporator pressure is maintained constant at 0.12MPa
* the mass flow rate of refrigerant is 1 kg/s.

refrigerants: R12,R134a,R22

Condenser pressures: 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,1.0, 1.4 MPa

Calculate the COP of the refrigeration cycle

* Save the results to the csv file

* Plot the COPs against the condenser pressure

 Author: Cheng Maohua cmh@seu.edu.cn
"""
import os
import matplotlib.pyplot as plt
import csv
import json

from vcc.utils import OutFiles
from vcc.vccobj import VCCycle


def csv_vars(refrigerants, x, y, FileName):
    with open(FileName, 'w', newline='') as csvfile:
        fieldnames = ['Condenser_Pressure']
        for ant in refrigerants:
            fieldnames.append("COP("+ant+")")

        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()

        # rows
        for i in range(len(x)):
            rowdict = {}
            rowdict['Condenser_Pressure'] = f"{x[i]:.2f}"
            for j in range(len(refrigerants)):
                rowdict["COP("+refrigerants[j] + ")"] = f"{y[j][i]:.2f}"
            writer.writerow(rowdict)


def plot_vars(refrigerants, x, y):
    plt.title('Variance of Condenser Pressure on Coefficient of Performance')
    plt.xlabel('Pressure(MPa)')
    plt.ylabel('COP')
    for i in range(len(refrigerants)):
        plt.plot(x, y[i], "-s", label=refrigerants[i])
    plt.legend(loc="best")
    plt.show()


if __name__ == "__main__":
    import sys
    #jsonname = sys.argv[1]
    jsonname = "ivcr_var.json"

    curpath = os.path.abspath(os.path.dirname(__file__))
    json_filename = curpath+'/jsonmodel/'+jsonname

    ResultFilePath = curpath+'/result/'
    # the base data of cycle
    with open(json_filename, 'r') as f:
        thedictcycle = json.loads(f.read())

    # vars
    refrigerants = ["R12", "R134a", "R22"]
    cdpressures = {"name": "Condenser",
                   "oPort": {"p": [0.4, 0.5, 0.6, 0.7, 0.8, 1.0, 1.4]}
                   }

    results = []
    for cant in refrigerants:
        # var refrigerants
        newdictcycle = thedictcycle.copy()
        newdictcycle["refrigerant"] = cant
        cur_result = []
        for i, var in enumerate(cdpressures["oPort"]["p"]):
            # var condenser pressures
            for device in newdictcycle["components"]:
                if device["name"] == cdpressures["name"]:
                    device["oPort"]["p"] = var
                    break

            # the simulator
            cycle = VCCycle(newdictcycle)
            cycle.simulator()
            cur_result.append(cycle.cop)

            # output to console
            # OutFiles(cycle)
            # output to the file
            # ResultFileName = ResultFilePath + \
            #    thedictcycle['name']+"_"+cant+"_"+str(i+1)+"_"
            #OutFiles(cycle, ResultFileName + '.txt')

        # after the analysis
        results.append(cur_result)

    # save to csv
    CSVFileName = ResultFilePath + thedictcycle['name']+"_var.csv"
    csv_vars(refrigerants, cdpressures["oPort"]["p"], results, CSVFileName)
    # plot
    plot_vars(refrigerants, cdpressures["oPort"]["p"], results)
