# Appendix: Technical Details for Overture Graph Tiles RFC

This appendix provides additional technical details to supplement the Overture Graph Tiles RFC.

## Appendix A: Valhalla Integration Details

### A.1 Binary File Formats

Valhalla uses several binary files during its graph building process:

1. **ways.bin**: Contains serialized `OSMWay` structures
   ```cpp
   struct OSMWay {
     uint64_t osmwayid_;
     uint64_t node_count_;
     uint8_t road_class_;
     uint8_t use_;
     uint8_t lanes_;
     uint8_t forward_lanes_;
     uint8_t backward_lanes_;
     uint32_t speed_;
     // Additional attributes...
   };
   ```

2. **nodes.bin**: Contains serialized `OSMNode` structures
   ```cpp
   struct OSMNode {
     uint64_t node_id;
     uint16_t access;
     NodeType type;
     bool intersection;
     bool traffic_signal;
     // Additional attributes...
     double lat;
     double lng;
   };
   ```

3. **way_nodes.bin**: Contains serialized `OSMWayNode` structures
   ```cpp
   struct OSMWayNode {
     uint64_t way_id;
     uint64_t node_id;
   };
   ```

### A.2 Rust Implementation for Binary File Generation

```rust
// Example Rust code for generating ways.bin
pub struct OSMWay {
    osmwayid: u64,
    node_count: u64,
    road_class: u8,
    use_type: u8,
    lanes: u8,
    forward_lanes: u8,
    backward_lanes: u8,
    speed: u32,
    // Additional fields...
}

impl OSMWay {
    pub fn from_overture_segment(segment: &Segment) -> Self {
        // Convert Overture segment to OSMWay
        OSMWay {
            osmwayid: segment.id.parse().unwrap_or(0),
            node_count: segment.geometry.coordinates.len() as u64,
            road_class: map_road_class(&segment.properties.class),
            // Map other attributes...
        }
    }
    
    pub fn write_to_binary<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        // Write binary representation
        writer.write_all(&self.osmwayid.to_le_bytes())?;
        writer.write_all(&self.node_count.to_le_bytes())?;
        // Write other fields...
        Ok(())
    }
}
```

### A.3 Valhalla Integration Command Line

```bash
# Example command line for Valhalla integration
# 1. Convert Overture data to Valhalla binary files
overture-transcoder --input overture-data.parquet --output-dir ./valhalla_tiles

# 2. Run Valhalla tile builder starting from ConstructEdges phase
valhalla_build_tiles --config ./valhalla.json --start construct_edges --end cleanup
```

## Appendix B: Overture Graph Tile Format Details

### B.1 Tile Indexing Scheme

Overture Graph Tiles will use a hierarchical indexing scheme:

```
┌─────────────────────────────────────────────────────────┐
│ Tile ID Structure                                       │
├─────────────────────────────────────────────────────────┤
│ Level (4 bits) | X Coordinate (24 bits) | Y Coordinate (24 bits) │
└─────────────────────────────────────────────────────────┘
```

- **Level**: Hierarchy level (0-15)
  - Level 0: Highway network
  - Level 1: Arterial network
  - Level 2: Local network
  - Additional levels as needed

- **X/Y Coordinates**: Based on a global grid system
  - Higher levels (lower numbers) have larger tiles
  - Each level increases resolution by a factor of 4

### B.2 Node Structure

```rust
struct Node {
    // Node identifier
    id: u64,
    
    // Geographic location
    lat: f32,
    lng: f32,
    
    // Node type (intersection, dead-end, etc.)
    node_type: u8,
    
    // Access restrictions
    access: u16,
    
    // Index to first outgoing edge
    edge_index: u32,
    
    // Number of outgoing edges
    edge_count: u16,
    
    // Additional attributes
    traffic_signal: bool,
    stop_sign: bool,
    administrative_index: u32,
}
```

### B.3 Directed Edge Structure

