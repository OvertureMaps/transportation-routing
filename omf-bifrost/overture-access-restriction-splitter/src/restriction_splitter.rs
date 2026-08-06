use overture_types::connector::ConnectorRef;
use overture_types::properties::{AccessRestriction};
use geo_types::{LineString, Coord};
use overture_types::Segment;

/// Represents corrected geometry and connector references for corrected segment part
#[derive(Debug)]
struct SplitSegmentPart
{
    /// Corrected geometry
    pub geometry: LineString<f64>,

    /// Corrected connector references
    pub connector_refs: Vec<ConnectorRef>,
}

/// Compares two floating point numbers with a small tolerance
fn fuzzy_compare(a: f64, b: f64) -> bool {
    // less than one centimeter at equator
    let epsilon = 1e-6;
    (a - b).abs() <= epsilon
}

/// Compares two coordinates with a small tolerance
fn fuzzy_compare_coords(c1: &Coord, c2: &Coord) -> bool {
    fuzzy_compare(c1.x, c2.x) && fuzzy_compare(c1.y, c2.y)
}

/// Removes duplicate start and end points from a LineString, while ensuring at least two points remain
fn remove_double_start_end(points: &mut LineString<f64>)
{
    if points.0.len() < 2 {
        panic!("LineString must have at least two points, but only has {}", points.0.len());
    }

    if points.0.len() == 2 {
        // We are done
        return;
    }

    // Check first point
    if fuzzy_compare_coords(&points.0[0], &points.0[1]) {
        // First two start points should be considered the same, remove first point
        points.0.remove(0);
        if points.0.len() == 2 {
            // A line string should have at least two points, so stop here
            return;
        }
    }

    // Check last point
    if fuzzy_compare_coords(&points.0[points.0.len() - 1], &points.0[points.0.len() - 2]) {
        // Last two end points should be considered the same, remove last point
        points.0.remove(points.0.len() - 1);
    }
}

/// Builds a new segment from the original segment and the split part, applying the given access restriction
fn build_segment_from_part(org_segment: &Segment, part: &SplitSegmentPart, access_restriction: Option<AccessRestriction>) -> Segment
{
    let mut access_restrictions: Vec<AccessRestriction> = Vec::new();
    if let Some(restriction) = access_restriction {
        access_restrictions.push(restriction);
    }

    let mut output = Segment {
        id: org_segment.id.clone(),
        properties: org_segment.properties.clone(),
        geometry: part.geometry.clone(),
        connectors: part.connector_refs.clone()
    };

    // Clean up geometry, deal with when access restriction border was on start or end point
    remove_double_start_end(&mut output.geometry);

    output.properties.access_restrictions = Some(access_restrictions);
    output
}

/// Calculates a linearly interpolated point between two coordinates relative to how far along line connecting them it should be
fn calc_interpolated_point(p1: &Coord, p2: &Coord, segment_length: f64, excess_length: f64) -> Coord
{
    let t = (segment_length - excess_length) / segment_length;
    let point_lon = p1.x * (1.0 - t) + p2.x * t;
    let point_lat = p1.y * (1.0 - t) + p2.y * t;
    Coord { x: point_lon, y: point_lat }
}

