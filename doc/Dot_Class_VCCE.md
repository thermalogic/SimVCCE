# Class Diagrams of VCCE in Dot

- [Class Diagrams of VCCE in Dot](#class-diagrams-of-vcce-in-dot)
  - [Class Method or Field](#class-method-or-field)
    - [Visibility](#visibility)
    - [Scope](#scope)
  - [Class and Instance Relationships](#class-and-instance-relationships)
    - [Instance-level Relationships](#instance-level-relationships)
    - [Class-level  Relationship](#class-level--relationship)

## Class Method or Field

### Visibility

When you define methods or fields, you can use **characters** to define the **visibility** of the corresponding item:

| Character | Visibility |
| --------- | :--------: |
| -         |  private   |
| +         |  public   |
  

```dot
digraph "classes" {
rankdir=BT
charset="utf-8"
"VCCycle" [
       color="black", fontcolor="black", 
      label="{VCCycle|
       + name : str\l 
       + refrigerant : str\l
       + comps : dict of device\l 
       + conns : Connector\l
       + Qin : float\l 
       + Qout : float\l 
       + Wc : float\l
       + cop : float\l
       + cop_hp : float\l|
       + VCCycle(dictcycle:dict)\l
       - component_simulator()\l 
       + simulator()\l 
       + __str__():str\l}", shape="record", style="solid"];
}


```
### Scope

The UML specifies two types of scope for members: `instance and class`

You can define **class**  methods or fields using the **static**

```dot
digraph "classes" {
rankdir=BT
charset="utf-8"
"components.device_siso.Device_SISO" [color="black", fontcoor="black", 
              label="{Device_SISO|
               + static devtype: str\l
               + static energy: str\l 
               + name:str \l 
               + iPort: Port\l 
               + oPort: Port\l|
               + Device_SISO(dictDev：dicts)\l 
               + state()\l 
               + mass_balance()\l 
               + balance()\l 
               + __str()__:str\l}",
               shape="record", style="solid"];
}

```

##  Class and Instance Relationships

### Instance-level Relationships

* Composition： a filled diamond shape (diamond)
* Aggregation： a filled diamond shape (odiamond)

```dot
digraph {
rankdir=BT

node [shape="record"]

"Connector" -> "VCCycle"
  [arrowhead="diamond", arrowtail="none",fontcolor="blue",
  label="1..*", style="solid"]

{ "Compressor",
  "Condenser",
  "Evaporator",
  "ExpansionValve"} -> "VCCycle"
  [arrowhead="odiamond", arrowtail="none",fontcolor="blue",
  label="1..*", style="solid"]

"Port"-> {
    "Connector",
    "ExpansionValve",
    "Evaporator",
    "Condenser",
    "Compressor"}
    [arrowhead="diamond", arrowtail="none",
     fontcolor="blue",label="2", style="solid"]
} 
```

### Class-level  Relationship
 
* Inheritance： a hollow triangle shape(empty)

```dot
digraph {
rankdir=BT
node [shape="record"]
{
   "Compressor",
    "Condenser",
    "Evaporator",
    "ExpansionValve"}->"Device_SISO"
     [arrowhead="empty", arrowtail="none",style="solid"];
}
```
