/* 

  compressor.cpp

*/

#include "compressor.hpp"

Compressor::Compressor(umComponent dictComp)
{
    name = any_cast<const char *>(dictComp["name"]);
    iPort = new Port(any_cast<mPort>(dictComp["iPort"]));
    oPort = new Port(any_cast<mPort>(dictComp["oPort"]));
    portdict = {{"iPort", iPort},
                {"oPort", oPort}};
    energy = "CompressorWork";
}

Compressor::~Compressor()
{
    delete iPort;
    delete oPort;
}

void Compressor::state()
{
    //    Isentropic compression (ideal VCR cycle)
    oPort->s = iPort->s;
}

void Compressor::balance()
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
    //energy
    Wc = iPort->mdot * (oPort->h - iPort->h);
}

void Compressor::setportaddress()
{
    if (iPort != portdict["iPort"])
        iPort = portdict["iPort"];
    if (oPort != portdict["oPort"])
        oPort = portdict["oPort"];
}

string Compressor::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Port::title;
    result += "\n" + iPort->resultstring();
    result += "\n" + oPort->resultstring();
    result += "\nThe compressor Work(kW): " + to_string_with_precision<double>(Wc, 3) + "\n";
    return result;
}

