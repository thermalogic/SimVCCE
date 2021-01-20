/* 
  The vapor-compression refrigeration cycle simulator for education in C++
  compressor.cpp

*/

#include "compressor.hpp"

Compressor::Compressor(dictDevice dictDev, mapNode nodes)
{
    name = any_cast<const char *>(dictDev["name"]);
    iNode = nodes[any_cast<int>(dictDev["iNode"])];
    oNode = nodes[any_cast<int>(dictDev["oNode"])];
}

Compressor::~Compressor()
{
}

void Compressor::state()
{
    //    Isentropic compression (ideal VCR cycle)
    oNode->s = iNode->s;
}

void Compressor::balance()
{
    // mass and energy balance
    // mass balance
    if (!isnan(iNode->mdot))
    {
        oNode->mdot = iNode->mdot;
    }
    else
    {
        if (!isnan(oNode->mdot))
            iNode->mdot = oNode->mdot;
    }
    //energy
    Wc = iNode->mdot * (oNode->h - iNode->h);
}

string Compressor::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Node::title;
    result += "\n" + iNode->resultstring();
    result += "\n" + oNode->resultstring();
    result += "\nThe Compressor Work(kW): " + to_string_with_precision<double>(Wc, 3) + "\n";
    return result;
}