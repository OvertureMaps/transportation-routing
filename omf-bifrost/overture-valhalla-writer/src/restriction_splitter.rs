use crate::writer::Data;
use crate::writer::{ConnectorRef, Point, Segment, AccessRestriction, AccessWhen};

static mut next_id: u32 = 1;
fn get_next_id() -> u32
{
    unsafe {
        let id = next_id;
        next_id += 1;
        id
    }
}

#[derive(Debug)]
struct SplitStreetPart
{
    pub points: Vec<Point>,
    pub connector_refs: Vec<ConnectorRef>, // already fixed
}

fn build_street_from_part(org_street: &Segment, part: &SplitStreetPart, access_restriction: Option<AccessRestriction>) -> Segment
{
    let mut access_restrictions: Vec<AccessRestriction> = Vec::new();
    if let Some(restriction) = access_restriction {
        access_restrictions.push(restriction);
    }

    Segment {
        name: org_street.name.clone(),
        road_class: org_street.road_class.clone(),
        access_restrictions,
        points: part.points.clone(),
        connectors: part.connector_refs.clone()
        
    }
}

fn get_all_data_before_offset(street: &Segment, total_length: f64, linear_offset: f64) -> SplitStreetPart
{
    let mut output = SplitStreetPart {
        points: Vec::new(),
        connector_refs: Vec::new()
    };

    if (linear_offset < 0.0) || (linear_offset > 1.0) {
        panic!("Linear offset out of range: {}", linear_offset);
    }

    // always put first one
    output.points.push(street.points[0].clone());

    let mut current_length = 0.0;
    let max_length = linear_offset * total_length;
    for i in 1..street.points.len() {
        let p1 = &street.points[i - 1];
        let p2 = &street.points[i];
        let segment_length = ((p2.lat - p1.lat).powi(2) + (p2.lon - p1.lon).powi(2)).sqrt();
        current_length += segment_length;

        if current_length < max_length {
            output.points.push(p2.clone());
        } else {
            // add interpolated point
            let excess_length = current_length - max_length;
            let t = (segment_length - excess_length) / segment_length;
            let point_lat = &street.points[i - 1].lat * (1.0 - t) + &street.points[i].lat * t;
            let point_lon = &street.points[i - 1].lon * (1.0 - t) + &street.points[i].lon * t;
            output.points.push(Point { lat: point_lat, lon: point_lon });
            break;
        }
    }

    let connector_correction = 1.0 - linear_offset;
    for connector in street.connectors.iter() {
        if connector.at < linear_offset {
            let mut new_connector = connector.clone();
            new_connector.at *= connector_correction;
            output.connector_refs.push(new_connector);
        }
    }

    output
}

fn get_all_data_between_offsets(street: &Segment, total_length: f64, start_linear_offset: f64, end_linear_offset: f64) -> SplitStreetPart
{
    if (start_linear_offset < 0.0) || (start_linear_offset > 1.0) {
        panic!("Start linear offset out of range: {}", start_linear_offset);
    }
    if (end_linear_offset < 0.0) || (end_linear_offset > 1.0) {
        panic!("End linear offset out of range: {}", end_linear_offset);
    }
    if start_linear_offset >= end_linear_offset {
        panic!("Start linear offset must be less than end linear offset");
    }

    let mut output = SplitStreetPart {
        points: Vec::new(),
        connector_refs: Vec::new()
    };

    let start_length = start_linear_offset * total_length;
    let end_length = end_linear_offset * total_length;

    let mut current_length = 0.0;
    let mut started = false;
    for i in 1..street.points.len() {
        let p1 = &street.points[i - 1];
        let p2 = &street.points[i];
        let segment_length = ((p2.lat - p1.lat).powi(2) + (p2.lon - p1.lon).powi(2)).sqrt();
        current_length += segment_length;

        if !started && (current_length >= start_length) {
            // add interpolated point
            let excess_length = current_length - start_length;
            let t = (segment_length - excess_length) / segment_length;
            let point_lat = &street.points[i - 1].lat * (1.0 - t) + &street.points[i].lat * t;
            let point_lon = &street.points[i - 1].lon * (1.0 - t) + &street.points[i].lon * t;
            output.points.push(Point { lat: point_lat, lon: point_lon });
            started = true;
        }

        if started {
            if current_length < end_length {
                output.points.push(p2.clone());
            } else {
                // add interpolated point
                let excess_length = current_length - end_length;
                let t = (segment_length - excess_length) / segment_length;
                let point_lat = &street.points[i - 1].lat * (1.0 - t) + &street.points[i].lat * t;
                let point_lon = &street.points[i - 1].lon * (1.0 - t) + &street.points[i].lon * t;
                output.points.push(Point { lat: point_lat, lon: point_lon });
                break;
            }
        }
    }

    let connector_correction = 1.0 / (end_linear_offset - start_linear_offset);
    for connector in street.connectors.iter() {
        if (connector.at >= start_linear_offset) && (connector.at <= end_linear_offset) {
            let mut new_connector = connector.clone();
            new_connector.at = (new_connector.at - start_linear_offset) * connector_correction;
            output.connector_refs.push(new_connector);
        }
    }

    output
}

