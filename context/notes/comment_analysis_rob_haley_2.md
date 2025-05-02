# Analysis of Rob Haley's Second Comment

## The Comment

> I currently have attribute parsing working in what I would describe as an "insane" way and got the output of that working in the Esri routing engine. I think I could simplify the way I am doing it though, I don't think I need to support all the values I am now. There should be some opportunity to combine values which is what I am going to try and document with what I described. Ideally we get a nice data structure out of it.

## Key Points

1. **Existing Work**: Rob has already implemented attribute parsing for Overture data.

2. **Current State**: The implementation is complex ("insane").

3. **Simplification Opportunity**: Potential to simplify by not supporting all values.

4. **Goal**: Create a cleaner data structure.

## Implications

Rob's comment suggests that he has already done significant work on parsing Overture attributes for routing purposes, specifically for the Esri routing engine. This is valuable information because:

1. It confirms that Overture data can be used for routing with appropriate transformation.

2. It suggests that the attribute mapping is complex but feasible.

3. It indicates that there may be opportunities to simplify the attribute mapping by focusing on the most important attributes.

4. It provides a potential reference point or starting point for our Valhalla integration work.

## Relevant Considerations

### Attribute Mapping Complexity

Rob's description of his attribute parsing as "insane" suggests that mapping between Overture's schema and routing engine requirements is non-trivial. This aligns with Kevin's concern about "the work to parse its schema."

### Attribute Prioritization

Rob's comment about not needing to "support all the values" suggests that we could take a phased approach:

1. Start with the most critical attributes for basic routing
2. Add support for additional attributes over time

### Data Structure Design

Rob's goal of getting a "nice data structure" aligns with our need to map to Valhalla's intermediate structures like `OSMData`, `OSMWay`, etc.

## Next Steps

1. **Consult with Rob**: It would be valuable to understand more about:
   - Which attributes he found most important for routing
   - What made the parsing "insane" and how it could be simplified
   - What his current data structure looks like

2. **Compare with Valhalla Requirements**: Analyze how Rob's approach for the Esri routing engine might translate to Valhalla's requirements.

3. **Identify Simplification Opportunities**: Look for ways to simplify the attribute mapping while still supporting essential routing functionality.

## Conclusion

Rob's experience with parsing Overture attributes for the Esri routing engine provides valuable insights and potentially a head start for our Valhalla integration work. His suggestion to simplify the attribute mapping aligns with an incremental approach to implementation.
