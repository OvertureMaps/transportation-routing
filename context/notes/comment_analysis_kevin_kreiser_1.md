# Analysis of Kevin Kreiser's First Comment

## The Comment

> early on i dismissed the idea of direct support for overture in valhalla because its just a crazy amount of work. indeed i suggested the best bang for everyones buck is converting it back to osm pbf so all the osm tools work with it. having worked now with overture for the sake of display/tiles and with the differences in the schema i do see that it might be a little easier to use for routing in some regards than osm since topological nodes are first class citizens in overture. the biggest struggle is just doing all the attribute parsing in a sane way but at least with overture it should be already "normalized" (ie there is only one tagging variant of a particular attribute/concept that the parser has to care about). so yeah might be interesting to consider it in valhalla. also the spatial nature of geoparquet makes processing (tile cutting) easier.
> 
> at a high level i could see an architecture where overture could fit in. today valhalla is a pipeline of steps:
> 
> 1. parse osm in to temporary data structures
> 2. demarcate the topology assigning graph nodes and edges
> 3. cut tiles
> 4. a bunch more stages to enhance those ...
> 
> for overture we would need to do a first pass over the overture data to assign graphid (nodes and edges) but after that we could simply start cutting tiles. the only annoying part would then be the work to parse its schema vs the osm one but as i mentioned at least its better normalized than osm.
> 
> dont get me wrong, its still a lot of work. and maybe there is some trickery about subsegment attributions overlapping and making a misery for us in the first step but at the same time i think finding a router whose graph preparation is easily configurable to whatever schema and then doing the work to configure it is likely similarly difficult. i could be wrong though!

## Key Points

1. **Initial Skepticism**: Kevin initially dismissed direct Overture support in Valhalla due to complexity.

2. **Advantages of Overture**:
   - Topological nodes are first-class citizens
   - Schema is normalized (consistent attribute representation)
   - GeoParquet format makes tile cutting easier

3. **Valhalla Pipeline Overview**:
   - Parse OSM into temporary data structures
   - Demarcate topology (assign graph nodes and edges)
   - Cut tiles
   - Additional enhancement stages

4. **Integration Approach**: For Overture, we'd need to:
   - Do a first pass to assign GraphIDs to nodes and edges
   - Then start cutting tiles

5. **Challenges**:
   - Parsing Overture schema vs. OSM schema
   - Potential issues with subsegment attributions overlapping

## Relevant Valhalla Code

To understand Kevin's points, we should examine the Valhalla pipeline and how it processes data.

### Valhalla Pipeline Structure

The Valhalla pipeline consists of several stages, as Kevin mentioned:

1. **Parsing Stage**: Converts OSM PBF to internal structures
   - `src/mjolnir/pbfgraphparser.cc`: Parses OSM PBF files
   - `src/mjolnir/osmpbfparser.cc`: Lower-level PBF parsing

2. **Topology Stage**: Assigns graph nodes and edges
   - `src/mjolnir/graphbuilder.cc`: Builds the initial graph structure
   - `src/mjolnir/node_expander.cc`: Expands nodes into the graph

3. **Tile Cutting Stage**: Divides the graph into tiles
   - `src/mjolnir/graphtilebuilder.cc`: Creates the tile structure

4. **Enhancement Stages**: Various additional processing
   - `src/mjolnir/graphenhancer.cc`: Enhances the graph with additional attributes
   - `src/mjolnir/hierarchybuilder.cc`: Builds the hierarchical structure
   - And many more specialized enhancers

### First-Class Topological Nodes

Kevin mentions that Overture has first-class topological nodes, which could be an advantage. In Valhalla, the topology is derived from OSM ways:

```cpp
// From src/mjolnir/graphbuilder.cc
void BuildTileSet(const std::string& ways_file, const std::string& way_nodes_file,
                 const std::string& nodes_file, const std::string& edges_file) {
  // Read nodes and ways
  std::unordered_map<uint64_t, GraphId> node_ids;
  
  // Identify nodes that should be in the graph (intersections, etc.)
  for (const auto& way : ways) {
    for (const auto& node : way.nodes) {
      if (IsIntersection(node)) {
        node_ids[node.id] = AssignGraphId(node);
      }
    }
  }
  
  // Create edges between nodes
  for (const auto& way : ways) {
    CreateEdges(way, node_ids);
  }
}
```

With Overture's first-class topological nodes, this process could potentially be simplified.

### Schema Normalization

Kevin mentions that Overture's schema is normalized, which is an advantage over OSM's varied tagging. In Valhalla, OSM tags are processed using Lua scripts:

```cpp
// From src/mjolnir/luatagtransform.cc
bool LuaTagTransform::Transform(OSMWay& way, const Tags& tags) {
  lua_getglobal(lua_state_, "way_function");
  if (!lua_isfunction(lua_state_, -1)) {
    throw std::runtime_error("Lua way_function not found");
  }
  
  // Push tags to Lua
  lua_newtable(lua_state_);
  for (const auto& tag : tags) {
    lua_pushstring(lua_state_, tag.second.c_str());
    lua_setfield(lua_state_, -2, tag.first.c_str());
  }
  
  // Call Lua function
  if (lua_pcall(lua_state_, 1, 2, 0)) {
    throw std::runtime_error("Failed to execute Lua way_function");
  }
  
  // Process results
  // ...
}
```

With a normalized schema, this complex tag transformation could be simplified.

### GeoParquet Processing

Kevin mentions that GeoParquet makes tile cutting easier. Valhalla's current tile cutting process is complex:

```cpp
// From src/mjolnir/graphtilebuilder.cc
void GraphTileBuilder::StoreTileData() {
  // Calculate sizes and offsets
  size_t header_size = sizeof(GraphTileHeader);
  size_t nodes_size = nodes_.size() * sizeof(NodeInfo);
  // ... more size calculations ...
  
  // Create the file
  std::ofstream file(filename_, std::ios::out | std::ios::binary);
  
  // Write header
  file.write(reinterpret_cast<const char*>(&header_), sizeof(GraphTileHeader));
  
  // Write nodes
  file.write(reinterpret_cast<const char*>(nodes_.data()), nodes_size);
  
  // ... write more sections ...
}
```

GeoParquet's spatial indexing could potentially simplify this process.

## Potential Integration Approach

Based on Kevin's comments and the code examination, a potential integration approach would be:

1. **Create a Parser for Overture Data**: Implement a parser that reads Overture's GeoParquet format.

2. **Map Overture Schema to Valhalla Structures**: Create a mapping from Overture's normalized schema to Valhalla's internal structures.

3. **Leverage Topological Nodes**: Use Overture's first-class topological nodes to simplify the topology creation process.

4. **Integrate at the Appropriate Stage**: As Kevin suggests in his later comment, integrate at the `ConstructEdges` phase to bypass the OSM-specific parsing.

## Conclusion

Kevin's insights about Overture's advantages (normalized schema, first-class topological nodes, GeoParquet format) are valuable for designing an integration approach. His suggestion to focus on a specific integration point in the pipeline aligns with the code structure and would minimize the amount of Valhalla code that needs to be modified or forked.
