#ifndef OSMDATA_H
#define OSMDATA_H

#include "osmnode.h"

struct OSMWayNode {
  struct OSMNode node;
  uint32_t way_index;
  uint32_t way_shape_node_index;
};

#endif /* OSMDATA_H */

