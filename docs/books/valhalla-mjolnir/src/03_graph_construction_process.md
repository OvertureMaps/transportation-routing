# Graph Construction Process

## Overview of the Graph Building Pipeline

Mjolnir transforms OSM data into Valhalla's routing graph through a multi-stage pipeline. Understanding this pipeline is essential for building a graph tile builder. The main stages are:

1. **Data Parsing**: Reading OSM data (covered in the previous chapter)
2. **Initial Graph Construction**: Creating a basic graph structure
3. **Graph Enhancement**: Adding additional attributes and metadata
4. **Hierarchical Graph Building**: Creating multiple levels of the graph
5. **Tile Creation**: Dividing the graph into tiles
6. **Serialization**: Writing tiles to disk

Let's examine each stage in detail.

## Initial Graph Construction

After parsing OSM data, Mjolnir constructs an initial graph using the `GraphBuilder` class:

```cpp
// From valhalla/mjolnir/graphbuilder.h
class GraphBuilder {
public:
  static void Build(const boost::property_tree::ptree& pt,
                   const std::string& ways_file,
                   const std::string& way_nodes_file,
                   const std::string& nodes_file,
                   const std::string& edges_file,
                   const std::string& complex_restriction_file,
                   const std::string& osmdata_file);
};
```

The implementation in `src/mjolnir/graphbuilder.cc` performs these key steps:

1. **Node Creation**: Convert OSM nodes to graph nodes
2. **Edge Creation**: Convert OSM ways to directed edges
3. **Initial Attribution**: Assign basic attributes to edges (road class, surface type, etc.)
4. **Connectivity Establishment**: Connect edges at nodes to form a network

Here's a simplified version of the process:

```cpp
// From src/mjolnir/graphbuilder.cc
void GraphBuilder::Build(...) {
  // Create a map of OSM node ids to graph node ids
  std::unordered_map<uint64_t, uint32_t> node_map;
  
  // Create nodes
  for (const auto& osm_node : osm_nodes) {
    if (IsIntersection(osm_node)) {
      uint32_t node_id = AddNode(osm_node);
      node_map[osm_node.id] = node_id;
    }
  }
  
  // Create edges
  for (const auto& osm_way : osm_ways) {
    // For each segment in the way
    for (size_t i = 0; i < osm_way.nodes.size() - 1; i++) {
      uint64_t from_node_id = osm_way.nodes[i];
      uint64_t to_node_id = osm_way.nodes[i+1];
      
      // Create directed edge(s)
      AddEdge(node_map[from_node_id], node_map[to_node_id], osm_way);
      
      // If not one-way, add edge in reverse direction
      if (!osm_way.one_way) {
        AddEdge(node_map[to_node_id], node_map[from_node_id], osm_way);
      }
    }
  }
}
```

During this process, Mjolnir identifies which OSM nodes should become graph nodes. Typically, these are:
- Intersections where multiple ways meet
- Endpoints of ways
- Points where road attributes change significantly

Not every OSM node becomes a graph node, as this would create an unnecessarily large graph. Instead, intermediate nodes along a way are used to define the shape of edges but don't become actual graph nodes.

## Graph Enhancement

After the initial graph is built, Mjolnir enhances it with additional information using the `GraphEnhancer` class:

```cpp
// From valhalla/mjolnir/graphenhancer.h
class GraphEnhancer {
public:
  static void Enhance(const boost::property_tree::ptree& pt,
                     const std::string& access_file);
};
```

The enhancement process includes:

1. **Adding Turn Lanes**: Information about lane configuration at intersections
2. **Speed Assignment**: Setting speed limits based on road class and other factors
3. **Access Restrictions**: Determining which modes of travel can use each edge
4. **Adding Administrative Information**: Country, state, and other boundaries
5. **Edge Classification**: Identifying ramps, links, internal edges, etc.

Here's a simplified example of the enhancement process:

```cpp
// From src/mjolnir/graphenhancer.cc
void GraphEnhancer::Enhance(...) {
  // For each tile in the graph
  for (const auto& tile_id : tile_ids) {
    GraphTileBuilder tile_builder(tile_id);
    
    // For each edge in the tile
    for (const auto& edge : tile_builder.directededges()) {
      // Enhance with additional attributes
      edge.set_speed(DetermineSpeed(edge));
      edge.set_access(DetermineAccess(edge));
      edge.set_classification(ClassifyEdge(edge));
      
      // Add turn lanes if available
      if (HasTurnLanes(edge)) {
        edge.set_turnlanes(GetTurnLanes(edge));
      }
      
      // Update the edge in the builder
      tile_builder.update_directededge(edge);
    }
    
    // Save the enhanced tile
    tile_builder.StoreTileData();
  }
}
```

The enhancement phase is where much of the routing intelligence is added to the graph. By analyzing the topology and attributes of the road network, Mjolnir can infer additional properties that aren't explicitly tagged in OSM data.

## Complex Restriction Building

Turn restrictions (e.g., no left turn, no U-turn) are crucial for accurate routing. Mjolnir processes these using the `ComplexRestrictionBuilder`:

```cpp
// From valhalla/mjolnir/complexrestrictionbuilder.h
class ComplexRestrictionBuilder {
public:
  ComplexRestrictionBuilder();
  
  // Set various attributes
  void set_from_id(const GraphId& id);
  void set_to_id(const GraphId& id);
  void set_via_id(const GraphId& id);
  void set_type(const RestrictionType type);
  // ...
};
```

