# Graph Traversal and Connectivity

## The Importance of Graph Connectivity

A routing graph is only useful if it's properly connected. This chapter explores how Mjolnir ensures proper connectivity in the graph, which is crucial for finding valid routes.

## Node and Edge Connectivity

In Valhalla's graph, connectivity is represented through nodes and directed edges:

```cpp
// From baldr/nodeinfo.h
class NodeInfo {
public:
  // Edge information
  uint32_t edge_index() const;
  uint32_t edge_count() const;
  
  // Access to outgoing edges
  const DirectedEdge* GetEdge(uint32_t idx) const;
};

// From baldr/directededge.h
class DirectedEdge {
public:
  // Connectivity information
  bool endnode() const;
  GraphId endnode() const;
  uint32_t opp_index() const;
  bool opp_local() const;
};
```

Each node has a list of edges, and each edge points to an end node:

```
Node A
  |
  +-- Edge 1 --> Node B
  |
  +-- Edge 2 --> Node C
  |
  +-- Edge 3 --> Node D
```

## Building Connectivity

During graph construction, Mjolnir builds connectivity by:

1. **Creating Nodes**: At intersections and endpoints
2. **Creating Edges**: Between connected nodes
3. **Setting References**: Ensuring edges reference the correct nodes
4. **Creating Opposing Edges**: For bidirectional roads

```cpp
// From src/mjolnir/graphbuilder.cc
void GraphBuilder::BuildEdges(const OSMData& osmdata) {
  // For each way
  for (const auto& way : osmdata.ways) {
    // Skip if not usable for routing
    if (!way.IsRoutable()) {
      continue;
    }
    
    // Get the nodes for this way
    std::vector<OSMNode> nodes;
    for (const auto& node_id : way.node_ids) {
      nodes.push_back(osmdata.nodes[node_id]);
    }
    
    // Create edges between nodes
    for (size_t i = 0; i < nodes.size() - 1; i++) {
      // Create a directed edge
      DirectedEdgeBuilder edge;
      
      // Set basic attributes
      edge.set_length(CalculateLength(nodes[i], nodes[i+1]));
      edge.set_use(GetUse(way));
      edge.set_speed(way.speed);
      edge.set_classification(way.road_class);
      
      // Set connectivity
      edge.set_endnode(true);
      edge.set_endnode(GetNodeId(nodes[i+1]));
      
      // Add the edge to the graph
      uint32_t edge_index = AddEdge(edge);
      
      // If bidirectional, add the opposing edge
      if (!way.oneway) {
        DirectedEdgeBuilder opp_edge;
        
        // Set basic attributes (same as forward edge)
        opp_edge.set_length(edge.length());
        opp_edge.set_use(edge.use());
        opp_edge.set_speed(edge.speed());
        opp_edge.set_classification(edge.classification());
        
        // Set connectivity
        opp_edge.set_endnode(true);
        opp_edge.set_endnode(GetNodeId(nodes[i]));
        
        // Set as opposing edge
        opp_edge.set_opp_index(edge_index);
        
        // Add the opposing edge to the graph
        uint32_t opp_edge_index = AddEdge(opp_edge);
        
        // Update the forward edge with the opposing index
        edge.set_opp_index(opp_edge_index);
        UpdateEdge(edge_index, edge);
      }
    }
  }
}
```

## Tile Boundary Connectivity

One of the challenges in a tiled system is maintaining connectivity across tile boundaries. Mjolnir handles this through:

1. **Duplicate Nodes**: Nodes at tile boundaries are duplicated in both tiles
2. **Edge References**: Edges that cross boundaries have references to nodes in adjacent tiles
3. **Transition Edges**: Special edges that connect to adjacent tiles

