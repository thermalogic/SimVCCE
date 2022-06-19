# SimVCCE

Branch 1: The Ideal Vapor-Compression Refrigeration Cycles

- [SimVCCE](#simvcce)
    - [Prerequisites：CoolProp, phyprops](#prerequisitescoolprop-phyprops)
  - [The Vapor-Compression Cycle Examples](#the-vapor-compression-cycle-examples)
    - [The Ideal Vapor-Compression Refrigeration Cycles](#the-ideal-vapor-compression-refrigeration-cycles)
    - [2 Performance Analysis of VCR cycle under Variable Conditions](#2-performance-analysis-of-vcr-cycle-under-variable-conditions)
  
The SimVCCE is the vapor-compression refrigeration and heat pump cycle steady-state simulator for education.

The simulator is available in Python, C++ with sequential-modular approach
  
  *  [Python](./vccpython)
  *  [C++](./vcccpp)

We wish that SimVCCE may be a helpful vehicle for you to understand the object-oriented programming and improve programming skills

### Prerequisites：CoolProp, phyprops

```bash
python -m pip install coolprop
```

```bash
python -m pip install phyprops
```

## The Vapor-Compression Cycle Examples

Yunus A. Cengel, Michael A. Boles,Thermodynamics: An Engineering Approach, 8th Edition, McGraw-Hill.2015

### The Ideal Vapor-Compression Refrigeration Cycles

* using the Python Module for the Cycle Flowsheet and Data

```
 ./vccpython/vccapp.py 
```
* using JSON File of the Cycle Flowsheet and Data

```
./vccpython/vccapp_json_one.py ivcr_11_1.json
```

EXAMPLE 11–1 The Ideal Vapor-Compression Refrigeration  Cycle

A refrigerator uses R134a as the working fluid and operates on an ideal vapor-compression refrigeration cycle between 0.14 and 0.8 MPa.

If the mass flow rate of the refrigerant is 0.05 kg/s, 

**Determine** 

* (a) the rate of heat removal from the refrigerated space and the power input to the compressor,
* (b) the rate of heat rejection to the environment, and 
* (c) the COP of the refrigerator.

![ivcr-11-1](./img/ivcr_11_1.jpg)

### 2 Performance Analysis of VCR cycle under Variable Conditions

```
./vccpython/vccapp_json_one_var.py)
```
The ideal vapor-compression refrigeration cycle of [Vapor-compression refrigeration simulation and tutorial](https://peer.asee.org/vapor-compression-refrigeration-simulation-and-tutorial.pdf)

* the evaporator pressure is maintained constant at 0.12MPa
* the mass flow rate of refrigerant is 1 kg/s.

**Variable conditions**

* **refrigerants**: R12,R134a,R22

* **condenser pressures**:0.4, 0.5, 0.6, 0.7, 0.8, 0.9,0.1, 1.4 MPa.

**Analysis**

Calculate the COP of the ideal vapor-compression refrigeration cycle 

* **Save** the results to the csv file

* **Plot** the COPs against the condenser pressure

