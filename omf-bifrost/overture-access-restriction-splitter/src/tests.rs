#[cfg(test)]
mod tests {
    use crate::restriction_splitter::split_segments;
    use overture_types::{ConnectorRef, Segment};
    use geo_types::{LineString, Coord};

    /// Helper function to build a segment with given geometry, access restrictions, and connectors
    fn build_segment(geometry: &[Coord], access_restrictions: Vec<(f64, f64)>, connectors: Vec<ConnectorRef>) -> Segment {
        let access_restrictions = access_restrictions.into_iter().map(|(start, end)| {
            overture_types::properties::AccessRestriction {
                access_type: "denied".to_string(),
                between: Some((start, end)),
                when: None,
            }
        }).collect();

        Segment {
            id: "id".to_string(),
            geometry: LineString::from(geometry.to_vec()),
            properties: overture_types::properties::SegmentProperties {
                class: Some("residential".to_string()),
                names: None,
                access_restrictions: Some(access_restrictions),
                speed_limits: None,
                subtype: None,
                surface: None,
            },
            connectors,
        }
    }

    fn fuzzy_compare(a: f64, b: f64) -> bool {
        let epsilon = 1e-4;
        (a - b).abs() <= epsilon
    }

    fn fuzzy_compare_coords(c1: &Coord, c2: &Coord) -> bool {
        fuzzy_compare(c1.x, c2.x) && fuzzy_compare(c1.y, c2.y)
    }

    #[test]
    fn test_split_none() {
        let segment = build_segment(
            &[
                geo_types::Coord { x: 0.0, y: 0.0 },
                geo_types::Coord { x: 1.0, y: 0.0 },
                geo_types::Coord { x: 2.0, y: 0.0 },
                geo_types::Coord { x: 3.0, y: 0.0 },
            ],
            vec![],
            vec![ConnectorRef{ id: "conn-1".to_string(), at: 0.5 }],
        );
        let mut segments = vec![segment];
        split_segments(&mut segments);

        // No splits should occur
        assert_eq!(segments.len(), 1);

        // Connector should be at the same spot
        assert_eq!(segments[0].connectors.len(), 1);
        assert!(fuzzy_compare(segments[0].connectors[0].at, 0.5));
    }

    #[test]
    fn test_split_one() {
        let segment = build_segment(
            &[
                geo_types::Coord { x: 0.0, y: 0.0 },
                geo_types::Coord { x: 1.0, y: 0.0 },
                geo_types::Coord { x: 2.0, y: 0.0 },
                geo_types::Coord { x: 3.0, y: 0.0 },
                geo_types::Coord { x: 4.0, y: 0.0 },
            ],
            vec![(0.25, 0.75)],
            vec![ConnectorRef{ id: "conn-1".to_string(), at: 0.125 },    // First connector is exactly in middle of first part
                             ConnectorRef{ id: "conn-2".to_string(), at: 0.375 },  // Second connector is one quarter into middle part
                             ConnectorRef{ id: "conn-3".to_string(), at: 0.83333 }], // Third connector is one third into last part
        );
        let mut segments = vec![segment];
        split_segments(&mut segments);
        // Should split into 3 segments
        assert_eq!(segments.len(), 3);

        // Only the middle segment should have an access restriction
        assert_eq!(segments[0].properties.access_restrictions.as_ref().unwrap().len(), 0);
        assert_eq!(segments[1].properties.access_restrictions.as_ref().unwrap().len(), 1);
        assert_eq!(segments[2].properties.access_restrictions.as_ref().unwrap().len(), 0);

        // Check geometries
        // First: 0.0 to 1.0
        assert_eq!(segments[0].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[0], &geo_types::Coord { x: 0.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[1], &geo_types::Coord { x: 1.0, y: 0.0 }));
        // Middle: 1.0 to 3.0
        assert_eq!(segments[1].geometry.0.len(), 3);
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[0], &geo_types::Coord { x: 1.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[1], &geo_types::Coord { x: 2.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[2], &geo_types::Coord { x: 3.0, y: 0.0 }));
        // Last: 3.0 to 4.0
        assert_eq!(segments[2].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[0], &geo_types::Coord { x: 3.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[1], &geo_types::Coord { x: 4.0, y: 0.0 }));

        // Check connectors
        assert_eq!(segments[0].connectors.len(), 2);
        assert_eq!(segments[1].connectors.len(), 3);
        assert_eq!(segments[2].connectors.len(), 2);

        // Check connectors added because of splits
        assert!(fuzzy_compare(segments[0].connectors[1].at, 1.0));
        assert!(fuzzy_compare(segments[1].connectors[0].at, 0.0));
        assert_eq!(segments[0].connectors[1].id, segments[1].connectors[0].id);

        assert!(fuzzy_compare(segments[1].connectors[2].at, 1.0));
        assert!(fuzzy_compare(segments[2].connectors[0].at, 0.0));
        assert_eq!(segments[1].connectors[2].id, segments[2].connectors[0].id);

        // Check connectors moved because of splits
        assert_eq!(segments[0].connectors[0].id, "conn-1");
        assert!(fuzzy_compare(segments[0].connectors[0].at, 0.5));
        assert_eq!(segments[1].connectors[1].id, "conn-2");
        assert!(fuzzy_compare(segments[1].connectors[1].at, 0.25));
        assert_eq!(segments[2].connectors[1].id, "conn-3");
        assert!(fuzzy_compare(segments[2].connectors[1].at, 0.3333));
    }

