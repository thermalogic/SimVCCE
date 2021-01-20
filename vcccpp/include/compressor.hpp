/*
The vapor-compression refrigeration cycle simulator for education in C++
  compressor.hpp
*/

#ifndef COMPRESSOR_HPP
#define COMPRESSOR_HPP

#include "common.hpp"

class Compressor : public CompBase
{
public:
  double Wc;

  // methods
  Compressor(dictDevice dictDev, mapNode nodes);
  ~Compressor();

  void state();
  void balance();
  string resultstring();
};

#endif /* Compressor_hpp */