```rust
struct DirectedEdge {
    // Edge identifier
    id: u64,
    
    // Connected nodes
    start_node: u32,
    end_node: u32,
    
    // Physical properties
    length_meters: f32,
    speed_kph: u8,
    
    // Classification
    road_class: u8,
    use_type: u8,
    
    // Access restrictions
    access: u16,
    
    // Indices to additional data
    shape_index: u32,
    name_index: u32,
    
    // Routing properties
    forward: bool,
    toll: bool,
    seasonal: bool,
    destination_only: bool,
    
    // Turn restrictions
    restriction_index: u32,
}
```

### B.4 Binary Tile Format

The binary tile format will use a structured layout:

```
┌─────────────────────────────────────────────────────────┐
│ Tile Header (fixed size)                                │
├─────────────────────────────────────────────────────────┤
│ Node Count                                              │
│ Edge Count                                              │
│ Name Count                                              │
│ Shape Count                                             │
│ ...                                                     │
├─────────────────────────────────────────────────────────┤
│ Nodes Array (fixed size per entry)                      │
├─────────────────────────────────────────────────────────┤
│ Directed Edges Array (fixed size per entry)             │
├─────────────────────────────────────────────────────────┤
│ Names Table (variable size)                             │
├─────────────────────────────────────────────────────────┤
│ Shapes Array (variable size)                            │
├─────────────────────────────────────────────────────────┤
│ Additional Data (variable size)                         │
└─────────────────────────────────────────────────────────┘
```

## Appendix C: Performance Considerations

### C.1 Memory Usage Optimization

To optimize memory usage during tile creation and routing:

1. **Streaming Processing**:
   ```rust
   fn process_overture_data<R: Read>(reader: R) -> Result<Vec<GraphTile>, Error> {
       let mut tiles = HashMap::new();
       let mut parser = GeoParquetReader::new(reader);
       
       while let Some(feature) = parser.next_feature()? {
           let tile_id = calculate_tile_id(feature.geometry);
           let tile = tiles.entry(tile_id).or_insert_with(|| GraphTile::new(tile_id));
           tile.add_feature(feature);
       }
       
       Ok(tiles.into_values().collect())
   }
   ```

2. **Parallel Processing**:
   ```rust
   fn create_tiles_in_parallel(data_path: &str) -> Result<(), Error> {
       let file = File::open(data_path)?;
       let reader = BufReader::new(file);
       
       // Split data into chunks
       let chunks = split_into_chunks(reader)?;
       
       // Process chunks in parallel
       let results: Vec<_> = chunks.par_iter()
           .map(|chunk| process_chunk(chunk))
           .collect();
           
       // Merge results
       merge_tiles(results)
   }
   ```

### C.2 Routing Performance Optimization

To optimize routing performance:

1. **Hierarchical Routing**:
   ```rust
   fn find_path(start: LatLng, end: LatLng) -> Result<Path, Error> {
       // Find closest nodes in the highest level (highways)
       let (start_high, end_high) = find_closest_high_level_nodes(start, end);
       
       // Route on highway network
       let high_path = route_on_level(start_high, end_high, Level::Highway);
       
       // Connect start and end to highway network
       let start_connection = connect_to_highway(start, start_high);
       let end_connection = connect_to_highway(end, end_high);
       
       // Combine paths
       combine_paths(start_connection, high_path, end_connection)
   }
   ```

2. **Tile Caching**:
   ```rust
   struct TileCache {
       tiles: LruCache<TileId, GraphTile>,
       max_size: usize,
   }
   
   impl TileCache {
       fn get_tile(&mut self, id: TileId) -> Result<&GraphTile, Error> {
           if !self.tiles.contains(&id) {
               let tile = load_tile_from_disk(id)?;
               self.tiles.put(id, tile);
           }
           Ok(self.tiles.get(&id).unwrap())
       }
   }
   ```

## Appendix D: Integration with Multiple Routing Engines

### D.1 Valhalla Integration

