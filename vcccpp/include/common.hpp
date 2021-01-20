/*
The vapor-compression refrigeration cycle simulator for education in C++
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
#include "node.hpp"
#include "CoolPropLib.h"

using namespace std;

class CompBase
{
public:
  string name;
  // nodes
  Node *iNode;
  Node *oNode;

  virtual void state() = 0;
  virtual void balance() = 0;
  virtual string resultstring() = 0;
};

typedef unordered_map<string, any> dictDevice;
typedef map<int, Node *> mapNode;
typedef unordered_map<string, CompBase *> mapComponent;

enum class ComponentClass
{
  Compressor,
  Condenser,
  Evaporator,
  ExpansionValve
};

template <typename T>
string to_string_with_precision(const T a_value, const int n = 6)
{
  ostringstream out;
  out.precision(n);
  if (!isnan(a_value))
    out << fixed << a_value;
  else
  {
    out << fixed << " -- ";
  }
  return out.str();
};

#endif /* COMMON_HPP */
