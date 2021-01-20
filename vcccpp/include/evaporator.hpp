/*
   The vapor-compression refrigeration cycle simulator for education in C++
   evaporator.hpp
*/

#ifndef EVAPORATOR_HPP
#define EVAPORATOR_HPP

#include "common.hpp"

class Evaporator : public CompBase
{
public:
    double Qin;

    // methods
    Evaporator(dictDevice dictDev, mapNode nodes);
    ~Evaporator();

    void state();
    void balance();
    string resultstring();
};

#endif /* EVAPORATOR_HPP */
