use std::fs::{write, File};
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::path::Path;
use parquet::record::Field;
use parquet::record::List;
use log::info;

use crate::valhalla_sys::{OsmWay, OsmWayNode};
use crate::restriction_splitter::split_streets;
//use crate::overture_types::{AccessRestriction, AccessWhen};

pub use overture_types::{AccessRestriction, AccessWhen};


#[derive(Debug, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64
}

#[derive(Debug, Clone)]
pub struct ConnectorRef {
    pub id: String,
    pub at: f64
}

#[derive(Debug)]
pub struct Connector {
    pub id: String,
    pub coordinate: Point
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub name: String,
    pub road_class: Option<String>,
    pub points: Vec<Point>,
    pub connectors: Vec<ConnectorRef>,
    pub access_restrictions: Vec<AccessRestriction>,
}

#[derive(Debug)]
pub struct Data {
    pub segments: Vec<Segment>,
    pub connectors: Vec<Connector>,
}

fn parse_point_wkb(wkb_data: &[u8]) -> Point {
    use geozero::wkb::Wkb;
    use geozero::ToGeo;
    use geo_types::Geometry;
    
    let wkb = Wkb(wkb_data);
    let geometry: Geometry<f64> = wkb.to_geo().unwrap();
    
    match geometry {
        Geometry::Point(point) => {
            Point {
                lat: point.y(),
                lon: point.x()
            }
        }
        _ => {
            panic!("Expected WKB to represent a Point");
        }   
    }
}

fn process_geometry_vector(wkb_data: &[u8]) -> Vec<Point> {
    use geozero::wkb::Wkb;
    use geozero::ToGeo;
    use geo_types::Geometry;
    
    let wkb = Wkb(wkb_data);
    let geometry: Geometry<f64> = wkb.to_geo().unwrap();
    
    match geometry {
        Geometry::LineString(line) => {
            let mut output : Vec<Point> = Vec::new();
            for point in line.points() {
                let added_point = Point {
                    lat: point.y(),
                    lon: point.x()
                };
                output.push(added_point);
            }
            output
        }
        _ => {
            panic!("Expected WKB to represent a LineString");
        }   
    }
}


fn process_connector_refs(connector_ref_list : List) -> Vec<ConnectorRef>
{
    let mut connector_refs = Vec::new();

    for connector_ref in connector_ref_list.elements() {
        if let Field::Group(group) = connector_ref {
            let mut connector_ref = ConnectorRef {
                id: String::new(),
                at: 0.0
            };
            for row in group.get_column_iter() {
                if row.0 == "connector_id" {
                    if let Field::Str(id) = row.1 {
                        connector_ref.id = id.to_string();
                    }
                } else if row.0 == "at" {
                    if let Field::Double(at) = row.1 {
                        connector_ref.at = *at;
                    }
                }
            }
            connector_refs.push(connector_ref);
        }
    }

    connector_refs
}

fn contains_access_restriction(restrictions: &Vec<AccessRestriction>, between: (f64, f64), when: &AccessWhen) -> bool
{
    let sigma = 0.0005;

    for restriction in restrictions {
        if !restriction.between.is_none() {
            let r_start = restriction.between.unwrap().0;
            let r_end = restriction.between.unwrap().1;

            let contains_start = (between.0 + sigma) > r_start;
            let contains_end = (between.1 - sigma) < r_end;

            let same_when = restriction.when.as_ref().unwrap() == when;

            if contains_start && contains_end && same_when {
                return true;
            }
        }
    }

    false
}