    #[test]
    fn test_split_two() {
        let segment = build_segment(
            &[
                geo_types::Coord { x: 0.0, y: 0.0 },
                geo_types::Coord { x: 1.0, y: 0.0 },
                geo_types::Coord { x: 2.0, y: 0.0 },
                geo_types::Coord { x: 3.0, y: 0.0 },
                geo_types::Coord { x: 4.0, y: 0.0 },
                geo_types::Coord { x: 5.0, y: 0.0 },
            ],
            vec![(0.2, 0.4), (0.6, 0.8)],
            // Connectors are all at 1/3 into each expected segment
            vec![ConnectorRef{ id: "conn-1".to_string(), at: 0.333 / 5.0},   // First segment
                 ConnectorRef{ id: "conn-2".to_string(), at: (1.0 + 0.333) / 5.0}, // Second segment
                 ConnectorRef{ id: "conn-3".to_string(), at: (2.0 + 0.333) / 5.0}, // Third segment
                 ConnectorRef{ id: "conn-4".to_string(), at: (3.0 + 0.333) / 5.0}, // Fourth segment
                 ConnectorRef{ id: "conn-5".to_string(), at: (4.0 + 0.333) / 5.0}]  // Fifth segment
        );
        let mut segments = vec![segment];
        split_segments(&mut segments);
        // Should split into 5 segments
        assert_eq!(segments.len(), 5);

        // Second and fourth segments should have access restrictions
        assert_eq!(segments[0].properties.access_restrictions.as_ref().unwrap().len(), 0);
        assert_eq!(segments[1].properties.access_restrictions.as_ref().unwrap().len(), 1);
        assert_eq!(segments[2].properties.access_restrictions.as_ref().unwrap().len(), 0);
        assert_eq!(segments[3].properties.access_restrictions.as_ref().unwrap().len(), 1);
        assert_eq!(segments[4].properties.access_restrictions.as_ref().unwrap().len(), 0);

        // Check geometries
        // First: 0.0 to 1.0
        assert_eq!(segments[0].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[0], &geo_types::Coord { x: 0.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[1], &geo_types::Coord { x: 1.0, y: 0.0 }));
        // Second: 1.0 to 2.0
        assert_eq!(segments[1].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[0], &geo_types::Coord { x: 1.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[1], &geo_types::Coord { x: 2.0, y: 0.0 }));
        // Third: 2.0 to 3.0
        assert_eq!(segments[2].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[0], &geo_types::Coord { x: 2.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[1], &geo_types::Coord { x: 3.0, y: 0.0 }));
        // Fourth: 3.0 to 4.0
        assert_eq!(segments[3].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[3].geometry.0[0], &geo_types::Coord { x: 3.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[3].geometry.0[1], &geo_types::Coord { x: 4.0, y: 0.0 }));
        // Fifth: 4.0 to 5.0
        assert_eq!(segments[4].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[4].geometry.0[0], &geo_types::Coord { x: 4.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[4].geometry.0[1], &geo_types::Coord { x: 5.0, y: 0.0 }));

        // Check connectors
        assert_eq!(segments[0].connectors.len(), 2);
        assert_eq!(segments[1].connectors.len(), 3);
        assert_eq!(segments[2].connectors.len(), 3);
        assert_eq!(segments[3].connectors.len(), 3);
        assert_eq!(segments[4].connectors.len(), 2);

        // Check connectors added because of splits
        assert!(fuzzy_compare(segments[0].connectors[1].at, 1.0));
        assert!(fuzzy_compare(segments[1].connectors[0].at, 0.0));
        assert_eq!(segments[0].connectors[1].id, segments[1].connectors[0].id);
        assert!(fuzzy_compare(segments[1].connectors[2].at, 1.0));
        assert!(fuzzy_compare(segments[2].connectors[0].at, 0.0));
        assert_eq!(segments[1].connectors[2].id, segments[2].connectors[0].id);
        assert!(fuzzy_compare(segments[2].connectors[2].at, 1.0));
        assert!(fuzzy_compare(segments[3].connectors[0].at, 0.0));
        assert_eq!(segments[2].connectors[2].id, segments[3].connectors[0].id);
        assert!(fuzzy_compare(segments[3].connectors[2].at, 1.0));
        assert!(fuzzy_compare(segments[4].connectors[0].at, 0.0));
        assert_eq!(segments[3].connectors[2].id, segments[4].connectors[0].id);

        // Check original connector moved because of splits
        assert_eq!(segments[0].connectors[0].id, "conn-1");
        assert!(fuzzy_compare(segments[0].connectors[0].at, 0.333));
        assert_eq!(segments[1].connectors[1].id, "conn-2");
        assert!(fuzzy_compare(segments[1].connectors[1].at, 0.333));
        assert_eq!(segments[2].connectors[1].id, "conn-3");
        assert!(fuzzy_compare(segments[2].connectors[1].at, 0.333));
        assert_eq!(segments[3].connectors[1].id, "conn-4");
        assert!(fuzzy_compare(segments[3].connectors[1].at, 0.333));
        assert_eq!(segments[4].connectors[1].id, "conn-5");
        assert!(fuzzy_compare(segments[4].connectors[1].at, 0.333));
    }

