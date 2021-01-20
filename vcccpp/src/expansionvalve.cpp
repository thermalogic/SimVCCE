/* 
  The vapor-compression refrigeration cycle simulator for education in C++
     expansionvalve.cpp
*/

#include "expansionvalve.hpp"

ExpansionValve::ExpansionValve(dictDevice dictDev, mapNode nodes)
{

    name = any_cast<const char *>(dictDev["name"]);
    iNode = nodes[any_cast<int>(dictDev["iNode"])];
    oNode = nodes[any_cast<int>(dictDev["oNode"])];
}

ExpansionValve::~ExpansionValve()
{
}

void ExpansionValve::state()
{
    oNode->h = iNode->h;
}

void ExpansionValve::balance()
{
    // mass and energy balance
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

string ExpansionValve::resultstring()
{
    string result;
    result = "\n" + name;
    result += "\n" + Node::title;
    result += "\n" + iNode->resultstring();
    result += "\n" + oNode->resultstring();
    return result;
}