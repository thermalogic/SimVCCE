/*
 
vccycleport.cpp
 - Input:
     * vector<umComponent> Components ={}
     * vector<tupConnector>  Connectors = {} 
*/

#include "vcc.hpp"

VCCycle::~VCCycle()
{
    delete curcon;

    for (mComponentObj::iterator iter = Comps.begin(); iter != Comps.end(); iter++)
        delete iter->second;
    Comps.clear();
}

VCCycle::VCCycle(vector<umComponent> dictComps, vector<tupConnector> vecConnectors)
{
    // 1 components
    for (auto &item : dictComps)
    {
        string classstr = any_cast<const char *>(item["classstr"]);
        string name = any_cast<const char *>(item["name"]);
        try
        { // register_type
            Comps.insert(mComponentObj::value_type(name, compinstance[classstr](item)));
        }
        catch (exception &e)
        {
            cout << e.what() << endl;
        }
    }
    // 2 connectors
    curcon = new Connector();
    for (auto &tconn : vecConnectors)
        curcon->AddConnector(tconn, Comps);
}

void VCCycle::state()
{
    // 1 state by process
    for (mComponentObj::iterator iter = Comps.begin(); iter != Comps.end(); iter++)
    {
        iter->second->state();
    };
    // 2 state of port/node
    for (auto &item : curcon->Nodes)
    {
        item->state();
    };
}

void VCCycle::balance()
{
    for (mComponentObj::iterator iter = Comps.begin(); iter != Comps.end(); iter++)
    {
        iter->second->balance();
        if (iter->second->energy == "CompressorWork")
        {
            Wc = ((Compressor *)iter->second)->Wc;
        }
        else
        {
            if (iter->second->energy == "RefrigerationCapacity")
                Qin = ((Evaporator *)iter->second)->Qin;
        }
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
    // 1 component's results
    for (mComponentObj::iterator iter = Comps.begin(); iter != Comps.end(); iter++)
    {
        cout << iter->second->resultstring() << endl;
    }
    // 2 node's results
    cout << "\n"
         << Port::title << endl;
    for (auto &item : curcon->Nodes)
    {
        cout << item->resultstring() << endl;
    }
}

void VCCycle::outresults()
{
    cout << resultstr() << endl;
    outdevresultstr();
}
