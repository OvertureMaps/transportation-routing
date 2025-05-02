# RFC: Overture Graph Tiles for Routing Engines

- **RFC Number**: 001
- **Author**: Swagata Prateek (pswagata@amazon.com)
- **Status**: Draft
- **Created**: 2025-05-02
- **Last Updated**: 2025-05-02

## Introduction

The Overture Maps Foundation provides a rich, standardized transportation schema that represents road networks and other transportation infrastructure. However, to use this data effectively for routing applications, it needs to be integrated with existing routing engines or processed by custom routing algorithms. This RFC proposes the development of Overture Graph Tiles, a specialized graph representation optimized for Overture transportation data.

Today, most routing engines are designed to work with OpenStreetMap (OSM) data, which has a different schema and structure compared to Overture data. While OSM uses a flat tagging system with ways and nodes, Overture provides a more structured schema with segments and connectors, normalized attributes, and first-class topological nodes. These differences create both challenges and opportunities for routing applications.

This document outlines a phased approach to developing Overture Graph Tiles, starting with integration into existing routing engines (specifically Valhalla) and culminating in a standalone, optimized tile format. This approach allows for immediate utility while working toward a more optimized long-term solution.

## Background and Motivation

### The Challenge of Routing with Overture Data

Overture Maps Foundation data offers several advantages over traditional mapping data sources:

1. **Normalized Schema**: Attributes follow a consistent, well-defined schema rather than an ad-hoc tagging system
2. **First-Class Topology**: Connectors explicitly define the topology of the road network
3. **Rich Attributes**: Comprehensive information about road characteristics, restrictions, and properties
4. **Standardized Format**: Data is provided in standard formats like GeoParquet

However, these advantages also create integration challenges with existing routing engines, which are typically designed for OSM data. The differences in data models require a conversion process that can be complex and potentially lossy.

### Current Approaches and Limitations

Current approaches to using Overture data for routing include:

1. **Direct Conversion to OSM**: Converting Overture data to OSM format, losing some of the advantages of Overture's schema
2. **Custom Routing Engines**: Building entirely new routing engines specifically for Overture data, which is resource-intensive
3. **Ad-hoc Integration**: Creating one-off integrations with specific routing engines, which aren't reusable

These approaches have significant limitations in terms of data fidelity, development effort, or reusability.

### The Need for Overture Graph Tiles

Overture Graph Tiles address these limitations by providing:

1. **Optimized Representation**: A graph representation specifically designed for Overture's data model
2. **Standardized Format**: A common format that can be used by multiple routing engines
3. **Phased Approach**: A path from immediate integration to long-term optimization
4. **Performance Focus**: Structures and algorithms optimized for routing performance

By developing Overture Graph Tiles, we aim to make Overture data more accessible and useful for routing applications, fostering adoption and innovation in the mapping ecosystem.

## Proposal: A Phased Approach to Overture Graph Tiles

We propose a three-phase approach to developing Overture Graph Tiles:

### Phase 1: Valhalla Integration via Binary Files

In this initial phase, we will create a transcoder that converts Overture transportation data to Valhalla's binary file formats, enabling routing with minimal modifications to existing tools.

### Phase 2: Direct Integration and Research

In the second phase, we will explore more efficient integration methods and begin research on an Overture-specific tile format.

### Phase 3: Overture Graph Tiles Implementation

In the final phase, we will implement a full Overture Graph Tiles format optimized for Overture's data model and routing applications.

This phased approach allows us to deliver value quickly while working toward a more optimized long-term solution.

## Detailed Design

### Understanding Valhalla's Architecture

Before diving into our integration approach, it's important to understand how Valhalla processes data and creates its routing graph.

Valhalla is a modern, open-source routing engine designed for flexibility and performance. Its architecture consists of several key components:

