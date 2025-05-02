# Overture Transportation Schema Documentation

## Official Documentation

The public documentation about Overture transportation schema is available at:
https://docs.overturemaps.org/schema/concepts/by-theme/transportation/

## Key Points

- The Overture Transportation Schema defines how transportation network data is structured in the Overture Maps Foundation data
- This schema will need to be mapped to Valhalla's internal data structures for routing
- Understanding this schema is essential for creating a transcoder from Overture data to Valhalla's expected input format

## Next Steps

- Review the schema documentation in detail
- Identify key entities and attributes in the Overture transportation schema
- Map these entities to Valhalla's data structures (OSMData, OSMWay, OSMNode, etc.)
- Determine any gaps or challenges in the mapping process

## Notes

This documentation will be crucial for developing the transcoder mentioned in Kevin Kreiser's PR comments, which will convert Overture data into the format expected by Valhalla's `ConstructEdges` phase.
