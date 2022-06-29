/*
  Compressor.hpp
*/

#ifndef COMPRESSOR_HPP
#define COMPRESSOR_HPP

#include "common.hpp"

class Compressor : public CompSISO
{
public:
  
  double Wc;

  // methods
  Compressor(umComponent dictComp);
  ~Compressor();

  void setportaddress();
  void state();
  void balance();
  string resultstring();
 
};

#endif /* Compressor_hpp */