/// Extracts all data (geometry and connector references) between the given linear offsets from the segment
fn get_all_data_between_offsets(segment: &Segment, total_length: f64, start_linear_offset: f64, end_linear_offset: f64) -> SplitSegmentPart
{
    if !(0.0..=1.0).contains(&start_linear_offset) {
        panic!("Start linear offset out of range: {start_linear_offset}");
    }
    if !(0.0..=1.0).contains(&end_linear_offset) {
        panic!("End linear offset out of range: {end_linear_offset}");
    }
    if start_linear_offset > end_linear_offset {
        panic!("Start linear offset ({start_linear_offset}) can't be greater than end linear offset ({end_linear_offset})");
    }

    if fuzzy_compare(start_linear_offset, 0.0) && fuzzy_compare(end_linear_offset, 1.0) {
        // shortcut: return full segment
        return SplitSegmentPart {
            geometry: segment.geometry.clone(),
            connector_refs: segment.connectors.clone()
        };
    }

    let mut output = SplitSegmentPart {
        geometry: LineString::new(vec![]),
        connector_refs: Vec::new()
    };

    let start_length = start_linear_offset * total_length;
    let end_length = end_linear_offset * total_length;

    // Keep track of current length along the segment and if we have started adding points
    let mut current_length = 0.0;
    let mut started = false;

    // Loop through all points in the segment geometry
    for i in 1..segment.geometry.0.len() {
        let p1 = &segment.geometry.0[i - 1];
        let p2 = &segment.geometry.0[i];
        let segment_length = ((p2.y - p1.y).powi(2) + (p2.x - p1.x).powi(2)).sqrt();
        current_length += segment_length;

        if !started && (current_length > start_length) {
            // add interpolated point
            let excess_length = current_length - start_length;
            let interpolated_point = calc_interpolated_point(p1, p2, segment_length, excess_length);
            output.geometry.0.push(interpolated_point);
        }

        started = current_length >= start_length;

        if started {
            if current_length < end_length {
                output.geometry.0.push(*p2);
            } else {
                // add interpolated point
                let excess_length = current_length - end_length;
                let interpolated_point = calc_interpolated_point(p1, p2, segment_length, excess_length);
                output.geometry.0.push(interpolated_point);
                break;
            }
        }
    }

    // Adjust connector references
    for connector in segment.connectors.iter() {
        if connector.at < start_linear_offset {
            continue;
        }

        if connector.at > end_linear_offset {
            continue;
        }

        let mut new_connector = connector.clone();
        if fuzzy_compare(start_linear_offset, end_linear_offset) {
            // prevent division by zero
            new_connector.at = 0.5;
        } else {
            new_connector.at = (new_connector.at - start_linear_offset) / (end_linear_offset - start_linear_offset);
        }
        
        output.connector_refs.push(new_connector);
    }

    output
}

/// Fixes up the remaining segment after a split by adjusting its geometry, connector references, and access restrictions
fn fixup_remaining_segment(segment: &mut Segment, start_linear_offset: f64)
{
    if fuzzy_compare(start_linear_offset, 0.0) {
        // nothing to be done
        return;
    }

    let total_length = calc_segment_length(segment.clone());
    let split_segment_part = get_all_data_between_offsets(segment, total_length, start_linear_offset, 1.0);
    segment.geometry = split_segment_part.geometry;
    segment.connectors = split_segment_part.connector_refs;

    // fixup access restrictions: adjust their between values
    let mut new_access_restrictions: Vec<AccessRestriction> = Vec::new();
    for restriction in &segment.properties.access_restrictions.clone().unwrap() {
        if restriction.between.is_some() {
            let r_start = restriction.between.unwrap().0;
            let r_end = restriction.between.unwrap().1;

            if r_end <= start_linear_offset {
                // this restriction is before the split, skip it
                continue;
            }

            // fix for new start and length
            let r_start = (r_start - start_linear_offset) / (1.0 - start_linear_offset);
            let r_end = (r_end - start_linear_offset) / (1.0 - start_linear_offset);

            new_access_restrictions.push(AccessRestriction {
                between: Some((r_start, r_end)),
                ..restriction.clone()
            });
        }
    }
    segment.properties.access_restrictions = Some(new_access_restrictions);
}

/// Aproximates the length of the segment by summing the distances between its points
fn calc_segment_length(segment: Segment) -> f64
{
    let mut total_length = 0.0;
    for i in 1..segment.geometry.0.len() {
        let p1 = &segment.geometry.0[i - 1];
        let p2 = &segment.geometry.0[i];
        let segment_length = ((p2.y - p1.y).powi(2) + (p2.x - p1.x).powi(2)).sqrt();
        total_length += segment_length;
    }
    total_length
}

