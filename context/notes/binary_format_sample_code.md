# Valhalla Binary Format Sample Code

## Reading Binary Files (C++)

### Basic File Reading
```cpp
#include <fstream>
#include <vector>
#include <valhalla/mjolnir/osmway.h>
#include <valhalla/mjolnir/osmnode.h>
#include <valhalla/mjolnir/osmdata.h>

// Read ways.bin
std::vector<valhalla::mjolnir::OSMWay> read_ways(const std::string& filename) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) {
        throw std::runtime_error("Cannot open " + filename);
    }
    
    // Get file size
    file.seekg(0, std::ios::end);
    size_t file_size = file.tellg();
    file.seekg(0, std::ios::beg);
    
    // Calculate number of records
    size_t record_count = file_size / sizeof(valhalla::mjolnir::OSMWay);
    if (file_size % sizeof(valhalla::mjolnir::OSMWay) != 0) {
        throw std::runtime_error("Invalid file size for OSMWay records");
    }
    
    // Read all records
    std::vector<valhalla::mjolnir::OSMWay> ways(record_count);
    file.read(reinterpret_cast<char*>(ways.data()), file_size);
    
    return ways;
}

// Read way_nodes.bin
std::vector<valhalla::mjolnir::OSMWayNode> read_way_nodes(const std::string& filename) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) {
        throw std::runtime_error("Cannot open " + filename);
    }
    
    file.seekg(0, std::ios::end);
    size_t file_size = file.tellg();
    file.seekg(0, std::ios::beg);
    
    size_t record_count = file_size / sizeof(valhalla::mjolnir::OSMWayNode);
    if (file_size % sizeof(valhalla::mjolnir::OSMWayNode) != 0) {
        throw std::runtime_error("Invalid file size for OSMWayNode records");
    }
    
    std::vector<valhalla::mjolnir::OSMWayNode> way_nodes(record_count);
    file.read(reinterpret_cast<char*>(way_nodes.data()), file_size);
    
    return way_nodes;
}
```

### Using Valhalla's Sequence Template
```cpp
#include <valhalla/midgard/sequence.h>

void read_with_sequence() {
    // Open files using Valhalla's sequence template
    valhalla::midgard::sequence<valhalla::mjolnir::OSMWay> ways("ways.bin", false);
    valhalla::midgard::sequence<valhalla::mjolnir::OSMWayNode> way_nodes("way_nodes.bin", false);
    
    // Access records by index
    for (size_t i = 0; i < ways.size(); ++i) {
        auto way = *ways[i];  // Dereference to get OSMWay
        std::cout << "Way " << way.way_id() << " has " << way.node_count() << " nodes\n";
    }
    
    // Iterate through way nodes
    for (size_t i = 0; i < way_nodes.size(); ++i) {
        auto way_node = *way_nodes[i];
        std::cout << "Node " << way_node.node.osmid_ 
                  << " belongs to way index " << way_node.way_index << "\n";
    }
}
```

## Writing Binary Files (C++)

### Basic File Writing
```cpp
void write_ways(const std::vector<valhalla::mjolnir::OSMWay>& ways, 
                const std::string& filename) {
    std::ofstream file(filename, std::ios::binary | std::ios::trunc);
    if (!file) {
        throw std::runtime_error("Cannot create " + filename);
    }
    
    // Write all records at once
    file.write(reinterpret_cast<const char*>(ways.data()), 
               ways.size() * sizeof(valhalla::mjolnir::OSMWay));
}

void write_way_nodes(const std::vector<valhalla::mjolnir::OSMWayNode>& way_nodes,
                     const std::string& filename) {
    std::ofstream file(filename, std::ios::binary | std::ios::trunc);
    if (!file) {
        throw std::runtime_error("Cannot create " + filename);
    }
    
    file.write(reinterpret_cast<const char*>(way_nodes.data()),
               way_nodes.size() * sizeof(valhalla::mjolnir::OSMWayNode));
}
```

### Using Valhalla's Sequence Template for Writing
```cpp
void write_with_sequence() {
    // Create files for writing
    valhalla::midgard::sequence<valhalla::mjolnir::OSMWay> ways("ways.bin", true);
    valhalla::midgard::sequence<valhalla::mjolnir::OSMWayNode> way_nodes("way_nodes.bin", true);
    
    // Create sample way
    valhalla::mjolnir::OSMWay way(12345);  // Way ID
    way.set_node_count(3);
    way.set_speed(50.0f);  // 50 KPH
    ways.push_back(way);
    
    // Create sample way nodes
    for (int i = 0; i < 3; ++i) {
        valhalla::mjolnir::OSMWayNode way_node;
        way_node.node.set_id(1000 + i);
        way_node.node.set_latlng(-122.0 + i * 0.001, 47.0 + i * 0.001);
        way_node.way_index = 0;  // References first way
        way_node.way_shape_node_index = i;
        way_nodes.push_back(way_node);
    }
    
    // Files are automatically flushed when sequence objects are destroyed
}
```

