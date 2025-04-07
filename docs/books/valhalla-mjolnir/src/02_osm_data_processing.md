# OSM Data Processing

## Overview of OpenStreetMap Data

OpenStreetMap (OSM) is the primary data source for Valhalla. OSM data consists of three basic elements:

1. **Nodes**: Points in space defined by latitude and longitude
2. **Ways**: Ordered lists of nodes that represent linear features like roads, rivers, and boundaries
3. **Relations**: Groups of nodes, ways, and other relations with specific roles (e.g., turn restrictions, routes)

OSM data is typically distributed in one of two formats:
- **PBF** (Protocol Buffer Format): A compact binary format
- **XML**: A more verbose text-based format

Mjolnir primarily works with PBF files due to their efficiency in storage and parsing speed.

## The OSM PBF Parser

Mjolnir includes a parser for OSM PBF files. The main class responsible for this is `OSMPBFParser`:

```cpp
// From valhalla/mjolnir/osmpbfparser.h
class OSMPBFParser {
public:
  OSMPBFParser(const boost::property_tree::ptree& pt);
  
  bool Parse(const boost::property_tree::ptree& pt, 
             const std::vector<std::string>& input_files);
  
private:
  // Internal parsing methods
  void ParseNodes(const OSMPBF::PrimitiveBlock& primblock);
  void ParseWays(const OSMPBF::PrimitiveBlock& primblock);
  void ParseRelations(const OSMPBF::PrimitiveBlock& primblock);
  // ...
};
```

The parser reads PBF files in blocks, processing nodes, ways, and relations in sequence. It uses the Google Protocol Buffers library to decode the binary format.

## Data Extraction Process

The data extraction process follows these steps:

1. **Initialize Data Structures**: Set up containers for nodes, ways, and relations
2. **Parse PBF Files**: Read and decode the binary data
3. **Filter Data**: Keep only relevant elements for routing (e.g., roads, paths)
4. **Build Internal Representations**: Convert OSM elements to Valhalla's internal data structures

Let's look at how Mjolnir processes each type of OSM element:

### Node Processing

Nodes are processed first, as they form the foundation of the graph:

```cpp
// From src/mjolnir/pbfgraphparser.cc
void PBFGraphParser::ParseNodes(const OSMPBF::PrimitiveBlock& primblock) {
  // For each primitive group
  for (int i = 0; i < primblock.primitivegroup_size(); ++i) {
    const OSMPBF::PrimitiveGroup& pg = primblock.primitivegroup(i);
    
    // Dense nodes
    if (pg.has_dense()) {
      const OSMPBF::DenseNodes& dn = pg.dense();
      // Process dense nodes...
      
      // Store relevant node information
      for (uint64_t i = 0; i < dn.id_size(); ++i) {
        // Extract node ID, lat, lon
        // Store in OSMNode structure
        // ...
      }
    }
    
    // Regular nodes
    for (int j = 0; j < pg.nodes_size(); ++j) {
      const OSMPBF::Node& node = pg.nodes(j);
      // Process regular node...
    }
  }
}
```

Nodes are converted to Valhalla's internal `OSMNode` structure:

```cpp
// From valhalla/mjolnir/osmnode.h
struct OSMNode {
  OSMNode() = delete;
  OSMNode(const uint64_t id, const double lat, const double lng)
      : node_id(id), access(0), type(NodeType::kStreetIntersection), 
        intersection(false), traffic_signal(false), forward_signal(false),
        backward_signal(false), non_link_edge(false), link_edge(false),
        shortlink(false), non_ferry_edge(false), ferry_edge(false),
        flat_loop(false), urban(false), tagged_access(false), 
        private_access(false), cash_only_toll(false), 
        lat(lat), lng(lng) {
  }
  
  uint64_t node_id;
  uint16_t access;
  NodeType type;
  // ... many more fields ...
};
```

### Way Processing

After nodes, ways are processed to identify road segments:

```cpp
// From src/mjolnir/pbfgraphparser.cc
void PBFGraphParser::ParseWays(const OSMPBF::PrimitiveBlock& primblock) {
  // For each primitive group
  for (int i = 0; i < primblock.primitivegroup_size(); ++i) {
    const OSMPBF::PrimitiveGroup& pg = primblock.primitivegroup(i);
    
    // Ways
    for (int j = 0; j < pg.ways_size(); ++j) {
      const OSMPBF::Way& way = pg.ways(j);
      
      // Get tags
      std::vector<Tag> tags = GetTagsFromWay(primblock, way);
      
      // Check if this way is a road
      if (tags.size() && (IsDriveable(tags) || IsFootway(tags) || IsBicycle(tags))) {
        // Process way for routing
        // ...
      }
    }
  }
}
```

