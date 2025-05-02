# Valhalla's Tiled Structure

## The Concept of Graph Tiles

At the heart of Valhalla's routing engine is its tiled graph structure. Unlike some other routing engines that load the entire graph into memory, Valhalla divides the world into a hierarchical system of tiles. This approach offers several advantages:

1. **Memory Efficiency**: Only the tiles needed for a specific route calculation need to be loaded into memory
2. **Scalability**: The system can handle planet-scale data while maintaining reasonable memory usage
3. **Partial Updates**: Individual tiles can be updated without rebuilding the entire graph
4. **Regional Extracts**: Work with data for specific regions only

## Tile Hierarchy and Levels

Valhalla organizes its routing graph into multiple hierarchical levels, each representing different types of roads:

- **Level 0**: Local roads and paths (highest detail)
- **Level 1**: Regional roads
- **Level 2**: Major highways and arterials (lowest detail)

This hierarchy mimics how humans navigate: using local streets near origin and destination, but preferring major roads for the middle portion of longer journeys.

The tiles are split up into these three levels or hierarchies as follows:
- Level 0 contains edges for highway roads (motorway, trunk, and primary) stored in 4-degree tiles
- Level 1 contains arterial roads (secondary and tertiary) saved in 1-degree tiles
- Level 2 contains local roads (unclassified, residential, service, etc.) saved in 0.25-degree tiles

This hierarchical structure allows the routing algorithm to quickly traverse long distances by using higher-level tiles, then switch to more detailed tiles as it approaches the origin and destination.

## Tile Coordinate System

Valhalla uses a custom tile coordinate system based on the following components:

```cpp
// From baldr/graphid.h
struct GraphId {
  uint64_t value;
  
  // Methods to access specific parts of the ID
  uint32_t tileid() const;
  uint32_t level() const;
  uint32_t id() const;
  
  // ...
};
```

Each `GraphId` contains:
- **Level**: The hierarchy level (0, 1, or 2)
- **Tile ID**: Identifies a specific tile within the level
- **ID**: Identifies a specific element (node or edge) within the tile

The tile ID is derived from the geographic coordinates using a space-filling curve algorithm, which ensures that nearby locations are likely to be in the same tile or adjacent tiles.

## Tile Size and Resolution

The size of tiles varies by level:

- Level 0 tiles are the largest, covering approximately 4 degrees × 4 degrees
- Level 1 tiles cover 1 degree × 1 degree
- Level 2 tiles are the smallest, covering 0.25 degrees × 0.25 degrees

This variable resolution allows for efficient storage and retrieval of road network data at different scales. The world is divided into a grid based on these tile sizes, with rows and columns starting from the bottom left (-180, -90) and increasing to the top right (180, 90).

## Code Example: Tile Coordinates Calculation

Here's how Valhalla converts geographic coordinates to tile coordinates:

```cpp
// From midgard/tiles.h
class Tiles {
public:
  /**
   * Get the tile ID given the latitude and longitude.
   * @param lat Latitude value.
   * @param lng Longitude value.
   * @return Returns the tile ID.
   */
  int32_t TileId(const float lat, const float lng) const;
  
  // ...
};
```

The implementation uses a projection system to map geographic coordinates to tile IDs:

```cpp
// From midgard/tiles.cc
int32_t Tiles::TileId(const float lat, const float lng) const {
  // Get the indexes within the tiles and return the tile ID
  const int32_t x = static_cast<int32_t>(floor((lng - tilebounds_.minx()) * inv_tile_size_));
  const int32_t y = static_cast<int32_t>(floor((lat - tilebounds_.miny()) * inv_tile_size_));
  return (y * ncolumns_) + x;
}
```

## Tile Storage Format

Valhalla stores tiles as binary files on disk. Each tile contains:

1. **Header**: Contains metadata about the tile
2. **Nodes**: Represent intersections in the road network
3. **Directed Edges**: Represent road segments with direction-specific attributes
4. **Edge Info**: Contains shared data for edges (e.g., shape, names)
5. **Additional Data**: Restrictions, traffic signs, admin information, etc.

The binary format is designed for efficient storage and quick loading. Here's the structure of a tile header:

```cpp
// From baldr/graphtileheader.h
class GraphTileHeader {
public:
  // ...
  
  uint32_t graphid_;                 // Tile ID within the hierarchy
  uint32_t date_created_;            // Date created
  uint32_t node_count_;              // Number of nodes
  uint32_t directed_edge_count_;     // Number of directed edges
  // ... many more fields ...
};
```

The tiles are stored on disk with a directory structure that reflects their hierarchy level and ID. For example, a tile with ID 756425 at level 2 would be stored at `/2/000/756/425.gph`.

## Tile Connectivity

Tiles connect to adjacent tiles through "transition" edges and nodes. These special connections allow routes to seamlessly cross tile boundaries. Additionally, connections between hierarchy levels are maintained through "transition up/down" edges, which allow routes to move between local, regional, and highway networks.

## Memory Management

Valhalla uses a tile cache to manage memory usage:

```cpp
// From baldr/graphreader.h
class GraphReader {
public:
  // ...
  
private:
  std::shared_ptr<GraphMemory> memory_;
  CacheLRU<GraphId, GraphTilePtr> cache_;
  // ...
};
```

This cache uses a least-recently-used (LRU) strategy to keep frequently accessed tiles in memory while allowing less-used tiles to be evicted when memory pressure increases. This approach is particularly important for mobile or embedded applications where memory resources may be limited.

Understanding this tiled structure is fundamental to building a graph tile builder, as it defines how the final output should be organized and stored.