## Coordinate Encoding/Decoding

### Encoding Coordinates
```cpp
struct CoordinateEncoder {
    static uint32_t encode_longitude(double lng) {
        if (lng < -180.0 || lng > 180.0) {
            return std::numeric_limits<uint32_t>::max();  // Invalid marker
        }
        double encoded = std::round((lng + 180.0) * 1e7);
        return (encoded >= 0 && encoded <= 360 * 1e7) ? 
               static_cast<uint32_t>(encoded) : 
               std::numeric_limits<uint32_t>::max();
    }
    
    static uint32_t encode_latitude(double lat) {
        if (lat < -90.0 || lat > 90.0) {
            return std::numeric_limits<uint32_t>::max();  // Invalid marker
        }
        double encoded = std::round((lat + 90.0) * 1e7);
        return (encoded >= 0 && encoded <= 180 * 1e7) ? 
               static_cast<uint32_t>(encoded) : 
               std::numeric_limits<uint32_t>::max();
    }
};
```

### Decoding Coordinates
```cpp
struct CoordinateDecoder {
    static double decode_longitude(uint32_t encoded) {
        if (encoded == std::numeric_limits<uint32_t>::max()) {
            return std::numeric_limits<double>::quiet_NaN();  // Invalid
        }
        return (static_cast<double>(encoded) / 1e7) - 180.0;
    }
    
    static double decode_latitude(uint32_t encoded) {
        if (encoded == std::numeric_limits<uint32_t>::max()) {
            return std::numeric_limits<double>::quiet_NaN();  // Invalid
        }
        return (static_cast<double>(encoded) / 1e7) - 90.0;
    }
    
    static bool is_valid(uint32_t encoded) {
        return encoded != std::numeric_limits<uint32_t>::max();
    }
};
```

## Rust Implementation Examples

### Basic Structures
```rust
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct OSMWay {
    osmwayid: u64,
    nodecount: u32,
    speed: u8,
    speed_limit: u8,
    backward_speed: u8,
    forward_speed: u8,
    // Note: This is simplified - actual struct has many more fields
    // See osmway.h for complete structure
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct OSMNode {
    osmid: u64,
    // Bit-packed fields would need careful handling in Rust
    // Consider using bitflags crate or manual bit manipulation
    packed_field1: u64,  // Contains name_index, ref_index, etc.
    packed_field2: u64,  // Contains country_iso_index, signals, etc.
    packed_field3: u32,  // Contains access, type, intersection flags, etc.
    bss_info: u32,
    linguistic_info_index: u32,
    lng7: u32,
    lat7: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct OSMWayNode {
    node: OSMNode,
    way_index: u32,
    way_shape_node_index: u32,
}
```

### Reading Binary Files in Rust
```rust
use std::mem;

fn read_ways(filename: &str) -> Result<Vec<OSMWay>, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let file_size = file.metadata()?.len() as usize;
    
    let record_size = mem::size_of::<OSMWay>();
    if file_size % record_size != 0 {
        return Err("Invalid file size for OSMWay records".into());
    }
    
    let record_count = file_size / record_size;
    let mut ways = Vec::with_capacity(record_count);
    
    // Read raw bytes
    let mut buffer = vec![0u8; file_size];
    file.read_exact(&mut buffer)?;
    
    // Convert bytes to structs
    for chunk in buffer.chunks_exact(record_size) {
        let way: OSMWay = unsafe {
            std::ptr::read(chunk.as_ptr() as *const OSMWay)
        };
        ways.push(way);
    }
    
    Ok(ways)
}

fn read_way_nodes(filename: &str) -> Result<Vec<OSMWayNode>, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let file_size = file.metadata()?.len() as usize;
    
    let record_size = mem::size_of::<OSMWayNode>();
    if file_size % record_size != 0 {
        return Err("Invalid file size for OSMWayNode records".into());
    }
    
    let record_count = file_size / record_size;
    let mut way_nodes = Vec::with_capacity(record_count);
    
    let mut buffer = vec![0u8; file_size];
    file.read_exact(&mut buffer)?;
    
    for chunk in buffer.chunks_exact(record_size) {
        let way_node: OSMWayNode = unsafe {
            std::ptr::read(chunk.as_ptr() as *const OSMWayNode)
        };
        way_nodes.push(way_node);
    }
    
    Ok(way_nodes)
}
```

