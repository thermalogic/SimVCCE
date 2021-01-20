/*

The vapor-compression refrigeration cycle simulator for education in C++

  evaporator.cpp

*/

#include "evaporator.hpp"

Evaporator::Evaporator(dictDevice dictDev, mapNode nodes)
{

    name = any_cast<const char *>(dictDev["name"]);
    iNode = nodes[any_cast<int>(dictDev["iNode"])];
    oNode = nodes[any_cast<int>(dictDev["oNode"])];
}

Evaporator::~Evaporator()
{
}

void Evaporator::state()
{
    iNode->p = oNode->p;
}

void Evaporator::balance()
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
    Qin = iNode->mdot * (oNode->h - iNode->h);
}

string Evaporator::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Node::title;
    result += "\n" + iNode->resultstring();
    result += "\n" + oNode->resultstring();
    result += "\nThe Refrigeration Capacity(kW): " + to_string_with_precision<double>(Qin, 3) + "\n";
    return result;
}