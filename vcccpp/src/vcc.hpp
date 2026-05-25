/*
  vccycleport.hpp
    
- The Factory Method Pattern: 
     typedef std::map<std::string, std::function<CompSISO *(umComponent, mapPort)>> compfactory;
     class ClassReg
*/

#ifndef VCC_HPP
#define VCC_HPP

#include "./common/common.hpp"
#include "./core/port.hpp"
#include "./core/connector.hpp"
#include "./components/compressor.hpp"
#include "./components/condenser.hpp"
#include "./components/evaporator.hpp"
#include "./components/expansionvalve.hpp"
#include <typeinfo>
#include <functional>
#include <fstream>
#include <sstream>
#include <regex>

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
    double Qout;
    double cop;
    double cop_hp;

    // methods
    VCCycle(vector<umComponent> dictcomps, vector<tupConnector> vecConnectors);
    ~VCCycle();

    void component_simulator();
    void simulator();
    void state();
    void balance();
    string resultstr();
    void outdevresultstr();
    void outresults();
};

#endif /* VCCycleport_hpp */