```cpp
// From src/mjolnir/graphbuilder.cc
void GraphBuilder::AddTransitionEdges() {
  // For each tile
  for (const auto& tile_id : tile_ids_) {
    // Get the tile
    const GraphTile* tile = reader_.GetGraphTile(tile_id);
    
    // Create a tile builder
    GraphTileBuilder builder(tile_id, tile_dir_);
    
    // For each node in the tile
    for (uint32_t node_idx = 0; node_idx < tile->header()->nodecount(); node_idx++) {
      const NodeInfo* node = tile->node(node_idx);
      
      // If the node is at a tile boundary
      if (IsTileBoundaryNode(node)) {
        // Find neighboring tiles
        std::vector<GraphId> neighbors = GetNeighboringTiles(tile_id, node);
        
        // For each neighboring tile
        for (const auto& neighbor_id : neighbors) {
          // Find the corresponding node in the neighboring tile
          GraphId neighbor_node_id = FindNodeInTile(neighbor_id, node->latlng());
          
          // If found, add transition edges
          if (neighbor_node_id.Is_Valid()) {
            // Add transition edge from this tile to the neighbor
            DirectedEdgeBuilder trans_edge;
            trans_edge.set_trans_up(false);
            trans_edge.set_trans_down(false);
            trans_edge.set_endnode(true);
            trans_edge.set_endnode(neighbor_node_id);
            
            builder.AddEdge(trans_edge);
          }
        }
      }
    }
    
    // Store the updated tile
    builder.StoreTileData();
  }
}
```

## Hierarchical Connectivity

Valhalla's hierarchical graph requires connectivity between different levels:

1. **Upward Transitions**: Edges that connect from a lower level to a higher level
2. **Downward Transitions**: Edges that connect from a higher level to a lower level

```cpp
// From src/mjolnir/hierarchybuilder.cc
void HierarchyBuilder::AddHierarchicalTransitions() {
  // For each level except the highest
  for (uint8_t level = 0; level < 2; level++) {
    // For each tile in this level
    for (const auto& tile_id : GetTileSet(level)) {
      // Get the tile
      const GraphTile* tile = reader_.GetGraphTile(tile_id);
      
      // Create a tile builder
      GraphTileBuilder builder(tile_id, tile_dir_);
      
      // For each node in the tile
      for (uint32_t node_idx = 0; node_idx < tile->header()->nodecount(); node_idx++) {
        const NodeInfo* node = tile->node(node_idx);
        
        // Check if this node exists in the next level up
        GraphId higher_node_id = FindNodeInHigherLevel(node, level + 1);
        if (higher_node_id.Is_Valid()) {
          // Add upward transition
          DirectedEdgeBuilder up_edge;
          up_edge.set_trans_up(true);
          up_edge.set_endnode(true);
          up_edge.set_endnode(higher_node_id);
          
          builder.AddEdge(up_edge);
          
          // Add downward transition in the higher level
          AddDownwardTransition(higher_node_id, GraphId(tile_id.tileid(), level, node_idx));
        }
      }
      
      // Store the updated tile
      builder.StoreTileData();
    }
  }
}
```

## Connectivity Validation

After building the graph, Mjolnir validates connectivity to ensure there are no disconnected components:

```cpp
// From src/mjolnir/graphvalidator.cc
void GraphValidator::ValidateConnectivity() {
  // For each tile
  for (const auto& tile_id : GetTileSet()) {
    // Get the tile
    const GraphTile* tile = reader_.GetGraphTile(tile_id);
    
    // For each node in the tile
    for (uint32_t node_idx = 0; node_idx < tile->header()->nodecount(); node_idx++) {
      const NodeInfo* node = tile->node(node_idx);
      
      // Skip transit nodes and other special nodes
      if (node->type() == NodeType::kTransit) {
        continue;
      }
      
      // Check if the node has any edges
      if (node->edge_count() == 0) {
        LOG_WARN("Isolated node found: " + std::to_string(node_idx));
        continue;
      }
      
      // Check if the node is reachable from other nodes
      if (!IsNodeReachable(tile_id, node_idx)) {
        LOG_WARN("Unreachable node found: " + std::to_string(node_idx));
      }
    }
  }
}
```

## Graph Traversal

Valhalla's routing algorithms traverse the graph using the connectivity information:

