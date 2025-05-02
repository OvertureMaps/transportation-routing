# Analysis of PR Comments

## Comment 1: Brad Richardson
> Great work 👏

Simple approval comment, no specific concerns to address.

## Comment 2: Rob Haley
> In reading through this there is a mechanism for Mjolnir to be extended to read in a custom dataset and process it to tile, but in doing so we are essentially building out Mjolnir itself. It would be essentially useless to anyone with that custom fork. And I don't suspect we want to support the a routing tool. Am I wrong I saying that?
> 
> Alternatively we don't want to reverse ourselves back into a pbf. That's not really value add.
> 
> I do think Valhalla is cool but would we be better off finding something better equipped to read in schematized segment data?
> 
> One of the tasks I will start, which would ultimately tie into this (or any other solution) was trying to list out what types of modes/maneuvers I think we are well positioned to support and trying to build some definitions around what those are in terms of how they are defined by the attributes. That could help with the core data structures part from chapter 4.

### Key Concerns:
1. **Forking Mjolnir**: Concern about creating a custom fork of Mjolnir that wouldn't be useful to others
2. **Converting to PBF**: Reluctance to convert Overture data back to OSM PBF format
3. **Alternative Solutions**: Question about whether there are better tools for schematized segment data
4. **Mode/Maneuver Support**: Plan to document supported modes and maneuvers based on available attributes

### Implications for Our Plan:
- Need to consider whether a custom Mjolnir implementation is the right approach
- Should explore integration points that don't require full forking of Mjolnir
- Need to document which routing features (modes, maneuvers) we can support with Overture data

## Comment 3: Kevin Kreiser
> early on i dismissed the idea of direct support for overture in valhalla because its just a crazy amount of work. indeed i suggested the best bang for everyones buck is converting it back to osm pbf so all the osm tools work with it. having worked now with overture for the sake of display/tiles and with the differences in the schema i do see that it might be a little easier to use for routing in some regards than osm since topological nodes are first class citizens in overture. the biggest struggle is just doing all the attribute parsing in a sane way but at least with overture it should be already "normalized" (ie there is only one tagging variant of a particular attribute/concept that the parser has to care about). so yeah might be interesting to consider it in valhalla. also the spatial nature of geoparquet makes processing (tile cutting) easier.
> 
> at a high level i could see an architecture where overture could fit in. today valhalla is a pipeline of steps:
> 
> 1. parse osm in to temporary data structures
> 2. demarcate the topology assigning graph nodes and edges
> 3. cut tiles
> 4. a bunch more stages to enhance those ...
> 
> for overture we would need to do a first pass over the overture data to assign graphid (nodes and edges) but after that we could simply start cutting tiles. the only annoying part would then be the work to parse its schema vs the osm one but as i mentioned at least its better normalized than osm.
> 
> dont get me wrong, its still a lot of work. and maybe there is some trickery about subsegment attributions overlapping and making a misery for us in the first step but at the same time i think finding a router whose graph preparation is easily configurable to whatever schema and then doing the work to configure it is likely similarly difficult. i could be wrong though!

### Key Points:
1. **Initial Skepticism**: Initially dismissed direct Overture support in Valhalla due to complexity
2. **Advantages of Overture**:
   - Topological nodes are first-class citizens
   - Schema is normalized (consistent attribute representation)
   - GeoParquet format makes tile cutting easier
3. **Valhalla Pipeline**:
   - Parse data into temporary structures
   - Assign graph nodes and edges
   - Cut tiles
   - Enhancement stages
4. **Integration Approach**: For Overture, we'd need to:
   - Do a first pass to assign GraphIDs to nodes and edges
   - Then start cutting tiles
5. **Challenges**:
   - Parsing Overture schema vs. OSM schema
   - Potential issues with subsegment attributions overlapping

### Implications for Our Plan:
- Consider leveraging Overture's normalized schema and first-class topological nodes
- Focus on the first pass of assigning GraphIDs to Overture data
- Be aware of potential challenges with subsegment attributions

## Comment 4: Rob Haley
> I currently have attribute parsing working in what I would describe as an "insane" way and got the output of that working in the Esri routing engine. I think I could simplify the way I am doing it though, I don't think I need to support all the values I am now. There should be some opportunity to combine values which is what I am going to try and document with what I described. Ideally we get a nice data structure out of it.

### Key Points:
1. **Existing Work**: Rob has already implemented attribute parsing for Overture data
2. **Current State**: The implementation is complex ("insane")
3. **Simplification Opportunity**: Potential to simplify by not supporting all values
4. **Goal**: Create a cleaner data structure

### Implications for Our Plan:
- Coordinate with Rob to understand his current attribute parsing approach
- Consider which attributes are essential vs. optional for routing
- Focus on creating a clean, simplified data structure

## Comment 5: Kevin Kreiser
> ok ive been reading through this and while i feel like ive made a small dent im not sure that it would be beneficial for me to continue.
> 
> if we assume (its what im assuming) the whole point of this exercise is to understand what one would need to do to route on overture then i feel like going deep on the integration point between overture and valhalla is more important than breadthwise exploring how valhalla works. that can wait until we have something sort of working.
> 
> my suggestion would be as follows:
> 
> mjolnir is a pipeline of steps. between each step there is an intermediate representation of the graph data. by the end you have the final graph. the pipeline can be paused or resumed between any two stages of the build. the intermediate data that each step produces allows one to pick up where one left off. the key to making compatible data for valhalla then is to simply find the stage where you can generate that intermediate data. in this case that stage is either the `ConstructEdges` phase or the `GraphBuilder` stage. i personally think the former is both ideal and highly plausible.
> 
> concretely it means you need to write a transcoder from overture into these fixed size structures (`OSMData`, `OSMWay`, `OSMNode`, `OSMWayNode`, and a few others etc) that `ConstructEdges` expects. the only other thing you need is a means of getting the admin boundary information into a spatialite db but that might be somewhat easy with something like duckdb and its many hookins for standardized spatial formats.
> 
> tldr; all you really need to know is what is in OSMData, what it looks like and how to fill it out from overture rather than from osm. thankfully those structures are simple so it should be fairly easy to grok. its good to know about the rest of the tile building pipeline but for the purposes of overture the bulk of the focus should be on parsing into these intermediate structures

### Key Points:
1. **Focus Recommendation**: Focus on the integration point between Overture and Valhalla
2. **Pipeline Insight**: Mjolnir pipeline has intermediate representations between steps
3. **Integration Point**: Target either the `ConstructEdges` phase or the `GraphBuilder` stage
4. **Specific Approach**: Write a transcoder from Overture to fixed-size structures (`OSMData`, `OSMWay`, `OSMNode`, `OSMWayNode`)
5. **Admin Boundaries**: Need a way to get admin boundary information into a spatialite DB (possibly using DuckDB)
6. **Key Takeaway**: Focus on understanding and filling out the OSMData structure from Overture data

### Implications for Our Plan:
- Focus on the `ConstructEdges` phase as the integration point
- Create a detailed mapping between Overture schema and Valhalla's OSMData structures
- Research how to handle admin boundaries using DuckDB or similar tools
- Prioritize understanding the specific data structures needed rather than the entire pipeline