pub fn import_overture_data(segment_path: &Path, connector_path: &Path) -> std::io::Result<Data> {
    let file = File::open(segment_path)?;
    let reader = SerializedFileReader::new(file)?;

    let iter = reader.get_row_iter(None)?;

    let mut segments: Vec<Segment> = Vec::new();
    for row in iter {
        let mut primary_name = String::new();
        let mut road_class: Option<String> = None;
        let mut geometry : Option<Vec<Point>> = None;
        let mut connectors: Option<Vec<ConnectorRef>> = None;
        let mut access_restrictions: Vec<AccessRestriction> = Vec::new();
        for column in row?.into_columns() {
            if column.0 == "names" {
                if let Field::Group(group) = column.1 {
                    for field in group.get_column_iter() {
                        if field.0 == "primary" {
                            if let Field::Str(name) = field.1 {
                                primary_name = name.to_string();
                            }
                        }
                    }
                }
            } else if column.0 == "geometry" {
                let field : Field = column.1;
                if let Field::Bytes(byte_array) = field {
                    geometry = Some(process_geometry_vector(byte_array.data()));
                }
            } else if column.0 == "connectors" {
                let field : Field = column.1;
                if let Field::ListInternal(connectorref_list) = field {
                    connectors = Some(process_connector_refs(connectorref_list));
                }
            } else if column.0 == "class" {
                let field : Field = column.1;
                if let Field::Str(class) = field {
                    road_class = Some(class.to_string());
                }            
            } else if column.0 == "access_restrictions" {
                let restrictions = column.1;
                if let Field::ListInternal(restrictions_list) = restrictions {
                    for (idx, restriction) in restrictions_list.elements().iter().enumerate() {
                        if let Field::Group(restriction_group) = restriction {
                            let mut access_restriction = AccessRestriction {
                                access_type: String::new(),
                                when: None,
                                between: None,
                            };
                            let mut should_ignore = false;
                            for (key, value) in restriction_group.get_column_iter() {
                                if key == "access_type" {
                                    if let Field::Str(access_type) = value {
                                        let access_type_str = access_type.to_string();
                                        should_ignore = access_type_str == "allowed";
                                        access_restriction.access_type = access_type_str;
                                    }
                                } else if key == "when" {
                                    if let Field::Group(when_group) = value {
                                        let mut when = AccessWhen {
                                            vehicle: None,
                                            bicycle: None,
                                            pedestrian: None,
                                        };
                                        let mut when_set = false;
                                        for (when_key, when_value) in when_group.get_column_iter() {
                                            if when_key == "vehicle" {
                                                when.vehicle = Some(true);
                                                when_set = true;
                                            } else if when_key == "bicycle" {
                                                when.bicycle = Some(true);
                                                when_set = true;
                                            } else if when_key == "pedestrian" {
                                                when.pedestrian = Some(true);
                                                when_set = true;
                                            }
                                        }
                                        if when_set {
                                            access_restriction.when = Some(when);
                                        }
                                    }
                                } else if key == "between" {
                                    if let Field::ListInternal(between_list) = value {
                                        if between_list.len() != 2 {
                                            panic!("expected exactly 2 elements in 'between', got {}", between_list.len());
                                        }
                                        let start = if let Field::Double(d) = &between_list.elements()[0] {
                                            *d
                                        } else {
                                            panic!("expected double as first element in 'between'");
                                        };
                                        let end = if let Field::Double(d) = &between_list.elements()[1]
                                        {
                                            *d
                                        } else {
                                            panic!("expected double as second element in 'between'");
                                        };
                                        access_restriction.between = Some((start, end));
                                    }
                                }
                            }

                            let has_between = !access_restriction.between.is_none();
                            let has_when = access_restriction.when.as_ref().is_some();
                            let mut duplicate_restriction = false;
                            if has_between && has_when {
                                // TODO: this works because they are sorted from large to small, should check if this is always the case
                                duplicate_restriction = contains_access_restriction(&access_restrictions, access_restriction.between.unwrap(), &access_restriction.when.clone().unwrap());
                            }

                            if has_between && !should_ignore && has_when && !duplicate_restriction {
                                access_restrictions.push(access_restriction);
                            }
                        }
                    }
                }
            }

        }

        // TODO: check if we have geometry and connectors before pushing
        segments.push(Segment {
            name: primary_name,
            road_class,
            points: geometry.unwrap(),
            connectors: connectors.unwrap(),
            access_restrictions
        });
    }

    let file = File::open(connector_path)?;
    let reader = SerializedFileReader::new(file)?;

    let iter = reader.get_row_iter(None)?;

    let mut connectors: Vec<Connector> = Vec::new();
    for row in iter {
        let mut id = String::new();
        let mut coordinate: Option<Point> = None;
        for column in row?.into_columns() {
            if column.0 == "id" {
                if let Field::Str(id_str) = column.1 {
                    id = id_str.to_string();
                }
            } else if column.0 == "geometry" {
                if let Field::Bytes(byte_array) = column.1 {
                    coordinate = Some(parse_point_wkb(byte_array.data()));
                }
            }
        }


        connectors.push(Connector {
            id,
            coordinate: coordinate.unwrap()
        });
    }

    Ok(Data { segments, connectors })
}