    #[test]
    fn test_split_shapepoints_1() {
        let segment = build_segment(
            &[
                geo_types::Coord { x: 0.0, y: 0.0 },
                geo_types::Coord { x: 4.0, y: 0.0 },
            ],
            vec![(0.25, 0.75)],
            vec![]
        );
        let mut segments = vec![segment];
        split_segments(&mut segments);

        // Should split into 3 segments
        assert_eq!(segments.len(), 3);

        // Check geometries
        // First: 0.0 to 1.0
        assert_eq!(segments[0].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[0], &geo_types::Coord { x: 0.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[1], &geo_types::Coord { x: 1.0, y: 0.0 }));
        // Middle: 1.0 to 3.0
        assert_eq!(segments[1].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[0], &geo_types::Coord { x: 1.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[1], &geo_types::Coord { x: 3.0, y: 0.0 }));
        // Last: 3.0 to 4.0
        assert_eq!(segments[2].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[0], &geo_types::Coord { x: 3.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[1], &geo_types::Coord { x: 4.0, y: 0.0 }));
    }

    #[test]
    fn test_split_shapepoints_2() {
        let segment = build_segment(
            &[
                geo_types::Coord { x: 0.0, y: 0.0 },
                geo_types::Coord { x: 2.0, y: 0.0 },
            ],
            vec![(0.25, 0.75)],
            vec![]
        );
        let mut segments = vec![segment];
        split_segments(&mut segments);

        // Should split into 3 segments
        assert_eq!(segments.len(), 3);

        // Check geometries
        // First: 0.0 to 0.5
        assert_eq!(segments[0].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[0], &geo_types::Coord { x: 0.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[1], &geo_types::Coord { x: 0.5, y: 0.0 }));
        // Middle: 0.5 to 1.5
        assert_eq!(segments[1].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[0], &geo_types::Coord { x: 0.5, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[1], &geo_types::Coord { x: 1.5, y: 0.0 }));
        // Last: 1.5 to 2.0
        assert_eq!(segments[2].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[0], &geo_types::Coord { x: 1.5, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[1], &geo_types::Coord { x: 2.0, y: 0.0 }));
    }