1. **Data Processing Pipeline**: Converts raw map data into a routable graph
2. **Tiled Graph Structure**: Organizes the graph into hierarchical tiles for efficient routing
3. **Routing Engine**: Performs pathfinding on the tiled graph
4. **API Layer**: Provides interfaces for routing requests

The data processing pipeline is particularly relevant for our integration. It consists of these main stages:

1. **Parse OSM**: Reads OSM PBF files and extracts ways, nodes, and relations
2. **Create OSMData**: Builds in-memory structures representing the transportation network
3. **Construct Edges**: Converts OSM entities into graph edges
4. **Build Graph**: Creates the tiled graph structure
5. **Enhance Graph**: Adds additional information like administrative boundaries
6. **Create Hierarchy**: Builds a hierarchical graph for efficient routing

Our integration will focus on bypassing the OSM-specific parts of this pipeline while leveraging the rest of Valhalla's capabilities.

### Phase 1: Valhalla Integration via Binary Files (3-4 months)

#### Architecture

In Phase 1, we'll create a transcoder that converts Overture data to Valhalla's binary file formats, which can then be processed by Valhalla's existing pipeline.

```
┌─────────────┐     ┌───────────────┐     ┌───────────────┐     ┌───────────────┐
│  Overture   │     │  Rust-based   │     │  Valhalla     │     │  Routing      │
│  Data       │────▶│  Transcoder   │────▶│  Tile Builder │────▶│  Engine       │
│ (GeoParquet)│     │               │     │               │     │               │
└─────────────┘     └───────────────┘     └───────────────┘     └───────────────┘
```

The key insight from our research is that we don't need to modify Valhalla's entire pipeline. Instead, we can:

1. Identify the right integration point in Valhalla's pipeline
2. Create a transcoder that converts Overture data to Valhalla's expected format at that point
3. Let Valhalla handle the rest of the process

Based on our analysis of Valhalla's codebase, the `ConstructEdges` phase is the ideal integration point. This means:

1. We bypass Valhalla's OSM parsing phase
2. We create our own parser for Overture data
3. We transform Overture data into the binary files that `ConstructEdges` expects
4. We let Valhalla handle the rest of the pipeline

#### Valhalla Pipeline and Integration Point

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Valhalla Pipeline                                 │
├─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬────────────────┤
│ Parse   │ Create  │ Construct│ Build   │ Enhance │ Create  │ Final          │
│ OSM     │ OSMData │ Edges   │ Graph   │ Graph   │ Hierarchy│ Tiles         │
└─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴────────────────┘
                      ▲
                      │
