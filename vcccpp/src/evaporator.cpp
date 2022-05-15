/* 

  Evaporator.cpp

*/

#include "evaporator.hpp"

Evaporator::Evaporator(umComponent dictComp)
{
    name = any_cast<const char *>(dictComp["name"]);
    iPort = new Port(any_cast<mPort>(dictComp["iPort"]));
    oPort = new Port(any_cast<mPort>(dictComp["oPort"]));
    portdict = {{"iPort", iPort},
                {"oPort", oPort}};
    energy = "RefrigerationCapacity";
}

Evaporator::~Evaporator()
{
    delete iPort;
    delete oPort;
}

void Evaporator::state()
{
    iPort->p = oPort->p;
}

void Evaporator::balance()
{
    // mass and energy balance
    // mass balance
    if (!isnan(iPort->mdot))
    {
        oPort->mdot = iPort->mdot;
    }
    else
    {
        if (!isnan(oPort->mdot))
            iPort->mdot = oPort->mdot;
    }
    Qin = iPort->mdot * (oPort->h - iPort->h);
}

void Evaporator::setportaddress()
{
    if (iPort != portdict["iPort"])
        iPort = portdict["iPort"];
    if (oPort != portdict["oPort"])
        oPort = portdict["oPort"];
}

string Evaporator::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Port::title;
    result += "\n" + iPort->resultstring();
    result += "\n" + oPort->resultstring();
    result += "\nThe Refrigeration Capacity(kW): " + to_string_with_precision<double>(Qin, 3) + "\n";
    return result;
}