/*
  CONNECTOR.hpp
*/

#ifndef CONNECTOR_HPP
#define CONNECTOR_HPP

#include "common.hpp"

class Connector
{
public:
  int index;
  vector<Port *> Nodes;
  Connector();
  ~Connector();
  void AddConnector(tupConnector tconn, mComponentObj &comps);

private:
  void getnodevalue(Port *port);
};
#endif /* CONNECTOR_HPP */