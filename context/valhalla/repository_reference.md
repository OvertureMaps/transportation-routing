# Valhalla Repository Reference

## Repository Location

The Valhalla repository is available locally for code reference and analysis. This repository contains the actual implementation of the concepts discussed in the Valhalla Mjolnir documentation book.

## Purpose

- Examine the actual code implementation of concepts described in the documentation
- Understand the specific data structures and interfaces needed for integration
- Verify the accuracy of our understanding from the documentation
- Identify specific integration points for the Overture Transportation Schema

## Key Components to Examine

- `src/mjolnir/`: Contains the implementation of the Mjolnir graph tile builder
- `valhalla/mjolnir/`: Header files defining the public interfaces
- `valhalla/baldr/`: Core data structures used in the graph
- `src/baldr/`: Implementation of these data structures
- `src/mjolnir/pbfgraphparser.cc`: Example of parsing external data into Valhalla's structures
- `src/mjolnir/graphbuilder.cc`: Implementation of the graph building process

## Integration Focus

Based on Kevin Kreiser's comments, we should focus particularly on:
- The `ConstructEdges` phase
- The `GraphBuilder` stage
- Data structures like `OSMData`, `OSMWay`, `OSMNode`, and `OSMWayNode`
- How administrative boundary information is handled

## Note

When referencing code in our documentation, we'll use relative paths (e.g., `src/mjolnir/graphbuilder.cc`) rather than the full filesystem path for clarity and portability.
