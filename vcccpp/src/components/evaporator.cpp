/* 

  Evaporator.cpp

*/

#include "evaporator.hpp"

Evaporator::Evaporator(umComponent dictComp)
{
    name = any_to_string(dictComp["name"]);
    iPort = new Port(any_cast<mPort>(dictComp["iPort"]));
    oPort = new Port(any_cast<mPort>(dictComp["oPort"]));
    portdict = {{"iPort", iPort},
                {"oPort", oPort}};
    energy = "QIN";
}

Evaporator::~Evaporator()
{
    delete iPort;
    delete oPort;
}

void Evaporator::state()
{
    // ideal Isobaric
    if (!isnan(oPort->p) && isnan(iPort->p))
    {
        iPort->p = oPort->p;
    }
    else if (!isnan(iPort->p) && isnan(oPort->p))
    {
        oPort->p = iPort->p;
    }
    else if (isnan(iPort->p) && isnan(oPort->p))
    {
        throw runtime_error("Evaporator: both ports p are NaN");
    }
}

void Evaporator::balance()
{
    // mass and energy balance
    // mass balance
    if (isnan(iPort->mdot) && isnan(oPort->mdot))
        throw runtime_error("Evaporator: mdot is NaN");
    if (!isnan(iPort->mdot))
    {
        oPort->mdot = iPort->mdot;
    }
    else
    {
        if (!isnan(oPort->mdot))
            iPort->mdot = oPort->mdot;
    }
    // energy balance
    if (isnan(iPort->h) || isnan(oPort->h))
        throw runtime_error("Evaporator: h is NaN");
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