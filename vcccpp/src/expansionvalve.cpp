/*
  expansionvalve.cpp

*/

#include "expansionvalve.hpp"

ExpansionValve::ExpansionValve(umComponent dictComp)
{
    name = any_cast<const char *>(dictComp["name"]);
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
    oPort->h = iPort->h;
}

void ExpansionValve::balance()
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