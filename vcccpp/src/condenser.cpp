/* 
  The vapor-compression refrigeration cycle simulator for education in C++

  condenser.cpp

*/

#include "condenser.hpp"

Condenser::Condenser(dictDevice dictDev, mapNode nodes)
{
    name = any_cast<const char *>(dictDev["name"]);
    iNode = nodes[any_cast<int>(dictDev["iNode"])];
    oNode = nodes[any_cast<int>(dictDev["oNode"])];
}

Condenser::~Condenser()
{
}

void Condenser::state()
{
    iNode->p = oNode->p;
}

void Condenser::balance()
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
}

string Condenser::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Node::title;
    result += "\n" + iNode->resultstring();
    result += "\n" + oNode->resultstring();
    return result;
}