    #[test]
    fn test_split_shapepoints_3() {
        let segment = build_segment(
            &[
                geo_types::Coord { x: 0.0, y: 0.0 },
                geo_types::Coord { x: 1.0, y: 0.0 },
                geo_types::Coord { x: 2.0, y: 0.0 },
            ],
            vec![(0.25, 0.75)],
            vec![]
        );
        let mut segments = vec![segment];
        split_segments(&mut segments);

        // Should split into 3 segments
        assert_eq!(segments.len(), 3);

        // Check geometries
        // First: 0.0 to 0.5
        assert_eq!(segments[0].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[0], &geo_types::Coord { x: 0.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[1], &geo_types::Coord { x: 0.5, y: 0.0 }));
        // Middle: 0.5, 1.0, 1.5
        assert_eq!(segments[1].geometry.0.len(), 3);
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[0], &geo_types::Coord { x: 0.5, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[1], &geo_types::Coord { x: 1.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[2], &geo_types::Coord { x: 1.5, y: 0.0 }));
        // Last: 1.5 to 2.0
        assert_eq!(segments[2].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[0], &geo_types::Coord { x: 1.5, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[1], &geo_types::Coord { x: 2.0, y: 0.0 }));
    }

    #[test]
    fn test_split_shapepoints_4() {
        let segment = build_segment(
            &[
                geo_types::Coord { x: 0.0, y: 0.0 },
                geo_types::Coord { x: 5.0, y: 0.0 },
            ],
            vec![(0.2, 0.4), (0.6, 0.8)],
            vec![]
        );
        let mut segments = vec![segment];
        split_segments(&mut segments);

        // Should split into 5 segments
        assert_eq!(segments.len(), 5);

        // Check geometries
        // First: 0.0 to 1.0
        assert_eq!(segments[0].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[0], &geo_types::Coord { x: 0.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[0].geometry.0[1], &geo_types::Coord { x: 1.0, y: 0.0 }));

        // Second: 1.0 to 2.0
        assert_eq!(segments[1].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[0], &geo_types::Coord { x: 1.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[1].geometry.0[1], &geo_types::Coord { x: 2.0, y: 0.0 }));

        // Third: 2.0 to 3.0
        assert_eq!(segments[2].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[0], &geo_types::Coord { x: 2.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[2].geometry.0[1], &geo_types::Coord { x: 3.0, y: 0.0 }));

        // Fourth: 3.0 to 4.0
        assert_eq!(segments[3].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[3].geometry.0[0], &geo_types::Coord { x: 3.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[3].geometry.0[1], &geo_types::Coord { x: 4.0, y: 0.0 }));

        // Fifth: 4.0 to 5.0
        assert_eq!(segments[4].geometry.0.len(), 2);
        assert!(fuzzy_compare_coords(&segments[4].geometry.0[0], &geo_types::Coord { x: 4.0, y: 0.0 }));
        assert!(fuzzy_compare_coords(&segments[4].geometry.0[1], &geo_types::Coord { x: 5.0, y: 0.0 }));
    }
}