Ways are converted to Valhalla's internal `OSMWay` structure:

```cpp
// From valhalla/mjolnir/osmway.h
class OSMWay {
public:
  OSMWay() = default;
  
  // Set way attributes based on tags
  void set_node_ids(const std::vector<uint64_t>& node_ids);
  void set_name(const std::string& name);
  void set_road_class(const RoadClass roadclass);
  // ... many more setters ...
  
  // Get way attributes
  const std::vector<uint64_t>& node_ids() const;
  const std::string& name() const;
  RoadClass road_class() const;
  // ... many more getters ...
  
private:
  std::vector<uint64_t> node_ids_;
  std::string name_;
  std::string name_en_;
  std::string alt_name_;
  std::string official_name_;
  // ... many more fields ...
};
```

### Relation Processing

Finally, relations are processed to capture complex structures like turn restrictions:

```cpp
// From src/mjolnir/pbfgraphparser.cc
void PBFGraphParser::ParseRelations(const OSMPBF::PrimitiveBlock& primblock) {
  // For each primitive group
  for (int i = 0; i < primblock.primitivegroup_size(); ++i) {
    const OSMPBF::PrimitiveGroup& pg = primblock.primitivegroup(i);
    
    // Relations
    for (int j = 0; j < pg.relations_size(); ++j) {
      const OSMPBF::Relation& relation = pg.relations(j);
      
      // Get tags
      std::vector<Tag> tags = GetTagsFromRelation(primblock, relation);
      
      // Check if this is a restriction relation
      if (IsRestriction(tags)) {
        // Process restriction
        // ...
      }
    }
  }
}
```

Relations are converted to Valhalla's internal `OSMRestriction` structure:

```cpp
// From valhalla/mjolnir/osmrestriction.h
struct OSMRestriction {
  OSMRestriction() = default;
  
  uint64_t from_way_id;
  uint64_t to_way_id;
  uint64_t via_node_id;
  uint64_t via_way_id;
  
  RestrictionType type;
  uint32_t day_on;
  uint32_t day_off;
  uint32_t hour_on;
  uint32_t hour_off;
};
```

## Tag Processing and Transformation

OSM tags (key-value pairs) contain information about road properties. Mjolnir processes these tags to extract:

- Road classification (highway, residential, footway, etc.)
- Access restrictions (vehicle types allowed)
- Speed limits
- Direction of travel (one-way, bidirectional)
- Surface type and quality
- And many more attributes

Mjolnir includes a tag transformation system that can be customized using Lua scripts:

```cpp
// From valhalla/mjolnir/luatagtransform.h
class LuaTagTransform {
public:
  LuaTagTransform(const std::string& lua_script);
  ~LuaTagTransform();
  
  // Transform tags for ways
  bool Transform(OSMWay& way, const Tags& tags);
  
  // Transform tags for nodes
  bool Transform(OSMNode& node, const Tags& tags);
  
private:
  lua_State* lua_state_;
};
```

This allows for flexible customization of how OSM tags are interpreted without modifying the C++ code. The Lua scripts transform the schemaless OSM tags into a structured set of values that can be efficiently processed by the C++ code.

## Data Filtering

Not all OSM data is relevant for routing. Mjolnir applies filters to focus on the elements that matter:

1. **Way Filtering**: Only ways with highway tags or other routable features are kept
2. **Node Filtering**: Nodes that aren't part of routable ways are discarded
3. **Relation Filtering**: Only relations relevant to routing (like restrictions) are processed

This filtering significantly reduces the amount of data that needs to be processed in subsequent steps.

## Memory Management During Parsing

Processing planet-scale OSM data requires careful memory management. Mjolnir uses several strategies:

1. **Streaming Processing**: Data is processed in chunks rather than loading everything at once
2. **Memory-Mapped Files**: For efficient access to large datasets
3. **Custom Memory Pools**: For allocating many small objects efficiently

```cpp
// Example of memory-mapped file usage
void OSMPBFParser::Parse(const std::string& filename) {
  // Memory map the file
  boost::iostreams::mapped_file_source file;
  file.open(filename);
  
  // Process the data in chunks
  const char* data = file.data();
  const size_t size = file.size();
  
  // ... process data ...
  
  // File is automatically unmapped when file goes out of scope
}
```

Understanding how Mjolnir processes OSM data is essential for building a graph tile builder, as it forms the foundation for all subsequent graph construction steps.
