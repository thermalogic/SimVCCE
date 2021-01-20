/*
  The vapor-compression refrigeration cycle simulator for education in C++
    expansionvalve.hpp
*/

#ifndef EXPANSIONVALVE_HPP
#define EXPANSIONVALVE_HPP

#include "common.hpp"

class ExpansionValve : public CompBase
{
public:
  // methods
  ExpansionValve(dictDevice dictDev, mapNode nodes);
  ~ExpansionValve();

  void state();
  void balance();
  string resultstring();
};

#endif /* EXPANSIONVALVE_HPP */
