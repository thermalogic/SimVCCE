/*----------------------------------------------------

  port.cpp
----------------------------------------------------------*/

#include "common.hpp"
#include "port.hpp"

Port::Port(mPort curmPort)
{
   index = NONEINDEX;
   if (curmPort.count("p") != 0)
      try
      {

         p = any_cast<double>(curmPort["p"]);
      }
      catch (...)
      {
      }
   if (curmPort.count("t") != 0)
      try
      {
         t = any_cast<double>(curmPort["t"]);
      }
      catch (...)
      {
         
      }
   if (curmPort.count("x") != 0)
      try
      {
         x = any_cast<double>(curmPort["x"]);
      }
      catch (...)
      {
      }
   if (curmPort.count("mdot") != 0)
      try
      {
         mdot = any_cast<double>(curmPort["mdot"]);
      }
      catch (...)
      {
        
      }
   
   stateok = false;

   if (!isnan(t) & !isnan(x))
   {
      tx();
   }
   else
   {
      if (!isnan(p) & !isnan(x))
      {
         px();
      }
      else
      {
         if (!isnan(p) & !isnan(t))
         {
            pt();
         }
      }
   }
}

Port::~Port()
{
}

void Port::tx()
{
   try
   {
      p = PropsSI("P", "T", 273.15 + t, "Q", x, "R134a") / 1.0e6;
      h = PropsSI("H", "T", 273.15 + t, "Q", x, "R134a") / 1000;
      s = PropsSI("S", "T", 273.15 + t, "Q", x, "R134a") / 1000;
      stateok = true;
   }
   catch (exception &e)
   {
      stateok = false;
   }
}

void Port::px()
{
   try
   {
      t = PropsSI("T", "P", p * 1.0e6, "Q", x, "R134a") -273.15;
      h = PropsSI("H", "P", p * 1.0e6, "Q", x, "R134a") / 1000;
      s = PropsSI("S", "P", p * 1.0e6, "Q", x, "R134a") / 1000;
      stateok = true;
   }
   catch (exception &e)
   {
      stateok = false;
   }
}

void Port::ps()
{
   try
   {
      if (isnan(h))
      {
         h = PropsSI("H", "P", p * 1.0e6, "S", s * 1000, "R134a") / 1000;
      }
      if (isnan(t))
      {
         t = PropsSI("T", "P", p * 1.0e6, "S", s * 1000, "R134a") - 273.15;
      }
      if (isnan(x))
      {
         x = PropsSI("Q", "P", p * 1.0e6, "S", s * 1000, "R134a");
         if (x == -1)
         {
            x = NAN;
         }
      }
      stateok = true;
   }
   catch (...)
   {
      stateok = false;
   }
}

void Port::ph()
{
   try
   {
      if (isnan(s))
      {
         s = PropsSI("S", "P", p * 1.0e6, "H", h * 1000, "R134a") / 1000;
      }
      if (isnan(t))
      {
         t = PropsSI("T", "P", p * 1.0e6, "H", h * 1000, "R134a") - 273.15;
      }
      if (isnan(x))
      {
         x = PropsSI("Q", "P", p * 1.0e6, "H", h * 1000, "R134a");
         if (x == -1)
         {
            x = NAN;
         }
      }
      stateok = true;
   }
   catch (...)
   {
      stateok = false;
   }
}

void Port::pt()
{
   try
   {
      if (isnan(s))
      {
         s = PropsSI("S", "P", p * 1.0e6, "T", t + 273.15, "R134a") / 1000;
      }
      if (isnan(h))
      {
         h = PropsSI("H", "P", p * 1.0e6, "T", t + 273.15, "R134a") / 1000;
      }
      if (isnan(x))
      {
         x = PropsSI("Q", "P", p * 1.0e6, "T", t + 273.15, "R134a");
         if (x == -1)
         {
            x = NAN;
         }
      }
      stateok = true;
   }
   catch (...)
   {
      stateok = false;
   }
}

void Port::state()
{
   if (stateok == false)
   {
      if (!isnan(p) & !isnan(s))
      {
         ps();
      }
      else
      {

         if (!isnan(p) & !isnan(h))
         {
            ph();
         }
         else
         {
            if (!isnan(p) & !isnan(t))
            {
               pt();
            }
         }
      }
   }
}

string Port::resultstring()
{
   string result;
   result = to_string(index);
   result += "\t" + to_string_with_precision<double>(p, 3);
   result += "\t" + to_string_with_precision<double>(t, 3);
   result += "\t" + to_string_with_precision<double>(h, 3);
   result += "\t\040\040\040" + to_string_with_precision<double>(s, 3);
   result += "\t" + to_string_with_precision<double>(x, 3);
   result += "\t" + to_string_with_precision<double>(mdot, 3);
   return result;
}