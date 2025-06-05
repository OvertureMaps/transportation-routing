# Valhalla Binary Format Analysis

## Overview

This document provides a detailed analysis of Valhalla's binary file formats used in the Mjolnir graph tile builder. These formats are essential for understanding how to create compatible binary files from Overture Maps data.

## Key Binary Files

Valhalla uses three main binary files during the graph construction process:

1. **ways.bin** - Contains OSMWay structures
2. **nodes.bin** - Contains Node structures  
3. **way_nodes.bin** - Contains OSMWayNode structures

## Binary Format Foundation

### Sequence Template

All binary files use Valhalla's `sequence<T>` template class defined in `valhalla/midgard/sequence.h`. This template:

- Memory-maps binary files for efficient access
- Stores POD (Plain Old Data) structures directly as binary data
- Provides random access to elements via indexing
- Handles both reading and writing operations
- Uses native endianness (no byte swapping)
- Requires no headers or metadata - just raw struct data

### File Structure

```
Binary File = [Struct1][Struct2][Struct3]...[StructN]
```

Each file is simply a contiguous array of C++ structures written directly to disk.

## OSMWay Structure (ways.bin)

### Header Definition
Located in: `valhalla/mjolnir/osmway.h`

### Key Fields Analysis

```cpp
struct OSMWay {
  uint64_t osmwayid_;           // OSM way ID
  uint32_t nodecount_;          // Number of nodes in this way
  uint8_t speed_;               // Speed in KPH
  uint8_t speed_limit_;         // Speed limit in KPH
  uint8_t backward_speed_;      // Backward speed in KPH
  uint8_t forward_speed_;       // Forward speed in KPH
  // ... many more fields for road attributes
};
```

### Size and Alignment
- The structure uses `memset(this, 0, sizeof(OSMWay))` for initialization
- All fields are packed with specific bit-field layouts
- Total size is approximately 200+ bytes per way

### Critical Fields for Overture Integration
- `osmwayid_`: Must be unique identifier from Overture segment
- `nodecount_`: Number of shape points in the segment
- Speed fields: Derived from Overture speed attributes
- Access and restriction fields: Mapped from Overture access rules

## OSMNode Structure (nodes.bin)

### Header Definition
Located in: `valhalla/mjolnir/osmnode.h`

### Key Fields Analysis

```cpp
struct OSMNode {
  uint64_t osmid_;              // OSM node ID
  
  // Bit-packed fields for names and references
  uint64_t name_index_ : 21;
  uint64_t ref_index_ : 21;
  uint64_t exit_to_index_ : 21;
  uint64_t named_intersection_ : 1;
  
  // More bit-packed fields for country, signals, signs
  uint64_t country_iso_index_ : 21;
  uint64_t state_iso_index_ : 21;
  uint64_t traffic_signal_ : 1;
  uint64_t stop_sign_ : 1;
  uint64_t yield_sign_ : 1;
  // ... more traffic control fields
  
  uint32_t access_ : 12;        // Access restrictions
  uint32_t type_ : 4;           // Node type
  uint32_t intersection_ : 1;   // Is intersection
  // ... more node attributes
  
  uint32_t bss_info_;           // Bike share station info
  uint32_t linguistic_info_index_;
  
  // Coordinates at 7-digit precision
  uint32_t lng7_;               // Longitude * 1e7 + 180*1e7
  uint32_t lat7_;               // Latitude * 1e7 + 90*1e7
};
```

### Coordinate Encoding
```cpp
void set_latlng(double lng, double lat) {
  lng = std::round((lng + 180) * 1e7);
  lng7_ = (lng >= 0 && lng <= 360 * 1e7) ? lng : std::numeric_limits<uint32_t>::max();
  
  lat = std::round((lat + 90) * 1e7);
  lat7_ = (lat >= 0 && lat <= 180 * 1e7) ? lat : std::numeric_limits<uint32_t>::max();
}
```

### Size and Alignment
- Heavily uses bit-fields to pack data efficiently
- Total size is approximately 40-50 bytes per node
- Uses `memset(this, 0, sizeof(OSMNode))` for initialization

## OSMWayNode Structure (way_nodes.bin)

### Header Definition
Located in: `valhalla/mjolnir/osmdata.h`

### Structure Analysis

```cpp
struct OSMWayNode {
  OSMNode node;                    // Full OSMNode structure
  uint32_t way_index = 0;          // Index into ways.bin
  uint32_t way_shape_node_index = 0; // Position within way's shape
};
```

### Purpose
- Links nodes to their parent ways
- Provides shape point ordering within ways
- Enables traversal from nodes to ways and vice versa

### Size
- Size = sizeof(OSMNode) + 8 bytes
- Approximately 48-58 bytes per way node

