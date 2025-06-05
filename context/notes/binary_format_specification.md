# Valhalla Binary Format Technical Specification

## File Format Overview

Valhalla uses three binary files with simple, direct struct serialization:

### File Structure
```
[Header: None]
[Data: Raw C++ structs written sequentially]
[Footer: None]
```

## Binary Encoding Details

### Endianness
- **Native system endianness** (typically little-endian on x86/x64)
- No byte swapping performed
- Files are platform-specific

### Alignment
- Natural C++ struct alignment
- No special packing beyond bit-fields
- Padding follows compiler defaults

### Data Types
- `uint64_t`: 8 bytes
- `uint32_t`: 4 bytes  
- `uint8_t`: 1 byte
- Bit-fields: Packed within containing integer types

## ways.bin Format

### Record Structure
Each record is an `OSMWay` struct (~200 bytes):

```cpp
struct OSMWay {
  uint64_t osmwayid_;           // 8 bytes: Way identifier
  uint32_t nodecount_;          // 4 bytes: Number of nodes
  uint8_t speed_;               // 1 byte: Speed in KPH
  uint8_t speed_limit_;         // 1 byte: Speed limit in KPH
  uint8_t backward_speed_;      // 1 byte: Backward speed
  uint8_t forward_speed_;       // 1 byte: Forward speed
  // ... additional fields (see osmway.h for complete structure)
};
```

### File Layout
```
[OSMWay Record 0]
[OSMWay Record 1]
[OSMWay Record 2]
...
[OSMWay Record N-1]
```

## nodes.bin Format

### Record Structure  
Each record is a `Node` struct (written by ConstructEdges, not OSMNode):

```cpp
struct Node {
  // Node data written by ConstructEdges
  // Exact structure varies - see graphbuilder.cc
};
```

**Note**: This file is OUTPUT from ConstructEdges, not input.

## way_nodes.bin Format

### Record Structure
Each record is an `OSMWayNode` struct (~50 bytes):

```cpp
struct OSMWayNode {
  OSMNode node;                    // ~40-50 bytes: Full node data
  uint32_t way_index;              // 4 bytes: Index into ways.bin
  uint32_t way_shape_node_index;   // 4 bytes: Position in way shape
};
```

### OSMNode Substructure
```cpp
struct OSMNode {
  uint64_t osmid_;                 // 8 bytes: Node ID
  
  // First 64-bit packed field
  uint64_t name_index_ : 21;       // String table index
  uint64_t ref_index_ : 21;        // Reference string index
  uint64_t exit_to_index_ : 21;    // Exit destination index
  uint64_t named_intersection_ : 1; // Boolean flag
  
  // Second 64-bit packed field  
  uint64_t country_iso_index_ : 21;
  uint64_t state_iso_index_ : 21;
  uint64_t traffic_signal_ : 1;
  uint64_t forward_signal_ : 1;
  uint64_t backward_signal_ : 1;
  uint64_t stop_sign_ : 1;
  uint64_t forward_stop_ : 1;
  uint64_t backward_stop_ : 1;
  uint64_t yield_sign_ : 1;
  uint64_t forward_yield_ : 1;
  uint64_t backward_yield_ : 1;
  uint64_t minor_ : 1;
  uint64_t direction_ : 1;
  uint64_t spare_ : 11;
  
  // 32-bit packed field
  uint32_t access_ : 12;
  uint32_t type_ : 4;
  uint32_t intersection_ : 1;
  uint32_t non_link_edge_ : 1;
  uint32_t link_edge_ : 1;
  uint32_t shortlink_ : 1;
  uint32_t non_ferry_edge_ : 1;
  uint32_t ferry_edge_ : 1;
  uint32_t flat_loop_ : 1;
  uint32_t urban_ : 1;
  uint32_t tagged_access_ : 1;
  uint32_t private_access_ : 1;
  uint32_t cash_only_toll_ : 1;
  uint32_t spare1_ : 5;
  
  uint32_t bss_info_;              // 4 bytes: Bike share info
  uint32_t linguistic_info_index_; // 4 bytes: Language info index
  uint32_t lng7_;                  // 4 bytes: Longitude * 1e7 + 180*1e7
  uint32_t lat7_;                  // 4 bytes: Latitude * 1e7 + 90*1e7
};
```

### File Layout
```
[OSMWayNode Record 0]
[OSMWayNode Record 1]
[OSMWayNode Record 2]
...
[OSMWayNode Record N-1]
```

## Coordinate Encoding

### Precision
- 7 decimal places (centimeter precision)
- Stored as 32-bit unsigned integers

### Longitude Encoding
```cpp
encoded_lng = round((longitude + 180.0) * 1e7)
```
- Range: 0 to 360*1e7 (3,600,000,000)
- Invalid coordinates: `std::numeric_limits<uint32_t>::max()`

### Latitude Encoding  
```cpp
encoded_lat = round((latitude + 90.0) * 1e7)
```
- Range: 0 to 180*1e7 (1,800,000,000)
- Invalid coordinates: `std::numeric_limits<uint32_t>::max()`

## Index Relationships

### way_nodes.bin → ways.bin
- `OSMWayNode.way_index` is 0-based index into ways.bin
- Must be valid index: `0 <= way_index < ways_count`

### Shape Point Ordering
- `OSMWayNode.way_shape_node_index` indicates position within way
- For a way with N nodes: indices are 0, 1, 2, ..., N-1
- Must match `OSMWay.nodecount_`

## File Size Calculations

### ways.bin
```
size = num_ways * sizeof(OSMWay)
     ≈ num_ways * 200 bytes
```

### way_nodes.bin  
```
size = total_shape_points * sizeof(OSMWayNode)
     ≈ total_shape_points * 50 bytes
```

## Validation Requirements

### Structural Validation
1. File size must be multiple of struct size
2. All indices must be within valid ranges
3. Coordinates must be valid or marked invalid

### Logical Validation
1. Each way's node count must match actual nodes in way_nodes.bin
2. way_shape_node_index must be sequential within each way
3. All referenced ways must exist in ways.bin

### Coordinate Validation
```cpp
bool is_valid_coordinate(uint32_t encoded) {
  return encoded != std::numeric_limits<uint32_t>::max();
}
```

## Implementation Notes

### Memory Layout
- Structs use compiler's natural alignment
- Bit-fields are packed within their containing integer
- No explicit padding control needed

### Initialization
- All structs use `memset(this, 0, sizeof(Struct))` 
- Ensures consistent zero-initialization of padding bytes
- Critical for binary compatibility

### Platform Considerations
- Files are not portable between architectures
- Different endianness requires regeneration
- Struct padding may vary between compilers
