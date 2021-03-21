/*
 The vapor-compression refrigeration cycle simulator 

*/
#include "vcc.hpp"

vector<umComponent> Components = {
    {{"name", "Compressor"},
     {"classstr", "Compressor"},
     {"iPort", (mPort){{"t", 0.0}, {"x", 1.0}, {"mdot", 0.08}}},
     {"oPort", (mPort){{"p", 0.6854}}}
    },
    {{"name", "Condenser"},
     {"classstr", "Condenser"},
     {"iPort", (mPort){}},
     {"oPort", (mPort){{"t", 26.0}, {"x", 0.0}}}
    },
    {{"name", "ExpansionValve"},
     {"classstr", "ExpansionValve"},
     {"iPort", (mPort){}},
     {"oPort", (mPort){}}
    },
    {{"name", "Evaporator"},
     {"classstr", "Evaporator"},
     {"iPort", (mPort){}},
     {"oPort", (mPort){}}
     }
};

vector<tupConnector> Connectors = {
    {{"Compressor", "oPort"}, {"Condenser", "iPort"}},
    {{"Condenser", "oPort"}, {"ExpansionValve", "iPort"}},
    {{"ExpansionValve", "oPort"}, {"Evaporator", "iPort"}},
    {{"Evaporator", "oPort"}, {"Compressor", "iPort"}}};

int main()
{
  // --- init the cycle analysis ----
  ClassReg curclassreg;
  curclassreg.register_type_all();
  VCCycle::compinstance = curclassreg.compinstance; // the instance of compfactory

  // --- start the cycle analysis -------
  unique_ptr<VCCycle> curcycle(new VCCycle(Components, Connectors));
  curcycle->state();
  curcycle->balance();
  curcycle->outresults();
  return 0;
}