```rust
fn convert_to_valhalla(tile: &OvertureGraphTile) -> Result<ValhallaTile, Error> {
    let mut valhalla_tile = ValhallaTile::new(tile.header.tile_id);
    
    // Convert nodes
    for node in &tile.nodes {
        let valhalla_node = convert_node_to_valhalla(node);
        valhalla_tile.add_node(valhalla_node);
    }
    
    // Convert edges
    for edge in &tile.directed_edges {
        let valhalla_edge = convert_edge_to_valhalla(edge);
        valhalla_tile.add_edge(valhalla_edge);
    }
    
    // Convert additional data
    // ...
    
    Ok(valhalla_tile)
}
```

### D.2 OSRM Integration

```rust
fn convert_to_osrm(tiles: &[OvertureGraphTile]) -> Result<OSRMData, Error> {
    let mut osrm_data = OSRMData::new();
    
    // Convert nodes
    for tile in tiles {
        for node in &tile.nodes {
            let osrm_node = convert_node_to_osrm(node);
            osrm_data.add_node(osrm_node);
        }
    }
    
    // Convert edges
    for tile in tiles {
        for edge in &tile.directed_edges {
            let osrm_edge = convert_edge_to_osrm(edge);
            osrm_data.add_edge(osrm_edge);
        }
    }
    
    // Build OSRM hierarchy
    osrm_data.build_hierarchy();
    
    Ok(osrm_data)
}
```

### D.3 GraphHopper Integration

```rust
fn convert_to_graphhopper(tiles: &[OvertureGraphTile]) -> Result<GraphHopperData, Error> {
    let mut gh_data = GraphHopperData::new();
    
    // Convert nodes and edges
    for tile in tiles {
        for node in &tile.nodes {
            gh_data.add_node(convert_node_to_gh(node));
        }
        
        for edge in &tile.directed_edges {
            gh_data.add_edge(convert_edge_to_gh(edge));
        }
    }
    
    // Build GraphHopper specific structures
    gh_data.build_index();
    
    Ok(gh_data)
}
```

## Appendix E: Testing and Validation

### E.1 Unit Testing

```rust
#[test]
fn test_segment_to_way_conversion() {
    let segment = Segment {
        id: "123".to_string(),
        geometry: LineString::new(vec![
            Coordinate { x: -122.4194, y: 37.7749 },
            Coordinate { x: -122.4195, y: 37.7750 },
        ]),
        properties: SegmentProperties {
            class: "motorway".to_string(),
            // Other properties...
        },
    };
    
    let way = OSMWay::from_overture_segment(&segment);
    
    assert_eq!(way.osmwayid, 123);
    assert_eq!(way.road_class, RoadClass::Motorway as u8);
    // Additional assertions...
}
```

### E.2 Integration Testing

```rust
#[test]
fn test_end_to_end_tile_creation() {
    // Create temporary directory for test
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_tile");
    
    // Process test data
    let result = process_overture_data(
        "test_data/sample.parquet",
        output_path.to_str().unwrap(),
    );
    
    assert!(result.is_ok());
    
    // Verify tile was created
    let tile_path = output_path.join("0/0/0.tile");
    assert!(tile_path.exists());
    
    // Load and verify tile contents
    let tile = GraphTile::load(tile_path).unwrap();
    assert!(tile.nodes.len() > 0);
    assert!(tile.directed_edges.len() > 0);
}
```

### E.3 Routing Validation

```rust
#[test]
fn test_routing_results() {
    let router = OvertureRouter::new("test_data/tiles");
    
    // Define test cases with known results
    let test_cases = vec![
        (
            LatLng { lat: 37.7749, lng: -122.4194 },
            LatLng { lat: 37.7833, lng: -122.4167 },
            RouteExpectation {
                max_distance_meters: 2000.0,
                min_distance_meters: 1800.0,
                expected_road_classes: vec![RoadClass::Arterial, RoadClass::Residential],
            },
        ),
        // Additional test cases...
    ];
    
    for (start, end, expectation) in test_cases {
        let route = router.route(start, end).unwrap();
        
        assert!(route.distance >= expectation.min_distance_meters);
        assert!(route.distance <= expectation.max_distance_meters);
        
        // Verify road classes used
        for road_class in expectation.expected_road_classes {
            assert!(route.edges.iter().any(|e| e.road_class == road_class));
        }
    }
}
```
