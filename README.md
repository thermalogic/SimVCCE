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

### Run

In the terminal of `./vccpython` 
 
**1 Using the Python Module for the Cycle Flowsheet and Data**

```bash
python  vccapp.py
```

**2 Using JSON File of the Cycle Flowsheet and Data**

```bash
python  vccapp_json.py
```

## The Example Vapor-Compression Cycles

The cycle of [Example 7.2-3,7.2-4](https://www.cpp.edu/~tknguyen/che302/Notes/chap7-2.pdf)

* [ivcr723.json](./vccpython/jsonmodel/ivcr723.json) 

* [ivcr724.json](./vccpython/jsonmodel/ivcr724.json) 

Refrigerant `134a` is the working fluid in an ideal vapor-compression refrigeration cycle 

The mass flow rate of the refrigerant is `0.08 kg/s`.

| No          | Cold region  | Warm region  | Enters the Compressor  | Leaves the Condenser   |
| -------------- |:-------------:| -----------:|------------------------:|------------------------:|
| Example 7.2-3  |    0 °C    | 26   °C   |Saturated vapor  0°C   | Saturated liquid 26°C  |
| Example 7.2-4   |    -10 °C  |          |Saturated vapor -10    | Saturated liquid 0.9MPa  |

**Determine**

 * the compressor power, in kW
 
 * the refrigeration capacity, in tons
 
 * the coefficient of performance

![](./img/vcr-7234-ts.jpg)

