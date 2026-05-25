/*
  Condenser.cpp
*/

#ifndef CONDENSER_HPP
#define CONDENSER_HPP

#include "../common/common.hpp"
#include "../core/port.hpp"

class Condenser : public CompSISO
{
public:
  double Qout;
  // methods
  Condenser(umComponent dictComp);
  ~Condenser();
 
  void setportaddress();
  void state();
  void balance();
  string resultstring();
 
};

#endif /* Condenser_hpp */
