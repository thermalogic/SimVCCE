/*
 The vapor-compression refrigeration cycle simulator - JSON Version

*/
#include "./src/vcc.hpp"
#include "./src/utils/json_loader.hpp"

// Global loader to keep string pool alive
unique_ptr<JSONLoader> g_loader;

int main(int argc, char* argv[])
{
    string jsonFile = "jsonmodel/demovcc.json";
    if (argc > 1) {
        jsonFile = argv[1];
    }

    cout << "Loading cycle from: " << jsonFile << endl;

    // --- init the cycle analysis ----
    ClassReg curclassreg;
    curclassreg.register_type_all();
    VCCycle::compinstance = curclassreg.compinstance; // the instance of compfactory

    // --- load from JSON -------
    try {
        g_loader = make_unique<JSONLoader>();
        if (!g_loader->loadFile(jsonFile)) {
            cerr << "Failed to load JSON file" << endl;
            return 1;
        }
        
        unique_ptr<VCCycle> curcycle = g_loader->createCycle();
        
        cout << "Successfully loaded cycle" << endl;

        // --- start the cycle analysis -------
        curcycle->simulator();
        curcycle->outresults();
    } catch (const exception& e) {
        cerr << "Error: " << e.what() << endl;
        return 1;
    }

    return 0;
}