#[derive(Debug)]
struct IndexedPoint {
    index: usize,
    point: Point
}

#[derive(Debug)]
struct Permissions {
    pedestrian_allowed: bool,
    auto_allowed: bool,
}


#[derive(Debug)]
struct ExportedRoad
{
    points: Vec<IndexedPoint>,
    permissions: Permissions
}

fn get_point_for_connector(
    connector_ref: &ConnectorRef,
    all_connectors: &[Connector]
) -> Option<Point> {
    all_connectors.iter()
        .find(|c| c.id == connector_ref.id)
        .map(|c| c.coordinate.clone())
}

fn get_connector_index_for_point(
    point: &Point,
    connector_refs: &[ConnectorRef],
    all_connectors: &[Connector]
) -> Option<usize>{
    for (connector_ref_index, connector_ref) in connector_refs.iter().enumerate() {
        let connector_point = get_point_for_connector(connector_ref, all_connectors);
        if connector_point.is_some() {
            let connector_point = connector_point.unwrap();
            if (point.lat - connector_point.lat).abs() < 1e-6 &&
               (point.lon - connector_point.lon).abs() < 1e-6 {
                return Some(connector_ref_index);
            }
        }
    }

    None
}

fn process_segment(
    segment: &Segment,
    all_connectors: &[Connector],
    next_index: &mut usize,
    permissions: Permissions
) -> ExportedRoad {
    let mut exported_road = ExportedRoad {
        points: Vec::new(),
        permissions
    };

    for point in segment.points.iter() {
        let connector_index = get_connector_index_for_point(point, &segment.connectors, all_connectors);
        if connector_index.is_some() {
            let connector_ref = &segment.connectors[connector_index.unwrap()];
            let connector_osm_index = all_connectors.iter()
                .position(|c| c.id == connector_ref.id)
                .expect("Connector not found in all connectors");
            exported_road.points.push(IndexedPoint {
                index: connector_osm_index,
                point: point.clone()
            });
        } else {
            // If no connector found, just use the point itself
            exported_road.points.push(IndexedPoint {
                index: *next_index,
                point: point.clone()
            });
            *next_index += 1;
        }
    }

    exported_road
}

