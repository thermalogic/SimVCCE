# SimVCCE

The SimVCCE is the vapor-compression refrigeration and heat pump cycle steady-state simulator for education.

The simulator is available in Python, C++ and Modelica

* branch: **B2022**`:  [Python : sequential-modular approach](./vccpython)

* branch **sm-port-conn**: Python, C++ and Modelica

We wish that SimVCCE may be a helpful vehicle for you to understand Computational Thinking and improve programming skills

### Prerequisites：CoolProp, phyprops

```bash
python -m pip install coolprop
```

```bash
python -m pip install phyprops
```
## The Vapor-Compression Cycle Examples

- [1 The Ideal Vapor-Compression Refrigeration Cycles](#1-the-ideal-vapor-compression-refrigeration-cycles)
- [2 The Cascade Refrigeration Cycle](#2-the-cascade-refrigeration-cycle)
- [3 Multistage Compression Refrigeration](#3-multistage-compression-refrigeration)
- [4 Performance Analysis of VCR cycle under Variable Conditions](#4-performance-analysis-of-vcr-cycle-under-variable-conditions)

### 1 The Ideal Vapor-Compression Refrigeration Cycles

>Thuan Ke Nguyen. CHE302:Chemical and Materials Engineering Thermodynamics I,USA

* using the Python Module for the Cycle Flowsheet and Data

  * [./vccpython/vccapp.py](./vccpython/vccapp.py) 
  
* using JSON File of the Cycle Flowsheet and Data

   * [./vccpython/vccapp_json.appn](./vccpython/vccapp_json.app) 

The cycles of [Example 7.2-3,7.2-4](https://www.cpp.edu/~tknguyen/che302/Notes/chap7-2.pdf)

Refrigerant `134a` is the working fluid in an ideal vapor-compression refrigeration cycle 

![vcr-cycle](./img/vcr-cycle.jpg)

The mass flow rate of the refrigerant is `0.08 kg/s`.

|   VCR         | Cold region | Warm region | Enter Compressor  | Leave Condenser   |
| -------------- |:-------------:| -----------:|------------------------:|------------------------:|
| 7.2-3  |    0 °C    | 26   °C   |Saturated vapor  0°C   | Saturated liquid 26°C  |
| 7.2-4   |    -10 °C  |          |Saturated vapor -10    | Saturated liquid 0.9MPa  |

**Determine**

 * the compressor power, in kW
 
 * the refrigeration capacity, in tons
 
 * the coefficient of performance

![vcr-7234-ts](./img/vcr-7234-ts.jpg)

### 2 The Cascade Refrigeration Cycle

>Majid Bahrami. ENSC461(S11)：Energy Conversion,Simon Fraser University,Canada

* [./playground/demo_cascade.py](./playground/demo_cascade.py)

### 3 Multistage Compression Refrigeration 

>Yunus A. Cengel, Michael A. Boles, Thermodynamics: An Engineering Approach, Seventh Edition,McGraw-Hill, 2011.

Two-stage Compression Refrigeration:Ex 11-58

* [./playground/demo_two_stage.py](./playground/demo_two_stage.py)


### 4 Performance Analysis of VCR cycle under Variable Conditions


* [./playground/demo_var.py](./playground/demo_var.py)

The ideal vapor-compression refrigeration cycle of [Vapor-compression refrigeration simulation and tutorial](https://peer.asee.org/vapor-compression-refrigeration-simulation-and-tutorial.pdf)

**Variable conditions**

* **refrigerants**: R12,R134a,R22

* **condenser pressures**:0.4, 0.5, 0.6, 0.7, 0.8, 0.9,0.1, 1.4 MPa.

