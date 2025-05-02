# Analysis of Rob Haley's First Comment

## The Comment

> In reading through this there is a mechanism for Mjolnir to be extended to read in a custom dataset and process it to tile, but in doing so we are essentially building out Mjolnir itself. It would be essentially useless to anyone with that custom fork. And I don't suspect we want to support the a routing tool. Am I wrong I saying that?
> 
> Alternatively we don't want to reverse ourselves back into a pbf. That's not really value add.
> 
> I do think Valhalla is cool but would we be better off finding something better equipped to read in schematized segment data?
> 
> One of the tasks I will start, which would ultimately tie into this (or any other solution) was trying to list out what types of modes/maneuvers I think we are well positioned to support and trying to build some definitions around what those are in terms of how they are defined by the attributes. That could help with the core data structures part from chapter 4.

## Key Concerns

1. **Custom Fork Concern**: Rob is concerned that extending Mjolnir to read a custom dataset would essentially require building out Mjolnir itself, resulting in a custom fork that wouldn't be useful to others.

2. **PBF Conversion**: He doesn't want to convert Overture data back to OSM PBF format as that doesn't add value.

3. **Alternative Tools**: He questions whether there might be better tools for handling schematized segment data than Valhalla.

4. **Mode/Maneuver Support**: He plans to document which modes and maneuvers Overture data can support, which could help with defining core data structures.

## Relevant Valhalla Code

To understand Rob's concerns about extending Mjolnir, we should examine how Valhalla's data ingestion pipeline works and what would be involved in extending it.

### Mjolnir Extension Points

Valhalla's Mjolnir component is primarily designed to work with OSM data, but it does have some extension points:

1. **Custom Data Parsers**: Mjolnir has a modular design where different parsers can be implemented for different data sources. The main parser is `OSMPBFParser`, but the architecture allows for other parsers.

2. **Lua Tag Transformation**: Valhalla uses Lua scripts to transform OSM tags into its internal representation, which provides some flexibility.

3. **Intermediate Data Structures**: The pipeline uses intermediate data structures that could potentially be populated from non-OSM sources.

### Extending vs. Forking

Rob's concern about "building out Mjolnir itself" is valid. Looking at the codebase, there are several challenges to extending Mjolnir without forking:

1. **OSM-Centric Design**: Many parts of the code assume OSM data structures and tagging conventions.

2. **Tight Coupling**: The components are tightly coupled, making it difficult to replace just the parsing stage.

3. **Limited Extension Points**: While there are some extension points, they may not be sufficient for a completely different data model like Overture.

## Potential Solutions

Based on the code examination, there are a few potential approaches to address Rob's concerns:

1. **Integration at a Later Stage**: As Kevin later suggests, we could bypass the early parsing stages and integrate at the `ConstructEdges` phase, which would avoid forking much of Mjolnir.

2. **Transcoder Approach**: Instead of extending Mjolnir directly, we could create a separate transcoder that converts Overture data to a format Mjolnir can consume.

3. **Alternative Routing Engines**: We could explore other routing engines that might be better designed for schematized data, as Rob suggests.

## Conclusion

Rob's concerns are valid based on the Valhalla codebase structure. The tight coupling and OSM-centric design of Mjolnir would make direct extension challenging without significant forking. This supports Kevin's later suggestion to focus on a specific integration point rather than trying to extend the entire pipeline.
