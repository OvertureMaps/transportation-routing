# Analysis of Kevin Kreiser's Second Comment (Part 1)

## The Comment (First Part)

> ok ive been reading through this and while i feel like ive made a small dent im not sure that it would be beneficial for me to continue.
> 
> if we assume (its what im assuming) the whole point of this exercise is to understand what one would need to do to route on overture then i feel like going deep on the integration point between overture and valhalla is more important than breadthwise exploring how valhalla works. that can wait until we have something sort of working.
> 
> my suggestion would be as follows:
> 
> mjolnir is a pipeline of steps. between each step there is an intermediate representation of the graph data. by the end you have the final graph. the pipeline can be paused or resumed between any two stages of the build. the intermediate data that each step produces allows one to pick up where one left off. the key to making compatible data for valhalla then is to simply find the stage where you can generate that intermediate data. in this case that stage is either the `ConstructEdges` phase or the `GraphBuilder` stage. i personally think the former is both ideal and highly plausible.

## Key Points

1. **Focus Recommendation**: Focus on the integration point between Overture and Valhalla rather than broadly exploring how Valhalla works.

2. **Pipeline Insight**: Mjolnir pipeline has intermediate representations between steps that allow pausing and resuming the build process.

3. **Integration Point**: Kevin suggests targeting either the `ConstructEdges` phase or the `GraphBuilder` stage, with a preference for the former.

## Relevant Valhalla Code

### Mjolnir Pipeline Structure

To understand Kevin's suggestion, we need to examine the Mjolnir pipeline structure and the `ConstructEdges` phase:

```cpp
// From src/valhalla_build_tiles.cc (simplified)
int main(int argc, char** argv) {
  // Parse command line options
  // ...
  
  // Parse OSM PBF data
  OSMData osmdata = PBFGraphParser::Parse(input_files);
  
  // Construct edges from the parsed data
  ConstructEdges(osmdata, ways_file, way_nodes_file, nodes_file, edges_file);
  
  // Build the graph from the constructed edges
  GraphBuilder::Build(config, ways_file, way_nodes_file, nodes_file, edges_file);
  
  // Enhance the graph with additional attributes
  GraphEnhancer::Enhance(config);
  
  // Build the hierarchical graph
  HierarchyBuilder::Build(config);
  
  // ... more stages ...
  
  return 0;
}
```

### The `ConstructEdges` Phase

The `ConstructEdges` phase is a critical step that converts the parsed OSM data into edge structures:

```cpp
// From src/mjolnir/pbfgraphparser.cc (simplified)
void ConstructEdges(OSMData& osmdata, const std::string& ways_file,
                   const std::string& way_nodes_file, const std::string& nodes_file,
                   const std::string& edges_file) {
  // Open files for writing
  std::ofstream ways_out(ways_file, std::ios::binary);
  std::ofstream way_nodes_out(way_nodes_file, std::ios::binary);
  std::ofstream nodes_out(nodes_file, std::ios::binary);
  std::ofstream edges_out(edges_file, std::ios::binary);
  
  // Process nodes
  for (const auto& node : osmdata.nodes) {
    // Write node to file
    nodes_out.write(reinterpret_cast<const char*>(&node), sizeof(OSMNode));
  }
  
  // Process ways
  for (const auto& way : osmdata.ways) {
    // Write way to file
    ways_out.write(reinterpret_cast<const char*>(&way), sizeof(OSMWay));
    
    // Process way nodes
    for (const auto& node : way.nodes) {
      // Write way node to file
      way_nodes_out.write(reinterpret_cast<const char*>(&node), sizeof(OSMWayNode));
    }
  }
  
  // Process edges
  for (const auto& edge : osmdata.edges) {
    // Write edge to file
    edges_out.write(reinterpret_cast<const char*>(&edge), sizeof(OSMEdge));
  }
  
  // Close files
  ways_out.close();
  way_nodes_out.close();
  nodes_out.close();
  edges_out.close();
}
```

### The `GraphBuilder` Stage

The `GraphBuilder` stage takes the output of `ConstructEdges` and builds the actual graph:

```cpp
// From src/mjolnir/graphbuilder.cc (simplified)
void GraphBuilder::Build(const boost::property_tree::ptree& pt,
                       const std::string& ways_file, const std::string& way_nodes_file,
                       const std::string& nodes_file, const std::string& edges_file) {
  // Read the files created by ConstructEdges
  std::vector<OSMWay> ways = ReadWays(ways_file);
  std::vector<OSMWayNode> way_nodes = ReadWayNodes(way_nodes_file);
  std::vector<OSMNode> nodes = ReadNodes(nodes_file);
  std::vector<OSMEdge> edges = ReadEdges(edges_file);
  
  // Build the graph
  BuildGraph(ways, way_nodes, nodes, edges);
  
  // Create tiles
  CreateTiles();
}
```

## Analysis of Integration Points

### `ConstructEdges` Phase

Kevin suggests the `ConstructEdges` phase as the ideal integration point. This makes sense because:

1. **Input Structure**: It takes an `OSMData` structure as input, which contains collections of nodes, ways, and relations.

2. **Output Format**: It produces binary files containing serialized versions of these structures.

3. **Minimal Dependencies**: It has relatively few dependencies on other parts of the Valhalla codebase.

4. **Clear Interface**: It has a well-defined interface that could be adapted for Overture data.

Integrating at this point would involve:
- Creating an `OSMData` structure from Overture data
- Passing this structure to `ConstructEdges` or a modified version of it
- Letting the rest of the pipeline proceed normally

### `GraphBuilder` Stage

The alternative integration point is the `GraphBuilder` stage. This would involve:
- Creating the binary files that `GraphBuilder` expects (ways_file, way_nodes_file, etc.)
- Calling `GraphBuilder::Build` with these files
- Letting the rest of the pipeline proceed normally

This approach would bypass more of the Valhalla pipeline but would require more understanding of the binary file formats.

## Conclusion

Kevin's suggestion to focus on the `ConstructEdges` phase as the integration point is well-founded based on the code examination. This phase provides a clear interface with the `OSMData` structure, which could serve as the target for our Overture data transcoder. By focusing on this specific integration point, we can minimize the amount of Valhalla code that needs to be modified while still leveraging its powerful routing capabilities.