fn fixup_remaining_street(street: &mut Segment, start_linear_offset: f64)
{
    // TODO: special case if start_linear_offset is 0.0, or close to it

    // fix connector refs: remove those before start_linear_offset, and adjust the others
    let connector_correction = 1.0 - start_linear_offset;
    let mut new_connectors: Vec<ConnectorRef> = Vec::new();
    for connector in street.connectors.iter() {
        if connector.at >= start_linear_offset {
            let mut new_connector = connector.clone();
            new_connector.at = (new_connector.at - start_linear_offset) * connector_correction;
            new_connectors.push(new_connector);
        }
    }
    street.connectors = new_connectors;

    // fix points: remove those before start_linear_offset and introduce interpolated point at start_linear_offset
    let total_length = calc_street_length(&street);
    let start_length = start_linear_offset * total_length;
    let mut new_points: Vec<Point> = Vec::new();
    let mut current_length = 0.0;
    let mut started = false;
    for i in 1..street.points.len() {
        let p1 = &street.points[i - 1];
        let p2 = &street.points[i];
        let segment_length = ((p2.lat - p1.lat).powi(2) + (p2.lon - p1.lon).powi(2)).sqrt();
        current_length += segment_length;
        if !started && (current_length >= start_length) {
            // add interpolated point
            let excess_length = current_length - start_length;
            let t = (segment_length - excess_length) / segment_length;
            let point_lat = &street.points[i - 1].lat * (1.0 - t) + &street.points[i].lat * t;
            let point_lon = &street.points[i - 1].lon * (1.0 - t) + &street.points[i].lon * t;
            new_points.push(Point { lat: point_lat, lon: point_lon });
            started = true;
        }
        if started {
            new_points.push(p2.clone());
        }
    }

    street.points = new_points;

    // fixup access restrictions: adjust their between values
    let mut new_access_restrictions: Vec<AccessRestriction> = Vec::new();
    for restriction in &street.access_restrictions {
        if !restriction.between.is_none() {
            let r_start = restriction.between.unwrap().0;
            let r_end = restriction.between.unwrap().1;

            if r_end <= start_linear_offset {
                // skip this restriction
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
    street.access_restrictions = new_access_restrictions;

}

fn calc_street_length(street: &Segment) -> f64
{
    let mut total_length = 0.0;
    for i in 1..street.points.len() {
        let p1 = &street.points[i - 1];
        let p2 = &street.points[i];
        let segment_length = ((p2.lat - p1.lat).powi(2) + (p2.lon - p1.lon).powi(2)).sqrt();
        total_length += segment_length;
    }
    total_length
}

fn split_street(street: &Segment) -> Vec<Segment>
{
    let mut remaining_street = street.clone();

    let mut output: Vec<Segment> = Vec::new();

    while remaining_street.access_restrictions.len() > 0 {
        let length = calc_street_length(&remaining_street);

        let first_part = SplitStreetPart {
            points: Vec::new(),
            connector_refs: Vec::new()
        };

        //let first_part_end_length = street.access_restrictions[0].between.as_ref().unwrap().0 * length;

        let mut first_part = get_all_data_before_offset(&street, length, street.access_restrictions[0].between.as_ref().unwrap().0);
        let mut second_part = get_all_data_between_offsets(&street, length, street.access_restrictions[0].between.as_ref().unwrap().0, street.access_restrictions[0].between.as_ref().unwrap().1);

        remaining_street.access_restrictions.remove(0);
        fixup_remaining_street(&mut remaining_street, street.access_restrictions[0].between.as_ref().unwrap().1);
        

        let connector_id = format!("split-{}-{}", street.name, get_next_id()).replace(" ", "-");
        let connector = ConnectorRef {
            id: connector_id.clone(),
            at: 1.0
        };
        first_part.connector_refs.push(connector.clone());
        let connector = ConnectorRef {
            id: connector_id,
            at: 0.0
        };
        second_part.connector_refs.insert(0, connector.clone());

        let connector_id = format!("split-{}-{}", street.name, get_next_id()).replace(" ", "-");
        let connector = ConnectorRef {
            id: connector_id.clone(),
            at: 1.0
        };
        second_part.connector_refs.push(connector.clone());
        let connector = ConnectorRef {
            id: connector_id,
            at: 0.0
        };
        remaining_street.connectors.insert(0, connector.clone());

        output.push(build_street_from_part(&street, &first_part, None));
        output.push(build_street_from_part(&street, &second_part, Some(street.access_restrictions[0].clone())));
    }

    output.push(remaining_street);

    output
}

struct ReplacedStreet
{
    index: usize,
    new_streets: Vec<Segment>
}

pub fn split_streets(streets: &mut Data)
{
    let mut replaced_streets: Vec<ReplacedStreet> = Vec::new();

    for (index, street) in streets.segments.iter().enumerate() {
        let splitted_street = split_street(street);
        if splitted_street.len() != 1 {
            replaced_streets.push(ReplacedStreet {
                index,
                new_streets: splitted_street
            });
        }
    }

    for replaced_street in replaced_streets.iter().rev() {
        streets.segments.remove(replaced_street.index);
        for new_street in replaced_street.new_streets.iter().rev() {
            streets.segments.insert(replaced_street.index, new_street.clone());
        }
    }
}