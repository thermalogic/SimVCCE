/*
The vapor-compression refrigeration cycle simulator for education in C++
  node.cpp

*/

#include "common.hpp"
#include "node.hpp"
string formatStr(string str, int length)
{
  int strLen = str.length();
  if (strLen == length)
  {
    return str;
  }
  else
  { 
    if (strLen < length)
    {
      int temp = length - strLen;
      string tem = "";
      for (int i = 0; i < temp; i++)
      {
           tem = tem + " ";
      }   
      return str + tem;
    }  
  }
  return str;
};

Node::Node(dictNode curdictnode)
{
   desc = any_cast<const char *>(curdictnode["desc"]);
   id = any_cast<int>(curdictnode["id"]);
   try
   {
      p = any_cast<double>(curdictnode["p"]);
   }
   catch (...)
   {
      p = NAN;
   }

   try
   {
      t = any_cast<double>(curdictnode["t"]);
   }
   catch (...)
   {
      t = NAN;
   }
   try
   {
      x = any_cast<double>(curdictnode["x"]);
   }
   catch (...)
   {
      x = NAN;
   }
   try
   {
      mdot = any_cast<double>(curdictnode["mdot"]);
   }
   catch (...)
   {
      mdot = NAN;
   }
   h = NAN;
   s = NAN;
   stateok = false;

   if (!isnan(t) & !isnan(x))
   {
      tx();
   }
}

Node::~Node()
{
}

void Node::tx()
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

void Node::ps()
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

void Node::ph()
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


void Node::pt()
{
   try
   {
      if (isnan(s))
      {
         s = PropsSI("S", "P", p * 1.0e6, "T",t+273.15, "R134a") / 1000;
      }
      if (isnan(h))
      {
         h = PropsSI("H", "P", p * 1.0e6, "T", t+273.15, "R134a") /1000;
      }
      if (isnan(x))
      {
         x = PropsSI("Q", "P", p * 1.0e6, "T", t+273.15, "R134a");
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

void Node::state()
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

string Node::resultstring()
{
   string result;
   result = to_string(id) + "\t" +formatStr(desc,30);
   result += to_string_with_precision<double>(p, 3);
   result += "\040\040\040" + to_string_with_precision<double>(t, 2);
   result += "\040\040\040" + to_string_with_precision<double>(h, 2);
   result += "\040\040\040" + to_string_with_precision<double>(s, 3);
   result += "\040\040\040\040" + to_string_with_precision<double>(x, 3);
   result += "\040\040\040" + to_string_with_precision<double>(mdot, 3);
   return result;
}