#include "osmdata.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct OSMWayNode * osmwaynode_new(uint64_t count)
{
    struct OSMWayNode * output = malloc(sizeof(struct OSMWayNode) * count);
    memset(output, 0, sizeof(struct OSMWayNode) * count);
    return output;
}

void osmwaynode_free(struct OSMWayNode * s)
{
    free(s);
}

void osmwaynode_clear(struct OSMWayNode * s, uint64_t count)
{
    memset(s, 0, sizeof(struct OSMWayNode) * count);
}

int osmwaynode_export(const struct OSMWayNode * s, int count, const char * fileName)
{
    // TODO: error checking
    FILE * outFile = fopen(fileName, "wb");
    fwrite(s, sizeof(struct OSMWayNode), count, outFile);
    fclose(outFile);
    return 1;
}

void osmwaynode_import(const char * fileName, struct OSMWayNode ** buffer, uint64_t * count)
{
    // TODO: error checking
    // TODO: move this to a common function (similar to osmnode_import etc)
    FILE * inFile = fopen(fileName, "rb");
    fseek(inFile, 0, SEEK_END);
    *count = ftell(inFile) / sizeof(struct OSMWayNode);
    // TODO: make sure file size is exactly a multiple of sizeof(struct OSMWayNode)
    fseek(inFile, 0, SEEK_SET);
    *buffer = malloc(sizeof(struct OSMWayNode) * (*count));
    fread(*buffer, sizeof(struct OSMWayNode), *count, inFile);
    fclose(inFile);
}

void osmwaynode_set_node(struct OSMWayNode * s, uint64_t index, const struct OSMNode * value)
{
   // OSMNode node;
   memcpy(&s[index].node, value, sizeof(struct OSMNode));
}

void osmwaynode_get_node(const struct OSMWayNode * s, uint64_t index, struct OSMNode * value)
{
    // OSMNode node;
    memcpy(value, &s[index].node, sizeof(struct OSMNode));
}

void osmwaynode_set_to_valhalla(struct OSMWayNode * s, uint64_t index, uint64_t way_index, uint64_t way_shape_node_index, uint64_t osmid, uint64_t lng7, uint64_t lat7, uint64_t intersection)
{
    memset(s + index, 0, sizeof(struct OSMWayNode));

    s[index].way_index = way_index;
    s[index].way_shape_node_index = way_shape_node_index;

    osmnode_set_to_valhalla(&s[index].node, 0, osmid, lng7, lat7, intersection);
}