The implementation in `src/mjolnir/complexrestrictionbuilder.cc` handles the conversion from OSM restriction relations to Valhalla's internal representation. Restrictions can be:
- Simple (from one edge to another)
- Complex (involving multiple via edges or nodes)
- Time-dependent (only active during certain hours)
- Mode-specific (only applying to certain vehicle types)

## Hierarchical Graph Building

Valhalla's multi-level graph is created by the `HierarchyBuilder`:

```cpp
// From valhalla/mjolnir/hierarchybuilder.h
class HierarchyBuilder {
public:
  static void Build(const boost::property_tree::ptree& pt);
};
```

The process involves:

1. **Edge Classification**: Determining which edges belong to which hierarchy level
2. **Shortcut Creation**: Creating direct connections between important nodes to bypass less important roads
3. **Transition Edge Creation**: Adding special edges to connect between hierarchy levels

Here's a simplified version of the hierarchy building process:

```cpp
// From src/mjolnir/hierarchybuilder.cc
void HierarchyBuilder::Build(const boost::property_tree::ptree& pt) {
  // Create level 0 (local) graph from the enhanced base graph
  CreateLevel0(pt);
  
  // Create level 1 (regional) graph
  CreateLevel1(pt);
  
  // Create level 2 (highway) graph
  CreateLevel2(pt);
  
  // Add transitions between levels
  AddTransitions(pt);
}

void HierarchyBuilder::CreateLevel1(const boost::property_tree::ptree& pt) {
  // For each tile in level 0
  for (const auto& tile_id : level0_tile_ids) {
    GraphTileBuilder level1_builder;
    
    // Get the level 0 tile
    const GraphTile* tile = reader.GetGraphTile(tile_id);
    
    // For each edge in the level 0 tile
    for (const auto& edge : tile->directededges()) {
      // If edge is important enough for level 1
      if (ShouldIncludeInLevel1(edge)) {
        // Add to level 1 graph
        level1_builder.AddEdge(edge);
      }
    }
    
    // Create shortcuts for level 1
    CreateShortcuts(level1_builder);
    
    // Save the level 1 tile
    level1_builder.StoreTileData();
  }
}
```

The hierarchical structure is what enables Valhalla to efficiently route over long distances. By using higher-level tiles with shortcuts, the routing algorithm can quickly traverse large areas without examining every local road.

## Shortcut Building

Shortcuts are special edges that represent multiple underlying edges, allowing for faster routing on higher hierarchy levels. They're created by the `ShortcutBuilder`:

```cpp
// From valhalla/mjolnir/shortcutbuilder.h
class ShortcutBuilder {
public:
  static void Build(const boost::property_tree::ptree& pt);
};
```

The implementation in `src/mjolnir/shortcutbuilder.cc` identifies paths that can be represented as shortcuts and creates the corresponding edges. Shortcuts preserve important routing properties like:
- Total distance
- Travel time
- Access restrictions
- Turn restrictions at endpoints

## Tile Creation and Serialization

The final step is to create the actual tile files that will be used by the routing engine. This is handled by the `GraphTileBuilder`:

```cpp
// From valhalla/mjolnir/graphtilebuilder.h
class GraphTileBuilder {
public:
  GraphTileBuilder(const GraphId& graphid, ...);
  
  // Add elements to the tile
  void AddNodeAccessRestriction(const NodeAccessRestriction& access);
  void AddNodeAdmin(const uint32_t node_index, const uint32_t admin_index);
  void AddAdmin(const std::string& country_name, const std::string& state_name, ...);
  // ...
  
  // Serialize the tile to disk
  void StoreTileData();
};
```

The serialization process writes the tile data in a binary format optimized for quick loading and minimal memory usage. The tiles are stored in a directory structure that reflects their hierarchy level and ID, making it easy to locate and load specific tiles during routing.

## The Complete Pipeline

Putting it all together, here's how the complete graph building pipeline is orchestrated in Valhalla:

```cpp
// From src/valhalla_build_tiles.cc
int main(int argc, char** argv) {
  // Parse command line arguments
  // ...
  
  // Parse OSM data
  OSMPBFParser parser;
  parser.Parse(input_files);
  
  // Build the initial graph
  GraphBuilder::Build(config, ...);
  
  // Enhance the graph with additional attributes
  GraphEnhancer::Enhance(config, ...);
  
  // Build restrictions
  RestrictionBuilder::Build(config, ...);
  
  // Build the hierarchical graph
  HierarchyBuilder::Build(config);
  
  // Build shortcuts for faster routing
  ShortcutBuilder::Build(config);
  
  // Add transit data if available
  if (include_transit) {
    TransitBuilder::Build(config);
  }
  
  // Validate the final graph
  GraphValidator::Validate(config);
  
  return 0;
}
```

This pipeline transforms raw OSM data into a sophisticated, hierarchical routing graph that enables Valhalla's efficient routing capabilities. The process can be run on modest hardware for regional extracts, or on more powerful systems for processing planet-scale data.

Understanding this process is essential for building a graph tile builder, as it provides the blueprint for transforming raw geographic data into a structure optimized for routing algorithms.