fn export_roads(exported_roads: &[ExportedRoad], output_dir: &Path) -> std::io::Result<()> {
    let mut ways = Vec::new();
    let mut waynodes = Vec::new();

    for (way_index, exported_road) in exported_roads.iter().enumerate() {
        let node_count = exported_road.points.len() as u16;
        let offset_way_index: u64 = way_index as u64 * 2;
        let auto_allowed = exported_road.permissions.auto_allowed;
        let pedestrian_allowed = exported_road.permissions.pedestrian_allowed;
        ways.push(OsmWay::new(offset_way_index + 1, 1, node_count, auto_allowed, pedestrian_allowed));
        ways.push(OsmWay::new(offset_way_index + 2, 1, node_count, auto_allowed, pedestrian_allowed));

        // Valhalla complains when road is only one way, so for now we export it twice, this is the first time...
        for (point_index, point) in exported_roads[way_index].points.iter().enumerate() {
            // TODO: only make intersection if other way intersects
            let intersection: u64 = 1;

            waynodes.push(OsmWayNode::new(
                offset_way_index as u32,
                point_index as u32,
                point.index as u64,
                point.point.lon,
                point.point.lat,
                intersection as u32,
            ));
        }

        // ... and this is the second time.
        for (point_index, point) in exported_roads[way_index].points.iter().rev().enumerate() {
            // TODO: only make intersection if other way intersects
            let intersection: u64 = 1;

            waynodes.push(OsmWayNode::new(
                offset_way_index as u32,
                point_index as u32,
                point.index as u64,
                point.point.lon,
                point.point.lat,
                intersection as u32,
            ));
        }
    }

    write(output_dir.join("ways.bin"), OsmWay::slice_as_bytes(&ways))?;
    write(output_dir.join("way_nodes.bin"), OsmWayNode::slice_as_bytes(&waynodes))?;
    Ok(())
}

fn check_permissions(segment: &Segment) -> Permissions {
    let road_class = segment.road_class.as_deref().unwrap_or("null");
    let mut pedestrian_allowed = !matches!(
        road_class,
        "motorway" | "trunk" | "cycleway" | "standard_gauge"
    );

    let mut auto_allowed = !matches!(
        road_class,
        "null" | "steps" | "path" | "living_street" | "pedestrian" | "footway" | "cycleway" | "standard_gauge"
    );

    if segment.access_restrictions.len() > 1 {
        panic!("Too many access restrictions on segment: {segment:#?}");
    }

    if segment.access_restrictions.len() == 0 {
        return Permissions {
            pedestrian_allowed,
            auto_allowed,
        };
    }

    let access_restriction = segment.access_restrictions.first().unwrap();
    if access_restriction.access_type != "denied" && access_restriction.access_type != "designated" {
        panic!("Unknown access restriction type: {}", access_restriction.access_type);
    }

    let mut restrictions_set = 0;
    if let Some(when) = &access_restriction.when {
        if when.vehicle.is_some() {
            auto_allowed = false;
            restrictions_set += 1;
        }
        if when.bicycle.is_some() {
            // TODO: bicycle handling
            restrictions_set += 1;
        }
        if when.pedestrian.is_some() {
            pedestrian_allowed = false;
            restrictions_set += 1;
        }
    } else {
        panic!("Access restriction does not have 'when' set: {access_restriction:#?}");
    }

    if restrictions_set != 1 {
        panic!("Expected exactly one access restriction to be set, got {restrictions_set}");
    }

    Permissions {
        pedestrian_allowed,
        auto_allowed,
    }
}

pub fn convert_overture_to_valhalla(input_dir : &Path, output_dir: &Path) -> std::io::Result<()>
{
    let segment_path = input_dir.join("segment.parquet");
    let connector_path = input_dir.join("connector.parquet");
    let mut overture_data = import_overture_data(&segment_path, &connector_path)?;

    split_streets(&mut overture_data);

    let mut exported_roads: Vec<ExportedRoad> = Vec::new();
    let mut next_index = 1;
    for (index, segment) in overture_data.segments.iter().enumerate() {
        let road_class: &str = segment.road_class.as_deref().unwrap_or("null");

        info!("Processing segment {} / {}: {} ({})", index + 1, overture_data.segments.len(), segment.name, road_class);
        let permissions = check_permissions(segment);

        if !permissions.auto_allowed && !permissions.pedestrian_allowed {
            info!("- Ignored");
            continue;
        } else {
            if permissions.auto_allowed {
                info!("- Auto allowed");
            }
            if permissions.pedestrian_allowed {
                info!("- Pedestrian allowed");
            }
        }

        exported_roads.push(process_segment(segment, &overture_data.connectors, &mut next_index, permissions));
    }

    export_roads(&exported_roads, output_dir)?;

    Ok(())
}
