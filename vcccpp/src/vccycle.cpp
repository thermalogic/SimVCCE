/*

The vapor-compression refrigeration cycle simulator for education in C++
  
  vcccycle.cpp
  
  - The Simple Factory Pattern: void VCCycle::itemnew(dictDevice item)

*/
#include "vccycle.hpp"

map<string, ComponentClass> MapStringToClass =
    {
        {"Compressor", ComponentClass::Compressor},
        {"Condenser", ComponentClass::Condenser},
        {"Evaporator", ComponentClass::Evaporator},
        {"ExpansionValve", ComponentClass::ExpansionValve}};

ComponentClass curclass(string idstr)
{
    int nPos = idstr.find("-");
    return MapStringToClass[idstr.substr(nPos + 1)];
}

template <typename T>
void newcompitem(dictDevice item, mapNode nodes, mapComponent &comps)
{
    string classstr = any_cast<const char *>(item["classstr"]);
    string name = any_cast<const char *>(item["name"]);
    string idstr = name + "-" + classstr;
    comps.insert(mapComponent::value_type(idstr, new T(item, nodes)));
}

void VCCycle::itemnew(dictDevice item)
{
    string classstr = any_cast<const char *>(item["classstr"]);
    ComponentClass curclass = MapStringToClass[classstr];

    switch (curclass)
    {
    case ComponentClass::Compressor:
        newcompitem<Compressor>(item, nodes, comps);
        break;
    case ComponentClass::Condenser:
        newcompitem<Condenser>(item, nodes, comps);
        break;
    case ComponentClass::Evaporator:
        newcompitem<Evaporator>(item, nodes, comps);
        break;
    case ComponentClass::ExpansionValve:
        newcompitem<ExpansionValve>(item, nodes, comps);
        break;
    default: //Optional
             ;
    }
}

void VCCycle::itembalance(ComponentClass curclass, CompBase *item)
{
    item->balance();
    switch (curclass)
    {
    case ComponentClass::Compressor:
    {
        Wc = ((Compressor *)item)->Wc;
        break;
    }
    case ComponentClass::Evaporator:
    {
        Qin = ((Evaporator *)item)->Qin;
        break;
    }
    default: //Optional
             ;
    }
}

// -- public ---

VCCycle::~VCCycle()
{
    for (mapNode::iterator iter = nodes.begin(); iter != nodes.end(); iter++)
        delete iter->second;
    nodes.clear();

    for (mapComponent::iterator iter = comps.begin(); iter != comps.end(); iter++)
        delete iter->second;
    comps.clear();
}

VCCycle::VCCycle(vector<dictDevice> dictNodes, vector<dictDevice> dictcomps)
{

    for (auto &item : dictNodes)
    {
        int id = any_cast<int>(item["id"]);
        nodes.insert(mapNode::value_type(id, new Node(item)));
    }

    for (auto &item : dictcomps)
    {
        itemnew(item);
    }
}

void VCCycle::state()
{
    for (mapComponent::iterator iter = comps.begin(); iter != comps.end(); iter++)
    {
        iter->second->state();
    };

    for (mapNode::iterator iter = nodes.begin(); iter != nodes.end(); iter++)
    {
        iter->second->state();
    };
}

void VCCycle::balance()
{
    for (mapComponent::iterator iter = comps.begin(); iter != comps.end(); iter++)
    {
        itembalance(curclass(iter->first), iter->second);
    }
    cop = Qin / Wc;
}

string VCCycle::resultstr()
{
    string result;
    result = "\n --- The Cycle ---\n";
    result += "\tCompression Work(kW): " + to_string_with_precision<double>(Wc, 3) + "\n";
    result += "\tRefrigeration Capacity(kW): " + to_string_with_precision<double>(Qin, 3) + "\n";
    result += "\tThe coefficient of performance: " + to_string_with_precision<double>(cop, 3) + "\n";
    return result;
}

void VCCycle::outdevresultstr()
{

    for (mapComponent::iterator iter = comps.begin(); iter != comps.end(); iter++)
    {
        cout << iter->second->resultstring() << endl;
    }

    cout << "\n"
         << Node::title << endl;
    for (mapNode::iterator iter = nodes.begin(); iter != nodes.end(); iter++)
    {
        cout << iter->second->resultstring() << endl;
    }
}

void VCCycle::outresults()
{
    cout << resultstr() << endl;
    outdevresultstr();
}
