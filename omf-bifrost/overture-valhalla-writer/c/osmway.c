#include "osmway.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct OSMWay * osmway_new(uint64_t count)
{
    struct OSMWay * output = malloc(sizeof(struct OSMWay) * count);
    memset(output, 0, sizeof(struct OSMWay) * count);
    return output;
}

void osmway_free(struct OSMWay * s)
{
    free(s);
}

void osmway_clear(struct OSMWay * s, uint64_t count)
{
    memset(s, 0, sizeof(struct OSMWay) * count);
}

int osmway_export(const struct OSMWay * s, int count, const char * fileName)
{
    // TODO: error checking
    FILE * outFile = fopen(fileName, "wb");
    fwrite(s, sizeof(struct OSMWay), count, outFile);
    fclose(outFile);
    return 1;
}

void osmway_import(const char * fileName, struct OSMWay ** buffer, uint64_t * count)
{
    // TODO: error checking
    // TODO: move this to a common function (similar to osmnode_import etc)
    FILE * inFile = fopen(fileName, "rb");
    fseek(inFile, 0, SEEK_END);
    *count = ftell(inFile) / sizeof(struct OSMWay);
    // TODO: make sure file size is exactly a multiple of sizeof(struct OSMWay)
    fseek(inFile, 0, SEEK_SET);
    *buffer = malloc(sizeof(struct OSMWay) * (*count));
    fread(*buffer, sizeof(struct OSMWay), *count, inFile);
    fclose(inFile);
}

void osmway_set_to_valhalla(struct OSMWay * s, uint64_t index, uint64_t osmid, uint64_t name_index, uint64_t nodecount)
{
    // Sets to sensible defaults that work well with Valhalla
    memset(s + index, 0, sizeof(struct OSMWay));
    
    // These need to be set to something sensible, typically from Rust code
    s[index].osmwayid_ = osmid;
    s[index].name_index_ = name_index;
    s[index].nodecount_ = nodecount;
    // ^^^^ End of fields that need to be set to something sensible

    // TODO: could also be 0, ("kPavedSmooth")? See "graphconstants.h" in Valhalla
    s[index].surface_ = 3; // kCompacted
    
    s[index].drive_on_right_ = 1;

    // TODO: could also be 6, ("kResidential") or 0 ("kMotorway")? See "graphconstants.h" in Valhalla
    s[index].road_class_ = 7; // kServiceOther

    // TODO: might want to use 0 here ("kRoad)?
    s[index].use_ = 25; // "kFootway" ("enum class Use : uint8_t")

    s[index].has_user_tags_ = 0;

    // TODO: Have a second look at this, does this mean pedestrian-only?
    s[index].pedestrian_forward_ = 1;
    s[index].pedestrian_backward_ = 1;

    s[index].speed_ = 25;
}