┌─────────┬───────────┴───────┐
│ Overture│ Overture          │
│ Data    │ Transcoder        │
└─────────┴───────────────────┘
```

#### Binary Files to Generate

Our transcoder needs to generate these binary files:

1. **ways.bin**: Contains serialized `OSMWay` structures representing road segments
2. **way_nodes.bin**: Contains serialized `OSMWayNode` structures linking ways to nodes
3. **nodes.bin**: Contains serialized `OSMNode` structures representing intersections

These files are then used by `GraphBuilder::BuildEdges` to create the `edges.bin` file, which is used by the rest of the Valhalla pipeline.

#### Entity Mapping

The core of our transcoder will be the mapping between Overture entities and Valhalla structures:

| Overture Entity | Valhalla Structure | Description |
|-----------------|-------------------|-------------|
| Segment | OSMWay | Road segments that can be traveled |
| Connector | OSMNode | Junction points between segments |
| Segment-Connector Relationship | OSMWayNode | Links segments to their connectors |

This mapping leverages the natural correspondence between Overture's segments and connectors and Valhalla's ways and nodes.

#### Detailed Mapping: Segment → OSMWay

Overture segments contain rich information about road characteristics that needs to be mapped to Valhalla's OSMWay structure:

```
┌─────────────────────────────────────────────────────────┐
│ Overture Segment                                        │
├─────────────────────────────────────────────────────────┤
│ id: "123"                                               │
│ geometry: LineString(...)                               │
│ properties:                                             │
│   subtype: "road"                                       │
│   class: "motorway"                                     │
│   access_restrictions: { vehicle: "yes", ... }          │
│   speed_limits: { default: 100, ... }                   │
│   connectors: [                                         │
│     { connector_id: "456", at: 0.0 },                   │
│     { connector_id: "789", at: 1.0 }                    │
│   ]                                                     │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│ Valhalla OSMWay                                         │
├─────────────────────────────────────────────────────────┤
│ osmwayid_: 123                                          │
│ node_ids_: [456, 789]                                   │
│ road_class_: kMotorway                                  │
│ speed_: 100                                             │
│ forward_access_: kAllAccess                             │
│ backward_access_: kAllAccess                            │
│ ...                                                     │
└─────────────────────────────────────────────────────────┘
```

#### Road Classification Mapping

Overture's road classification system needs to be mapped to Valhalla's road classes:

| Overture Class | Valhalla Road Class |
|----------------|---------------------|
| motorway | kMotorway |
| trunk | kTrunk |
| primary | kPrimary |
| secondary | kSecondary |
| tertiary | kTertiary |
| residential | kResidential |
| unclassified | kUnclassified |
| service | kService |
| pedestrian | kPedestrian |
| footway | kFootway |
| ... | ... |

#### Access Restriction Mapping

Overture's access restrictions need to be mapped to Valhalla's access flags:

| Overture Access | Valhalla Access |
|-----------------|----------------|
| vehicle: "yes" | kAllAccess |
| vehicle: "no" | kNoAccess |
| motor_vehicle: "yes" | kAutoAccess \| kTruckAccess \| kMotorcycleAccess |
| foot: "yes" | kPedestrianAccess |
| bicycle: "yes" | kBicycleAccess |
| ... | ... |

#### Implementation Steps

1. **Binary Format Analysis**: Understand Valhalla's binary file formats through code analysis and testing
2. **Rust Structure Implementation**: Create Rust structs matching Valhalla's formats
3. **Basic Entity Mapping**: Implement core entity mapping logic
4. **Attribute Mapping**: Implement attribute mapping for essential routing attributes
5. **Testing and Validation**: Validate output with Valhalla's tools
6. **Performance Optimization**: Optimize for large datasets

#### Administrative Boundaries

Administrative boundaries require special handling. We'll:

1. Use DuckDB to process Overture's admin boundary data
2. Convert it to a spatialite database format that Valhalla can use
3. Integrate this with the main pipeline

This approach allows us to leverage Valhalla's existing capabilities for administrative lookups.

#### Phase 1 Milestone Achievement

By the end of Phase 1, we will have:

1. **Functional Transcoder**: A working Rust-based transcoder that converts Overture transportation data to Valhalla's binary formats
2. **Complete Attribute Mapping**: A comprehensive mapping between Overture and Valhalla attributes for basic routing
3. **Command-line Tool**: A user-friendly command-line interface for the transcoder
4. **Integration Script**: A script that automates the process of converting Overture data and running Valhalla's build pipeline
5. **Documentation**: Detailed documentation on the transcoder, attribute mapping, and integration process
6. **Validation Suite**: A suite of tests to validate the correctness of the transcoder's output
7. **Sample Dataset**: A sample dataset demonstrating successful routing with Overture data

This milestone represents the first practical use of Overture data for routing applications, enabling developers to use Overture data with the Valhalla routing engine without modifying Valhalla's core code.

### Phase 2: Direct Integration and Research (5-8 months)

#### Architecture

In Phase 2, we'll explore more efficient integration methods and begin research on an Overture-specific tile format.

```
┌─────────────┐     ┌───────────────┐     ┌───────────────┐
│  Overture   │     │  Enhanced     │     │  Valhalla     │
│  Data       │────▶│  Integration  │────▶│  Routing      │
│ (GeoParquet)│     │  Layer        │     │  Engine       │
└─────────────┘     └───────────────┘     └───────────────┘
                           │
                           ▼
                    ┌───────────────┐
                    │  Tile Format  │
                    │  Research     │
                    └───────────────┘
