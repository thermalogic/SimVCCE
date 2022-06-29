/*
   Evaporator.hpp
*/

#ifndef EVAPORATOR_HPP
#define EVAPORATOR_HPP

#include "common.hpp"

class Evaporator : public CompSISO
{
public:
    double Qin;
    // methods
    Evaporator(umComponent dictComp);
    ~Evaporator();
  
    void setportaddress();
    void state();
    void balance();
    string resultstring();
 
};

#endif /* EVAPORATOR_HPP */
