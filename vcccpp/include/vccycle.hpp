/*
  The vapor-compression refrigeration cycle simulator for education in C++
    
    vccycle.hpp
*/

#ifndef VCCYCLE_HPP
#define VCCYCLE_HPP

#include "common.hpp"
#include "node.hpp"
#include "compressor.hpp"
#include "condenser.hpp"
#include "evaporator.hpp"
#include "expansionvalve.hpp"

class VCCycle
{
public:
    mapNode nodes;
    mapComponent comps;

    double Wc;
    double Qin;
    double cop;

    // methods
    VCCycle(vector<dictDevice> dictNodes, vector<dictDevice> dictcomps);
    ~VCCycle();

    void state();
    void balance();
    string resultstr();
    void outdevresultstr();
    void outresults();

private:
    void itemnew(dictDevice item);
    void itembalance(ComponentClass curclass, CompBase *item);
};

#endif /* VCCycle_hpp */