```cpp
// From thor/astar.cc
std::vector<PathInfo> AStar::GetBestPath(const Location& origin,
                                      const Location& destination,
                                      GraphReader& reader,
                                      const sif::mode_costing_t& mode_costing,
                                      const sif::TravelMode mode) {
  // Initialize the origin and destination locations
  Init(origin, destination);
  
  // Get the cost object
  const auto& costing = mode_costing[static_cast<uint32_t>(mode)];
  
  // Find the path
  std::vector<PathInfo> path;
  if (SearchForPath(reader, origin, destination, costing, path)) {
    return path;
  }
  
  // Path not found
  return {};
}

bool AStar::SearchForPath(GraphReader& reader,
                        const Location& origin,
                        const Location& destination,
                        const sif::cost_ptr_t& costing,
                        std::vector<PathInfo>& path) {
  // Initialize the priority queue
  std::priority_queue<AStarElement> pqueue;
  
  // Add the origin to the queue
  pqueue.push({origin.edges[0].id, 0.0f, 0.0f, 0.0f, {}, 0});
  
  // Expand until we find the destination or exhaust the queue
  while (!pqueue.empty()) {
    // Get the next element
    const auto element = pqueue.top();
    pqueue.pop();
    
    // If this is the destination, we're done
    if (element.id == destination.edges[0].id) {
      // Build the path
      BuildPath(element, path);
      return true;
    }
    
    // Get the tile containing this edge
    const GraphTile* tile = reader.GetGraphTile(element.id);
    
    // Get the directed edge
    const DirectedEdge* edge = tile->directededge(element.id.id());
    
    // Get the end node
    const NodeInfo* node = tile->node(edge->endnode());
    
    // Expand all outgoing edges
    for (uint32_t i = 0; i < node->edge_count(); i++) {
      // Get the outgoing edge
      const DirectedEdge* next_edge = tile->directededge(node->edge_index() + i);
      
      // Skip if this edge is not allowed for this mode
      if (!costing->Allowed(next_edge)) {
        continue;
      }
      
      // Calculate the cost to traverse this edge
      float edge_cost = costing->EdgeCost(next_edge);
      
      // Calculate the transition cost between edges
      float trans_cost = costing->TransitionCost(edge, node, next_edge);
      
      // Calculate the total cost
      float cost = element.cost + edge_cost + trans_cost;
      
      // Calculate the heuristic cost (A* estimate to destination)
      float h = Heuristic(next_edge->endnode(), destination);
      
      // Add to the priority queue
      pqueue.push({next_edge->endnode(), cost, h, edge_cost, element, i});
    }
  }
  
  // Path not found
  return false;
}
```

## Connectivity Diagram

Here's a visual representation of connectivity in Valhalla's graph:

```
Level 2 (Highways)
+-----+                +-----+
| A2  |--------------->| B2  |
+-----+                +-----+
   |                      |
   | Transition           | Transition
   | Down                 | Down
   v                      v
Level 1 (Regional Roads)
+-----+                +-----+
| A1  |--------------->| B1  |
+-----+                +-----+
   |                      |
   | Transition           | Transition
   | Down                 | Down
   v                      v
Level 0 (Local Roads)
+-----+                +-----+
| A0  |--------------->| B0  |
+-----+                +-----+
   |                      |
   |                      |
+-----+                +-----+
| C0  |--------------->| D0  |
+-----+                +-----+
```

## Tile Boundary Connectivity Diagram

Here's a visual representation of connectivity across tile boundaries:

```
Tile 1                 Tile 2
+-------------------+  +-------------------+
|                   |  |                   |
|    +-----+        |  |        +-----+    |
|    | A   |        |  |        | B   |    |
|    +-----+        |  |        +-----+    |
|       |           |  |           |       |
|       |           |  |           |       |
|       |           |  |           |       |
|    +-----+        |  |        +-----+    |
|    | C   |        |  |        | D   |    |
|    +-----+        |  |        +-----+    |
|       |           |  |           |       |
|       |           |  |           |       |
|       v           |  |           v       |
|    +-----+        |  |        +-----+    |
|    | E   |--------|--+------->| F   |    |
|    +-----+        |  |        +-----+    |
|                   |  |                   |
+-------------------+  +-------------------+
```

## Ensuring proper connectivity is crucial for building a functional routing graph. Without it, routes may be impossible to find or may take unnecessarily long detours. Mjolnir's careful handling of connectivity ensures that Valhalla can find optimal routes across the entire graph.
