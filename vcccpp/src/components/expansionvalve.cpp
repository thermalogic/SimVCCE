/*
  expansionvalve.cpp

*/

#include "expansionvalve.hpp"

ExpansionValve::ExpansionValve(umComponent dictComp)
{
    name = any_to_string(dictComp["name"]);
    iPort = new Port(any_cast<mPort>(dictComp["iPort"]));
    oPort = new Port(any_cast<mPort>(dictComp["oPort"]));
    portdict = {{"iPort", iPort},
                {"oPort", oPort}};
    
 
}

ExpansionValve::~ExpansionValve()
{
    delete iPort;
    delete oPort;
}

void ExpansionValve::state()
{
    // ideal Isenthalpic expansion
    if (!isnan(iPort->h) && isnan(oPort->h))
    {
        oPort->h = iPort->h;
    }
    else if (!isnan(oPort->h) && isnan(iPort->h))
    {
        iPort->h = oPort->h;
    }
    else if (isnan(iPort->h) && isnan(oPort->h))
    {
        throw runtime_error("ExpansionValve: both ports h are NaN");
    }
}

void ExpansionValve::balance()
{
    // mass and energy balance
    // mass balance
    if (isnan(iPort->mdot) && isnan(oPort->mdot))
        throw runtime_error("ExpansionValve: mdot is NaN");
    if (!isnan(iPort->mdot))
    {
        oPort->mdot = iPort->mdot;
    }
    else
    {
        if (!isnan(oPort->mdot))
            iPort->mdot = oPort->mdot;
    }
}

void ExpansionValve::setportaddress()
{
  if (iPort!=portdict["iPort"])
      iPort=portdict["iPort"];
  if (oPort!=portdict["oPort"])
      oPort=portdict["oPort"];
}

string ExpansionValve::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Port::title;
    result += "\n" + iPort->resultstring();
    result += "\n" + oPort->resultstring();
    return result;
}