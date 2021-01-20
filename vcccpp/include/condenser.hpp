/*
  The vapor-compression refrigeration cycle simulator for education in C++
     condenser.cpp
*/

#ifndef CONDENSER_HPP
#define CONDENSER_HPP

#include "common.hpp"

class Condenser : public CompBase
{
public:
  // methods
  Condenser(dictDevice dictDev, mapNode nodes);
  ~Condenser();

  void state();
  void balance();
  string resultstring();
};

#endif /* Condenser_hpp */
