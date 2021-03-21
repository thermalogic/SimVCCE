/*

COMMON_HPP
*/
#ifndef COMMON_HPP
#define COMMON_HPP

#include <iostream>
#include <iomanip>
#include <string>
#include <map>
#include <unordered_map>
#include <any>
#include <exception>
#include <cmath>
#include <vector>
#include <sstream>
#include <tuple>
#include "port.hpp"
#include "CoolPropLib.h"

using namespace std;

typedef unordered_map<string, any> umComponent;
// port
typedef tuple<string,string> tupPort;
typedef tuple<tupPort,tupPort> tupConnector;
typedef map<string, Port *> mapPortObj;

class CompBase
{
public:
  string name;
  string energy;
  Port *iPort;
  Port *oPort;
  mapPortObj portdict;
  
  virtual void setportaddress()=0;
  virtual void state() = 0;
  virtual void balance() = 0;
  virtual string resultstring() = 0;
  
};

typedef unordered_map<string, CompBase *> mComponentObj;

template <typename T>
string to_string_with_precision(const T a_value, const int n = 6)
{
  ostringstream out;
  out.precision(n);
  if (!isnan(a_value))
     out << fixed << a_value;
  else
  {
     out << fixed <<" -- ";
  }
  return out.str();
}

#endif /* COMMON_HPP */
