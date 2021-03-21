
/* 
  Connector.cpp
*/

#include "connector.hpp"

Connector::Connector()
{
}

Connector::~Connector()
{
   for (auto &item : Nodes)
      delete item;
   Nodes.clear();
}

void Connector::getnodevalue(Port *port)
{

   if ((Nodes[index]->index == NONEINDEX) & (port->index != NONEINDEX))
   {
      Nodes[index]->index = port->index;
   };
   if (isnan(Nodes[index]->p) & !isnan(port->p))
   {
      Nodes[index]->p = port->p;
   };
   if (isnan(Nodes[index]->t) & !isnan(port->t))
   {
      Nodes[index]->t = port->t;
   };
   if (isnan(Nodes[index]->h) & !isnan(port->h))
   {
      Nodes[index]->h = port->h;
   };
   if (isnan(Nodes[index]->s) & !isnan(port->s))
   {
      Nodes[index]->s = port->s;
   };
   if (isnan(Nodes[index]->x) & !isnan(port->x))
   {
      Nodes[index]->x = port->x;
   };
   if (isnan(Nodes[index]->mdot) & !isnan(port->mdot))
   {
      Nodes[index]->mdot = port->mdot;
   };
}

void Connector::AddConnector(tupConnector tconn, mComponentObj &comps)
{
   auto [comp0, port0] = get<0>(tconn);
   auto [comp1, port1] = get<1>(tconn);

   // 1 get the index of port in Nodes
   index = Nodes.size();
   comps[comp0]->portdict[port0]->index = index;
   // 2  init the Node[index] using the port0
   Nodes.push_back(comps[comp0]->portdict[port0]);
   // 3 join the port1 info to the Node[index]
   getnodevalue(comps[comp1]->portdict[port1]);
   // 1,2,3 use the origin address of ports

   // 4 send the Node[index] info back to the port1 by change the address of port1 to the Node[index]
   comps[comp1]->portdict[port1] = Nodes[index];
   comps[comp1]->setportaddress();
}
