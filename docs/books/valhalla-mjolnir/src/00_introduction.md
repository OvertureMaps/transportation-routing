# Understanding Valhalla's Mjolnir: The Graph Tile Builder
By Swagata Prateek

## Introduction

This document analyzes Valhalla's Mjolnir component, which converts OpenStreetMap (OSM) data into Valhalla's tiled routing graph structure. The goal is to provide an understanding of how Mjolnir works, enabling the creation of custom graph tile builders.

Mjolnir, named after Thor's hammer in Norse mythology, processes geographic data and transforms it into a structured, hierarchical graph that Valhalla uses for routing. Understanding Mjolnir is essential for creating custom graph tile builders or modifying Valhalla's data processing pipeline.

## Document Structure

This document is organized into the following chapters:

1. **Introduction and Overview** - Overview of Mjolnir and its role in Valhalla
2. **Valhalla's Tiled Structure** - The graph tile concept and hierarchical structure
3. **OSM Data Processing** - How Mjolnir processes OpenStreetMap data
4. **Graph Construction Process** - The process of building the graph
5. **Core Data Structures** - Key classes and data structures in the graph building process
6. **Tile Creation and Management** - How tiles are created, stored, and managed
7. **Hierarchical Graph Building** - Creating the multi-level routing hierarchy
8. **Costing and Edge Attribution** - How road attributes and routing costs are assigned
9. **Special Cases and Features** - Handling complex scenarios like restrictions, transit, etc.
10. **Building a Graph Tile Builder** - Guidelines for implementation

Each chapter includes code examples, references to specific files in the Valhalla codebase, and explanations of algorithms and processes.

This document explores how Mjolnir transforms map data into a routing graph.