## Binary File Relationships

### Data Flow
```
OSM/Overture Data → PBF Parser → OSMData → Binary Files → ConstructEdges → Graph Tiles
```

### File Dependencies
1. **ways.bin**: Independent, contains way definitions
2. **nodes.bin**: Independent, contains node definitions  
3. **way_nodes.bin**: References both ways.bin (via way_index) and contains embedded nodes

### Index Relationships
- `OSMWayNode.way_index` → Index into ways.bin
- `OSMWayNode.way_shape_node_index` → Position within way's node sequence
- Various `*_index` fields → Indices into string tables and other auxiliary data

## Integration Point: ConstructEdges

### Function Signature
```cpp
void ConstructEdges(const std::string& ways_file,
                   const std::string& way_nodes_file,
                   const std::string& nodes_file,
                   const std::string& edges_file,
                   const std::function<GraphId(const OSMNode&)>& graph_id_predicate,
                   const std::function<uint32_t(const OSMNode&)>& grid_id_predicate,
                   const bool infer_turn_channels);
```

### File Usage
```cpp
sequence<OSMWay> ways(ways_file, false);           // Read ways.bin
sequence<OSMWayNode> way_nodes(way_nodes_file, false); // Read way_nodes.bin
sequence<Edge> edges(edges_file, true);           // Write edges
sequence<Node> nodes(nodes_file, true);           // Write nodes
```

### Processing Logic
1. Iterate through way_nodes.bin sequentially
2. Group way nodes by way_index to reconstruct complete ways
3. Validate node coordinates and way geometry
4. Create graph edges from way segments
5. Assign nodes to spatial tiles
6. Write output to edges and nodes files

## Technical Requirements

### Endianness
- Uses native system endianness
- No byte swapping performed
- Files are platform-specific

### Alignment
- Structures use natural C++ alignment
- No special packing directives beyond bit-fields
- Memory layout matches in-memory representation

### File Size Estimation
For a dataset with:
- 1M ways: ~200MB ways.bin
- 10M nodes: ~400MB nodes.bin  
- 50M way nodes: ~2.5GB way_nodes.bin

### Memory Requirements
- Files are memory-mapped for efficient access
- Virtual memory usage equals file size
- Physical memory usage depends on access patterns

## Implementation Considerations for Overture

### Data Mapping Challenges
1. **ID Translation**: Overture segment/connector IDs → OSM-style IDs
2. **Coordinate Precision**: Ensure 7-digit precision encoding
3. **Attribute Mapping**: Overture schema → OSM tag equivalents
4. **Topology**: Overture connectors → OSM intersection nodes

### Required Transformations
1. **Segments → Ways**: Map Overture segments to OSMWay structures
2. **Connectors → Nodes**: Map Overture connectors to OSMNode structures  
3. **Shape Points → WayNodes**: Create OSMWayNode entries for all shape points
4. **Indexing**: Maintain consistent indexing between files

### Validation Requirements
1. **Coordinate Validity**: All coordinates must be valid lat/lng
2. **Index Consistency**: way_index must reference valid ways
3. **Node Count Matching**: OSMWay.nodecount_ must match actual nodes
4. **Spatial Coherence**: Connected ways must share nodes

## Next Steps for Implementation

### Phase 1: Structure Definition
1. Create Rust equivalents of OSMWay, OSMNode, OSMWayNode
2. Implement binary serialization/deserialization
3. Create test cases with known data

### Phase 2: Data Mapping
1. Design Overture → OSM attribute mapping
2. Implement coordinate transformation
3. Handle ID generation and management

### Phase 3: File Generation
1. Implement binary file writers
2. Create validation tools
3. Test with ConstructEdges integration

### Phase 4: Integration Testing
1. Generate test files from sample Overture data
2. Verify ConstructEdges can process the files
3. Validate resulting graph tiles

## Compatibility Notes

### Version Considerations
- Binary format is tied to specific Valhalla versions
- Structure layouts may change between releases
- Always target a specific Valhalla version

### Platform Dependencies
- Files are not portable between different architectures
- Endianness and alignment differences affect compatibility
- Generate files on target platform when possible

## References

### Key Source Files
- `valhalla/mjolnir/osmway.h` - OSMWay structure definition
- `valhalla/mjolnir/osmnode.h` - OSMNode structure definition  
- `valhalla/mjolnir/osmdata.h` - OSMWayNode and container definitions
- `valhalla/midgard/sequence.h` - Binary file I/O template
- `src/mjolnir/graphbuilder.cc` - ConstructEdges implementation
- `src/mjolnir/util.cc` - File path constants and utilities

### Integration Points
- `ConstructEdges()` function - Primary integration target
- `OSMData` structure - Container for all parsed data
- Binary file naming conventions in util.cc