/// Splits a segment into multiple segments based on its access restrictions
fn split_segment(segment: &Segment, next_connector_id: &mut u32) -> Vec<Segment>
{
    // This will contain the remaining part of the segment to be processed
    let mut remaining_segment = segment.clone();

    // This will contain the output segments
    let mut output: Vec<Segment> = Vec::new();

    // Loop while there are access restrictions to process
    while !remaining_segment.properties.access_restrictions.as_ref().unwrap().is_empty() {
        let length = calc_segment_length(remaining_segment.clone());

        let access_restrictions = remaining_segment.properties.access_restrictions.clone().unwrap();
        let between = access_restrictions[0].between.as_ref().unwrap();
        let between_start = between.0;
        let between_end = between.1;

        // Build first part: before access restriction
        let mut first_part = get_all_data_between_offsets(&remaining_segment, length, 0.0, between_start);

        // Build second part: access restriction part
        let mut second_part = get_all_data_between_offsets(&remaining_segment, length, between_start, between_end);

        // Remove processed access restriction from remaining segment
        remaining_segment.properties.access_restrictions.as_mut().unwrap().remove(0);

        // Fixup remaining segment (part after this access restriction) so it can be processed in the next iteration
        fixup_remaining_segment(&mut remaining_segment, between_end);

        // Add connector reference between first and second part
        let connector_id = format!("split-{}-{}", segment.id, next_connector_id).replace(" ", "-");
        *next_connector_id += 1;
        let connector = ConnectorRef {
            id: connector_id.clone(),
            at: 1.0
        };
        first_part.connector_refs.push(connector.clone());
        let connector = ConnectorRef {
            id: connector_id,
            at: 0.0
        };
        // Make sure to insert at the start, since it's the start of the second part
        second_part.connector_refs.insert(0, connector.clone());

        // Add connector reference between second and remaining part
        let connector_id = format!("split-{}-{}", segment.id, next_connector_id).replace(" ", "-");
        *next_connector_id += 1;
        let connector = ConnectorRef {
            id: connector_id.clone(),
            at: 1.0
        };
        second_part.connector_refs.push(connector.clone());
        let connector = ConnectorRef {
            id: connector_id,
            at: 0.0
        };
        // Make sure to insert at the start, since it's the start of the remaining part
        remaining_segment.connectors.insert(0, connector.clone());

        // Store first and second part as new segment segments
        let pushed_segment = build_segment_from_part(segment, &first_part, None);
        output.push(pushed_segment);
        let pushed_segment = build_segment_from_part(segment, &second_part, Some(segment.properties.access_restrictions.as_ref().unwrap()[0].clone()));
        output.push(pushed_segment);
    }

    output.push(remaining_segment);

    output
}

/// Splits segments so that all segments contain at most one access restriction and
/// this optional access restriction applies to the whole segment
pub fn split_segments(segments: &mut Vec<Segment>)
{
    // Keeps track of segments that need to be replaced
    struct ReplacedSegment
    {
        index: usize,
        new_segments: Vec<Segment>
    }

    let mut replaced_segments: Vec<ReplacedSegment> = Vec::new();
    let mut next_connector_id: u32 = 1;

    for (index, segment) in segments.iter().enumerate() {
        let splitted_segment = split_segment(segment, &mut next_connector_id);
        if splitted_segment.len() != 1 {
            // We will have to replace this segment
            replaced_segments.push(ReplacedSegment {
                index,
                new_segments: splitted_segment
            });
        }
    }

    // Apply replacements in reverse order (so indices remain valid)
    for replaced_segment in replaced_segments.iter().rev() {
        segments.remove(replaced_segment.index);
        for new_segment in replaced_segment.new_segments.iter().rev() {
            segments.insert(replaced_segment.index, new_segment.clone());
        }
    }
}
