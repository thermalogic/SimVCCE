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
        string classstr = any_to_string(item["classstr"]);
        string name = any_to_string(item["name"]);
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

    // 3 set port addresses for all components
    for (mComponentObj::iterator iter = Comps.begin(); iter != Comps.end(); iter++)
        iter->second->setportaddress();

    Wc = 0.0;
    Qin = 0.0;
    Qout = 0.0;
    cop = 0.0;
    cop_hp = 0.0;
}

void VCCycle::component_simulator()
{
    vector<Port *> state_nodes = curcon->Nodes;

    vector<string> keys;
    for (auto &iter : Comps)
        keys.push_back(iter.first);

    bool deviceok = false;
    int CountsDev = Comps.size();
    int i = 0;

    while (deviceok == false && i <= CountsDev)
    {
        vector<string> keys_to_process = keys;
        for (auto &curdev : keys_to_process)
        {
            try
            {
                // step 2: the port state: thermal process
                Comps[curdev]->state();

                // step 3: the port state: new port's parameter pairs
                size_t j = 0;
                while (j < state_nodes.size())
                {
                    if (!state_nodes[j]->stateok)
                    {
                        state_nodes[j]->state();
                        if (state_nodes[j]->stateok)
                        {
                            state_nodes.erase(state_nodes.begin() + j);
                        }
                        else
                        {
                            j++;
                        }
                    }
                    else
                    {
                        j++;
                    }
                }

                // step 4: the port state: the energy and mass balance
                Comps[curdev]->balance();
                // Remove curdev from keys
                auto pos = find(keys.begin(), keys.end(), curdev);
                if (pos != keys.end())
                {
                    keys.erase(pos);
                }
            }
            catch (...)
            {
                // Pass the exception and continue
            }
        }
        i++;
        if (keys.size() == 0)
        {
            deviceok = true;
        }
    }

    if (keys.size() > 0)
    {
        for (auto &k : keys)
        {
            cout << k << " ";
        }
        cout << endl;
    }
}

void VCCycle::simulator()
{
    component_simulator();

    Wc = 0.0;
    Qin = 0.0;
    Qout = 0.0;

    for (auto &iter : Comps)
    {
        if (iter.second->energy == "CompressionWork")
        {
            Wc += ((Compressor *)iter.second)->Wc;
        }
        else if (iter.second->energy == "QIN")
        {
            Qin += ((Evaporator *)iter.second)->Qin;
        }
        else if (iter.second->energy == "QOUT")
        {
            Qout += ((Condenser *)iter.second)->Qout;
        }
    }
    cop = Qin / Wc;
    cop_hp = Qout / Wc;
}

void VCCycle::state()
{
    simulator();
}

void VCCycle::balance()
{
    // Already handled in simulator
}

string VCCycle::resultstr()
{
    string result;
    result = "\n --- The Cycle ---\n";
    result += "\tCompression Work(kW): " + to_string_with_precision<double>(Wc, 3) + "\n";
    result += "\tRefrigeration Capacity(kW): " + to_string_with_precision<double>(Qin, 3) + "\n";
    result += "\tCapacity(ton): " + to_string_with_precision<double>(Qin * 60.0 * (1.0 / 211.0), 3) + "\n";
    result += "\tThe heat transfer rate(kW): " + to_string_with_precision<double>(Qout, 3) + "\n";
    result += "\tThe coefficient of performance: " + to_string_with_precision<double>(cop, 3) + "\n";
    result += "\tThe coefficient of performance(heat pump): " + to_string_with_precision<double>(cop_hp, 3) + "\n";
    return result;
}

void VCCycle::outdevresultstr()
{
    for (auto &iter : Comps)
    {
        cout << iter.second->resultstring() << endl;
    }
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
