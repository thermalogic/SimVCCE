/* 

  condenser.cpp

*/

#include "condenser.hpp"

Condenser::Condenser(umComponent dictComp)
{
    name = any_to_string(dictComp["name"]);
    iPort = new Port(any_cast<mPort>(dictComp["iPort"]));
    oPort = new Port(any_cast<mPort>(dictComp["oPort"]));
    portdict = {{"iPort", iPort},
                {"oPort", oPort}};
    energy = "QOUT";
    Qout = NAN;
}

Condenser::~Condenser()
{
    delete iPort;
    delete oPort;
}

void Condenser::state()
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
        throw runtime_error("Condenser: both ports p are NaN");
    }
}

void Condenser::balance()
{
    // mass and energy balance
    // mass balance
    if (isnan(iPort->mdot) && isnan(oPort->mdot))
        throw runtime_error("Condenser: mdot is NaN");
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
        throw runtime_error("Condenser: h is NaN");
    Qout = iPort->mdot * (iPort->h - oPort->h);
}

void Condenser:: setportaddress()
{
  if (iPort!=portdict["iPort"])
      iPort=portdict["iPort"];
  if (oPort!=portdict["oPort"])
      oPort=portdict["oPort"];
}

string Condenser::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Port::title;
    result += "\n" + iPort->resultstring();
    result += "\n" + oPort->resultstring();
    result += "\nThe heat transfer rate(kW): " + to_string_with_precision<double>(Qout, 3) + "\n";
    return result;
}