### Writing Binary Files in Rust
```rust
fn write_ways(ways: &[OSMWay], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    
    for way in ways {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                way as *const OSMWay as *const u8,
                mem::size_of::<OSMWay>()
            )
        };
        file.write_all(bytes)?;
    }
    
    Ok(())
}

fn write_way_nodes(way_nodes: &[OSMWayNode], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    
    for way_node in way_nodes {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                way_node as *const OSMWayNode as *const u8,
                mem::size_of::<OSMWayNode>()
            )
        };
        file.write_all(bytes)?;
    }
    
    Ok(())
}
```

### Coordinate Encoding in Rust
```rust
impl OSMNode {
    fn set_latlng(&mut self, lng: f64, lat: f64) {
        // Encode longitude
        if lng >= -180.0 && lng <= 180.0 {
            let encoded = ((lng + 180.0) * 1e7).round() as u64;
            self.lng7 = if encoded <= 360 * 10_000_000 {
                encoded as u32
            } else {
                u32::MAX
            };
        } else {
            self.lng7 = u32::MAX;
        }
        
        // Encode latitude
        if lat >= -90.0 && lat <= 90.0 {
            let encoded = ((lat + 90.0) * 1e7).round() as u64;
            self.lat7 = if encoded <= 180 * 10_000_000 {
                encoded as u32
            } else {
                u32::MAX
            };
        } else {
            self.lat7 = u32::MAX;
        }
    }
    
    fn get_latlng(&self) -> (f64, f64) {
        let lng = if self.lng7 == u32::MAX {
            f64::NAN
        } else {
            (self.lng7 as f64 / 1e7) - 180.0
        };
        
        let lat = if self.lat7 == u32::MAX {
            f64::NAN
        } else {
            (self.lat7 as f64 / 1e7) - 90.0
        };
        
        (lng, lat)
    }
}
```

## Validation Functions

### C++ Validation
```cpp
bool validate_ways_file(const std::string& filename) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) return false;
    
    file.seekg(0, std::ios::end);
    size_t file_size = file.tellg();
    
    // Check if file size is multiple of struct size
    return (file_size % sizeof(valhalla::mjolnir::OSMWay)) == 0;
}

bool validate_way_nodes_consistency(const std::string& ways_file,
                                   const std::string& way_nodes_file) {
    auto ways = read_ways(ways_file);
    auto way_nodes = read_way_nodes(way_nodes_file);
    
    // Group way nodes by way_index
    std::map<uint32_t, std::vector<valhalla::mjolnir::OSMWayNode>> grouped;
    for (const auto& wn : way_nodes) {
        grouped[wn.way_index].push_back(wn);
    }
    
    // Validate each way
    for (size_t i = 0; i < ways.size(); ++i) {
        const auto& way = ways[i];
        auto it = grouped.find(i);
        
        if (it == grouped.end()) {
            std::cerr << "Way " << i << " has no nodes\n";
            return false;
        }
        
        if (it->second.size() != way.node_count()) {
            std::cerr << "Way " << i << " node count mismatch\n";
            return false;
        }
        
        // Check shape node indices are sequential
        std::sort(it->second.begin(), it->second.end(),
                 [](const auto& a, const auto& b) {
                     return a.way_shape_node_index < b.way_shape_node_index;
                 });
        
        for (size_t j = 0; j < it->second.size(); ++j) {
            if (it->second[j].way_shape_node_index != j) {
                std::cerr << "Way " << i << " has non-sequential shape indices\n";
                return false;
            }
        }
    }
    
    return true;
}
```

## Testing and Debugging

### Hex Dump Utility
```cpp
void hex_dump_file(const std::string& filename, size_t max_bytes = 256) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) {
        std::cerr << "Cannot open " << filename << "\n";
        return;
    }
    
    std::vector<uint8_t> buffer(max_bytes);
    file.read(reinterpret_cast<char*>(buffer.data()), max_bytes);
    size_t bytes_read = file.gcount();
    
    for (size_t i = 0; i < bytes_read; i += 16) {
        std::cout << std::hex << std::setw(8) << std::setfill('0') << i << ": ";
        
        for (size_t j = 0; j < 16 && i + j < bytes_read; ++j) {
            std::cout << std::hex << std::setw(2) << std::setfill('0') 
                      << static_cast<int>(buffer[i + j]) << " ";
        }
        std::cout << "\n";
    }
}
```

This sample code provides the foundation for implementing binary file readers and writers compatible with Valhalla's format. The key points are:

1. **Direct struct serialization** - no headers or metadata
2. **Native endianness** - platform-specific
3. **Careful coordinate encoding** - 7-digit precision with offset
4. **Index consistency** - way_index must reference valid ways
5. **Validation** - ensure file sizes and relationships are correct