```

#### Direct Integration with Valhalla

We'll explore modifying Valhalla's code to accept in-memory data structures directly, bypassing the binary file stage. This would involve:

1. Creating a new version of `ConstructEdges` that accepts in-memory data structures
2. Building these structures directly from Overture data
3. Passing them to the modified function

This approach could improve performance by eliminating file I/O overhead, but it requires modifying Valhalla's core code.

#### Tile Format Research

In parallel, we'll research tile formats for Overture data:

1. Analyze existing graph tile formats (Valhalla, OSRM, etc.)
2. Identify optimization opportunities for Overture data
3. Design a prototype tile format
4. Test with small datasets

This research will inform the design of the Overture Graph Tiles format in Phase 3.

#### Advanced Feature Implementation

We'll also implement support for advanced features:

1. Turn restrictions
2. Time-dependent attributes
3. Administrative boundaries
4. Special road features (bridges, tunnels, etc.)

These features are important for realistic routing but require careful mapping between Overture and Valhalla's data models.

#### Phase 2 Milestone Achievement

By the end of Phase 2, we will have:

1. **Direct Integration Prototype**: A working prototype that demonstrates direct integration with Valhalla's core code, bypassing the binary file stage
2. **Performance Analysis**: Comprehensive benchmarks comparing the binary file approach with direct integration
3. **Tile Format Specification**: A detailed specification for Overture Graph Tiles, including data structures, encoding formats, and spatial indexing
4. **Prototype Tile Implementation**: A small-scale implementation of the tile format with basic functionality
5. **Advanced Feature Support**: Implementation of turn restrictions, time-dependent attributes, and other advanced features
6. **Research Documentation**: Detailed documentation of our research findings, including comparisons of different tile formats and integration approaches
7. **Proof of Concept**: A demonstration of routing using the prototype tile format

This milestone represents a significant advancement in our understanding of how to optimize Overture data for routing applications. The research and prototypes developed during this phase will provide the foundation for the full implementation in Phase 3.

### Phase 3: Overture Graph Tiles Implementation (8-12 months)

#### Architecture

In Phase 3, we'll implement a full Overture Graph Tiles format optimized for Overture's data model and routing applications.

```
┌─────────────┐     ┌───────────────┐     ┌───────────────┐
│  Overture   │     │  Overture     │     │  Overture     │
│  Data       │────▶│  Tile         │────▶│  Graph        │
│ (GeoParquet)│     │  Builder      │     │  Tiles        │
└─────────────┘     └───────────────┘     └───────────────┘
                                                │
                                                ▼
