#include "osmdata.h"

#include <stdio.h>
#include <stdint.h>

void osmwaynode_set_way_index(struct OSMWayNode * s, uint64_t index, uint64_t value)
{
   // uint32_t way_index;
   s[index].way_index = value;
}

uint64_t osmwaynode_get_way_index(const struct OSMWayNode * s, uint64_t index)
{
   // uint32_t way_index;
   return s[index].way_index;
}

void osmwaynode_set_way_shape_node_index(struct OSMWayNode * s, uint64_t index, uint64_t value)
{
   // uint32_t way_shape_node_index;
   s[index].way_shape_node_index = value;
}

uint64_t osmwaynode_get_way_shape_node_index(const struct OSMWayNode * s, uint64_t index)
{
   // uint32_t way_shape_node_index;
   return s[index].way_shape_node_index;
}

static void print_field(const char * field_name, uint64_t value)
{
   printf("%s: %lu\n", field_name, value);
}

void osmwaynode_dump(const struct OSMWayNode * s, uint64_t index)
{
   // Dumping fields for OSMWayNode
   print_field("way_index", (uint64_t)s[index].way_index);
   print_field("way_shape_node_index", (uint64_t)s[index].way_shape_node_index);
}

