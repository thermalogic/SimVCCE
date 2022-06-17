# SimVCCE

Branch 1: The Ideal Vapor-Compression Refrigeration Cycles

- [SimVCCE](#simvcce)
    - [Prerequisites：CoolProp, phyprops](#prerequisitescoolprop-phyprops)
  - [The Vapor-Compression Cycle Examples](#the-vapor-compression-cycle-examples)
    - [1 The Ideal Vapor-Compression Refrigeration Cycles](#1-the-ideal-vapor-compression-refrigeration-cycles)
  
The SimVCCE is the vapor-compression refrigeration and heat pump cycle steady-state simulator for education.

The simulator is available in Python, C++ and Modelica with sequential-modular approach

* branch: **B2022**`: 
  
  *  [Python](./vccpython)
  *  [C++](./vcccpp)

* branch **sm-port-conn**: Python, C++ and Modelica

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

### 1 The Ideal Vapor-Compression Refrigeration Cycles

* using the Python Module for the Cycle Flowsheet and Data

```
 ./vccpython/vccapp.py 
```
* using JSON File of the Cycle Flowsheet and Data

```
./vccpython/vccapp_json_one.py jsonname_of_cycle
```

EXAMPLE 11–1 The Ideal Vapor-Compression Refrigeration  Cycle

A refrigerator uses R134a as the working fluid and operates on an ideal vapor-compression refrigeration cycle between 0.14 and 0.8 MPa.

If the mass flow rate of the refrigerant is 0.05 kg/s, 

**Determine** 

* (a) the rate of heat removal from the refrigerated space and the power input to the compressor,
* (b) the rate of heat rejection to the environment, and 
* (c) the COP of the refrigerator.

![ivcr-11-1](./img/ivcr_11_1.jpg)

