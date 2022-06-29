/*
  vccycleport.hpp
    
- The Factory Method Pattern: 
     typedef std::map<std::string, std::function<CompSISO *(umComponent, mapPort)>> compfactory;
     class ClassReg
*/

#ifndef VCC_HPP
#define VCC_HPP

#include "common.hpp"
#include "port.hpp"
#include "connector.hpp"
#include "compressor.hpp"
#include "condenser.hpp"
#include "evaporator.hpp"
#include "expansionvalve.hpp"
#include <typeinfo>
#include <functional>

typedef std::map<std::string, std::function<CompSISO *(umComponent)>> compfactory;

class ClassReg
{
public:
    compfactory compinstance;
    // register
    template <typename T>
    void register_type(const std::string &name)
    {
        compinstance[name] = [](umComponent item) { return new T(item); };
    }

    void register_type_all()
    { // if you have the new component class, register it here!
        register_type<Compressor>("Compressor");
        register_type<Condenser>("Condenser");
        register_type<Evaporator>("Evaporator");
        register_type<ExpansionValve>("ExpansionValve");
    }
};

class VCCycle
{
public:
    inline static compfactory compinstance;
    Connector *curcon;
    mComponentObj Comps;

    double Wc;
    double Qin;
    double cop;

    // methods
    VCCycle(vector<umComponent> dictcomps, vector<tupConnector> vecConnectors);
    ~VCCycle();

    void state();
    void balance();
    string resultstr();
    void outdevresultstr();
    void outresults();
};

#endif /* VCCycleport_hpp */
