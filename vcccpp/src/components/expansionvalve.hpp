/*
   ExpansionValve.cpp
*/

#ifndef EXPANSIONVALVE_HPP
#define EXPANSIONVALVE_HPP

#include "../common/common.hpp"
#include "../core/port.hpp"

class ExpansionValve : public CompSISO
{
public:
   // methods
   ExpansionValve(umComponent dictComp);
   ~ExpansionValve();

   void setportaddress();
   void state();
   void balance();
   string resultstring();
 
};

#endif /* EXPANSIONVALVE_hpp */
