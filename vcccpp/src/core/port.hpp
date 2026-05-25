/*--------------------------------------------- 

   Port.cpp

-------------------------------------------------------*/

#ifndef PORT_HPP
#define PORT_HPP

#include <string>
#include <unordered_map>
#include <any>
#include <exception>
#include <cmath>
#include "CoolPropLib.h"

using namespace std;

#define NONEINDEX -100

typedef unordered_map<string, any> mPort;

class Port
{
public:
  inline static const string title = "Port   	P(MPa)   T(C)  H(kJ/kg)	  S(kJ/kg.K)  Quality MDOT(kg/s)"; //C++17

  int index = NONEINDEX;

  double p = NAN;
  double t = NAN;
  double h = NAN;
  double s = NAN;
  double x = NAN;
  double mdot = NAN;

  bool stateok;

  // methods
  Port(mPort curmPort);
  ~Port();

  void tx();
  void pt();
  void px();
  void ps();
  void ph();
  void state();
  string resultstring();
};

#endif /* port_hpp */
