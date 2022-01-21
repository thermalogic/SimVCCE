# Playground for Your New Ideas

## 1. Performance Analysis of VCR cycle under Variable Conditions

* [ivcrvar.json](./jsonmodel/ivcrvar.json) 

* demo_vccapp_json_var.py

The ideal vapor-compression refrigeration cycle of [Vapor-compression refrigeration simulation and tutorial](https://peer.asee.org/vapor-compression-refrigeration-simulation-and-tutorial.pdf)

* the evaporator pressure is maintained constant at 0.12MPa
* the mass flow rate of refrigerant is 1 kg/s.

**Variable conditions**

* **refrigerants**: R12,R134a,R22

* **condenser pressures**:0.4, 0.5, 0.6, 0.7, 0.8, 0.9,0.1, 1.4 MPa.

**Analysis**

Calculate the COP of the ideal vapor-compression refrigeration cycle 

* **Plot** the COPs against the condenser pressure

* **Save** the results to the csv file

## 2. The Home Heat Pump for Space Heating

The Cycle of [A Home Heat Pump for Space Heating](https://www.ohio.edu/mechanical/thermo/Intro/Chapt.1_6/refrigerator/heatpump.html)

The heat pump system absorbs heat from the evaporator placed outside in order to pump heat into the air flowing through the insulated duct over the condenser section

![ohio_48](./img/ohio_48.jpg)

In this example, we evaluate the following:

**The Heat Pump**

* Heat absorbed by the evaporator: Qin

* Heat rejected by the condenser: Qout

* Coefficient of Performance as a heat pump:COP_hp

* Determine the **mass** flow rate of the refrigerant R134a

**The Insulated Duct**

* Determine the **mass** flow rate of the air flowing in the insulated duct

* Assuming that all this heat is absorbed by the air, determine the exit **temperature** of the air at station


