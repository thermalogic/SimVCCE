/*

The vapor-compression refrigeration cycle simulator for education in C++
    node.hpp

*/

#ifndef NODE_HPP
#define NODE_HPP

#include <string>
#include <unordered_map>
#include <any>
#include <exception> 
#include <cmath>
#include "CoolPropLib.h"

using namespace std;

typedef unordered_map<string, any>  dictNode;

class Node
{
  public:
    inline static const string title = "NodeID 	         DESC               P(MPa)   T(C)  H(kJ/kg)  S(kJ/kg.K)  Quality MDOT(kg/s)"; 
    // fields
    string desc;
    int id;
  
    double p;
    double t;
    double h;
    double s;
    double x;
    double mdot;
    bool stateok;
    
    // methods
    Node(dictNode curdictnode);
    ~Node();
    
    void tx();
    void ps();
    void ph();
    void pt();
    void state();
    string resultstring();
   
   
};

#endif /* node_hpp */