┌─────────────┐     ┌───────────────┐     ┌───────────────┐
│  Routing    │     │  Routing      │     │  Converter    │
│  API        │◀────│  Engine       │◀────│  Modules      │
│             │     │               │     │               │
└─────────────┘     └───────────────┘     └───────────────┘
```

#### Tile Structure

The Overture Graph Tiles will consist of:

1. **Tile Hierarchy**:
   - Multiple levels for different road classifications
   - Optimized for Overture's road network

2. **Core Components**:
   - Nodes (derived from Connectors)
   - Directed Edges (derived from Segments)
   - Edge Attributes (derived from properties)
   - Turn Restrictions
   - Administrative Information

3. **Optimizations**:
   - Spatial indexing for efficient lookup
   - Compression for reduced storage
   - Caching-friendly structure

#### Detailed Tile Structure

```
┌─────────────────────────────────────────────────────────┐
│ Overture Graph Tile                                     │
├─────────────────────────────────────────────────────────┤
│ Header                                                  │
│  - Tile ID                                              │
│  - Version                                              │
│  - Bounding Box                                         │
│  - Creation Date                                        │
├─────────────────────────────────────────────────────────┤
│ Nodes                                                   │
│  - Node ID                                              │
│  - Location                                             │
│  - Type                                                 │
│  - Access                                               │
│  - Edge Index                                           │
├─────────────────────────────────────────────────────────┤
│ Directed Edges                                          │
│  - Edge ID                                              │
│  - Start Node                                           │
│  - End Node                                             │
│  - Length                                               │
│  - Speed                                                │
│  - Access                                               │
│  - Classification                                       │
│  - Attributes Index                                     │
├─────────────────────────────────────────────────────────┤
│ Edge Attributes                                         │
│  - Names                                                │
│  - Shape Points                                         │
│  - Lane Information                                     │
│  - Turn Restrictions                                    │
│  - Traffic Data                                         │
├─────────────────────────────────────────────────────────┤
│ Administrative Information                              │
│  - Boundaries                                           │
│  - Countries                                            │
│  - Regions                                              │
└─────────────────────────────────────────────────────────┘
```

#### Routing Engine Integration

We'll create converters to integrate Overture Graph Tiles with multiple routing engines:

1. **Valhalla**: Convert to Valhalla's tile format
2. **OSRM**: Convert to OSRM's graph format
3. **GraphHopper**: Convert to GraphHopper's graph format

This approach allows Overture Graph Tiles to be used with existing routing engines while maintaining the advantages of the optimized format.

#### API Development

We'll also develop APIs for working with Overture Graph Tiles:

1. **Tile Creation API**: For building tiles from Overture data
2. **Tile Access API**: For reading and querying tiles
3. **Routing API**: For performing routing operations directly on tiles

These APIs will make it easier for developers to work with Overture Graph Tiles in their applications.

#### Phase 3 Milestone Achievement

By the end of Phase 3, we will have:

1. **Complete Tile Format Implementation**: A fully implemented Overture Graph Tiles format with all planned features and optimizations
2. **Tile Builder Tool**: A high-performance tool for creating Overture Graph Tiles from Overture transportation data
3. **Multiple Engine Converters**: Converters that transform Overture Graph Tiles to formats compatible with Valhalla, OSRM, and GraphHopper
4. **Comprehensive API**: A well-documented API for creating, accessing, and using Overture Graph Tiles
5. **Performance-Optimized Implementation**: An implementation that has been thoroughly benchmarked and optimized for speed and memory usage
6. **Full Documentation**: Comprehensive documentation covering the tile format, APIs, and integration options
7. **Example Applications**: Sample applications demonstrating the use of Overture Graph Tiles for various routing scenarios
8. **Large-Scale Validation**: Validation of the tile format with large, real-world datasets covering diverse geographic areas

This milestone represents the culmination of our work on Overture Graph Tiles. The resulting format and tools will provide a powerful foundation for routing applications using Overture data, offering both performance advantages and flexibility in integration with existing systems.

## Technical Challenges and Solutions

The development of Overture Graph Tiles presents several significant technical challenges that we must address to ensure success. This section explores these challenges and our proposed solutions in depth.

### Binary Format Compatibility

Creating binary files that are compatible with Valhalla's expectations is a complex undertaking. Valhalla's binary formats are not formally documented, and they rely on C++ memory layouts that can be difficult to reproduce in other languages.

The challenge is compounded by the fact that these binary formats may change between Valhalla versions. Even minor differences in structure alignment, padding, or endianness could cause Valhalla to misinterpret our generated files, leading to routing errors or crashes.

Our solution involves a multi-faceted approach:

First, we will conduct thorough analysis of Valhalla's code to understand exactly how it reads and writes these binary files. This includes examining the `sequence` class implementation and the structures it serializes.

Second, we will develop a comprehensive test suite that generates test files with known content and verifies that Valhalla interprets them correctly. This will include edge cases like large values, special characters, and boundary conditions.

Third, we will implement careful binary validation in our transcoder. Before outputting a file, we'll verify that it meets all the structural requirements that Valhalla expects. This includes checking field alignments, ensuring correct byte ordering, and validating size constraints.

By combining these approaches, we can ensure that our generated binary files will be compatible with Valhalla's expectations, even as the format evolves over time.

### Attribute Mapping Complexity

Mapping between Overture's rich, normalized schema and Valhalla's OSM-derived attributes presents significant challenges. The two systems use different terminology, different value ranges, and different organizational principles.

For example, Overture uses explicit connectors to define the topology of the road network, while Valhalla infers topology from shared nodes between ways. Overture has a structured approach to access restrictions with scoping rules, while Valhalla uses a flatter model derived from OSM tags.

Our solution is to develop a comprehensive mapping framework that addresses these differences:

We will start by identifying the core attributes that are essential for basic routing functionality, such as road classification, access restrictions, and directionality. For these attributes, we will develop precise, well-tested mappings that ensure routing accuracy.

Next, we will implement an extensible attribute mapping system that can be enhanced over time. This system will include validation rules to ensure that mapped attributes make sense in the target system.

For complex attributes like turn restrictions and time-dependent properties, we will develop specialized mapping logic that preserves the semantic meaning of the original data, even if the representation differs.

Throughout this process, we will maintain detailed documentation of our mapping decisions, including the rationale behind each choice and any limitations or edge cases that users should be aware of.

### Performance with Large Datasets

Overture data covers vast geographic areas, and processing this data efficiently requires careful attention to performance and memory usage. A naive implementation could easily consume excessive memory or take prohibitively long to process.

Our solution combines several performance optimization strategies:

We will implement streaming processing wherever possible, reading and processing data in chunks rather than loading entire datasets into memory. This approach allows us to handle arbitrarily large datasets with bounded memory usage.

For operations that benefit from parallelism, we will implement multi-threaded processing. This includes tile creation, attribute mapping, and file writing. By carefully managing thread synchronization and work distribution, we can achieve near-linear scaling with the number of available CPU cores.

We will optimize our data structures for memory efficiency, using compact representations and avoiding unnecessary duplication. This includes using appropriate integer sizes, reusing string instances, and employing memory-efficient containers.

For particularly large datasets, we will implement incremental processing capabilities that allow work to be resumed if interrupted. This includes checkpointing mechanisms and progress tracking.

These optimizations will ensure that our tools can handle even the largest Overture datasets efficiently, making them practical for real-world use cases.

### Tile Format Design

Designing an optimal tile format for routing is a complex challenge that requires balancing multiple competing concerns. The format must be efficient for common routing operations, compact enough for reasonable storage and transmission, and flexible enough to accommodate future enhancements.

Our approach to tile format design will be methodical and evidence-based:

We will begin by analyzing existing tile formats used by Valhalla, OSRM, and other routing engines, identifying their strengths and weaknesses. This analysis will include both theoretical considerations and practical benchmarking.

Based on this analysis, we will design a tile format specifically optimized for Overture's data model. This format will leverage Overture's normalized schema and first-class topological nodes to create a more efficient representation than would be possible with OSM-derived data.

We will implement prototype versions of the format and test them with realistic routing scenarios. These tests will measure performance metrics like query time, memory usage, and tile size.

Based on these tests, we will refine the format to optimize for the most common operations while maintaining reasonable performance for edge cases. This may include specialized indexing structures, compression techniques, or caching optimizations.

Throughout this process, we will maintain backward compatibility and provide clear migration paths for users as the format evolves.

### Routing Engine Integration

Ensuring compatibility with multiple routing engines presents challenges due to their different expectations, data models, and APIs. Each engine has its own assumptions and requirements that must be accommodated.

Our integration strategy focuses on clean interfaces and thorough testing:

We will design converter modules with clear, well-defined interfaces. Each converter will be responsible for transforming Overture Graph Tiles into the format expected by a specific routing engine.

These converters will be thoroughly tested with a variety of input data and routing scenarios. This testing will include both functional correctness (do routes match expectations?) and performance characteristics (is conversion efficient?).

We will provide comprehensive integration documentation for each supported routing engine, including example configurations, common pitfalls, and performance tuning advice.

For popular engines like Valhalla, we will develop deeper integration options that offer better performance or additional features. This might include custom plugins or extensions that allow the engine to work more directly with Overture Graph Tiles.

By taking this approach, we can ensure that Overture Graph Tiles work seamlessly with a variety of routing engines, giving users flexibility in their implementation choices.

## Implementation Timeline

### Phase 1: Valhalla Integration (Months 1-4)

- **Month 1**: Binary format analysis and Rust structure implementation
- **Month 2**: Basic entity mapping and attribute mapping
- **Month 3**: Testing, validation, and initial performance optimization
- **Month 4**: Documentation, examples, and release of initial version

### Phase 2: Direct Integration and Research (Months 5-8)

- **Month 5**: Direct integration prototype and performance evaluation
- **Month 6**: Tile format analysis and Overture tile design
- **Month 7**: Prototype implementation of tile format
- **Month 8**: Testing, benchmarking, and research documentation

### Phase 3: Overture Graph Tiles (Months 9-12+)

- **Month 9-10**: Tile builder implementation
- **Month 11**: Routing engine integration and converter modules
- **Month 12**: Performance optimization and API development
- **Month 13+**: Documentation, examples, and production readiness

## Success Metrics

1. **Functional Metrics**:
   - Successful routing between arbitrary points using Overture data
   - Support for multiple transportation modes
   - Handling of turn restrictions and access limitations

2. **Performance Metrics**:
   - Tile creation time compared to existing solutions
   - Routing query performance
   - Memory usage during processing and routing

3. **Integration Metrics**:
   - Number of supported routing engines
   - Ease of integration (measured by lines of code/configuration)
   - Compatibility with existing tools

4. **Adoption Metrics**:
   - Number of projects using Overture Graph Tiles
   - Community contributions
   - Feature requests and feedback

## Alternatives Considered

### 1. Fork Existing Routing Engine

**Approach**: Create a fork of an existing routing engine (e.g., Valhalla) specifically for Overture data.

**Pros**:
- Leverages existing routing algorithms
- Potentially faster initial implementation

**Cons**:
- Creates maintenance burden
- Limits flexibility for Overture-specific optimizations
- Ties solution to a specific routing engine

### 2. Direct Use of GeoParquet

**Approach**: Use GeoParquet data directly for routing without intermediate tile format.

**Pros**:
- Eliminates conversion step
- Works directly with source data

**Cons**:
- Likely slower for routing operations
- Requires more memory
- Less optimized for routing-specific queries

### 3. Third-Party Conversion Tools

**Approach**: Rely on third-party tools to convert Overture data to existing formats.

**Pros**:
- Leverages existing tools
- Reduces development effort

**Cons**:
- Less control over conversion process
- May not fully utilize Overture's data model
- Potential compatibility issues

## Conclusion

The proposed Overture Graph Tiles project provides a path from immediate integration with existing routing engines to a fully optimized, Overture-specific solution. By taking a phased approach, we can deliver value quickly while working toward a more optimized long-term solution.

The unique characteristics of Overture's transportation schema—including normalized attributes, first-class topological nodes, and comprehensive properties—make it well-suited for an optimized graph representation. The proposed tile format will leverage these characteristics to provide efficient routing capabilities while maintaining compatibility with existing routing engines.

We recommend proceeding with Phase 1 implementation while beginning research for Phases 2 and 3.

## References

1. Overture Maps Foundation Transportation Schema
2. Valhalla Routing Engine Documentation
3. GraphHopper Routing Engine Documentation
4. OSRM (Open Source Routing Machine) Documentation
5. GeoParquet Specification
