use crate::osm;
use crate::utils;

use std::fs::File;

use parquet::file::reader::{FileReader, SerializedFileReader};
use std::path::Path;
use parquet::record::Field;

use parquet::record::List;

#[derive(Debug, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64
}

#[derive(Debug)]
pub struct ConnectorRef {
    pub id: String,
    pub at: f64
}

#[derive(Debug)]
pub struct Connector {
    pub id: String,
    pub coordinate: Point
}

#[derive(Debug)]
pub struct Segment {
    pub name: String,
    pub points: Vec<Point>,
    pub connectors: Vec<ConnectorRef>,
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
            assert!(false, "Expected WKB to represent a Point");
            Point {
                lat: 0.0,
                lon: 0.0
            }
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
            assert!(false, "Expected WKB to represent a LineString");
            Vec::new()
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

pub fn import_overture_data(segment_path: &Path, connector_path: &Path) -> std::io::Result<Data> {
    let file = File::open(&segment_path)?;
    let reader = SerializedFileReader::new(file)?;

    let iter = reader.get_row_iter(None)?;

    let mut segments: Vec<Segment> = Vec::new();
    for row in iter {
        let mut primary_name = String::new();
        let mut geometry : Option<Vec<Point>> = None;
        let mut connectors: Option<Vec<ConnectorRef>> = None;
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
            } else if column.0 == "geometry"
            {
                let field : Field = column.1;
                if let Field::Bytes(byte_array) = field {
                    geometry = Some(process_geometry_vector(byte_array.data()));
                }
            } else if column.0 == "connectors" {
                let field : Field = column.1;
                if let Field::ListInternal(connectorref_list) = field {
                    connectors = Some(process_connector_refs(connectorref_list));
                }
            }
        }

        // TODO: check if we have geometry and connectors before pushing
        segments.push(Segment {
            name: primary_name,
            points: geometry.unwrap(),
            connectors: connectors.unwrap()
        });
    }

    let file = File::open(&connector_path)?;
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
            id: id,
            coordinate: coordinate.unwrap()
        });
    }

    Ok(Data { segments: segments, connectors: connectors })
}

#[derive(Debug)]
struct IndexedPoint {
    index: usize,
    point: Point
}

#[derive(Debug)]
struct ExportedRoad
{
    points: Vec<IndexedPoint>
}

fn get_point_for_connector(
    connector_ref: &ConnectorRef,
    all_connectors: &Vec<Connector>
) -> Option<Point> {
    all_connectors.iter()
        .find(|c| c.id == connector_ref.id)
        .map(|c| c.coordinate.clone())
}

fn get_connector_index_for_point(
    point: &Point,
    connector_refs: &Vec<ConnectorRef>,
    all_connectors: &Vec<Connector>
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
    all_connectors: &Vec<Connector>,
    next_index: &mut usize
) -> ExportedRoad {
    let mut exported_road = ExportedRoad {
        points: Vec::new()
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

fn export_roads(exported_roads: &Vec<ExportedRoad>, output_dir: &str)
{
    let mut total_way_nodes: u64 = 0;
    for way_index in 0..exported_roads.len() {
        total_way_nodes += (exported_roads[way_index].points.len() * 2) as u64;
    }

    unsafe {
        let ways = osm::osmway_new((exported_roads.len() * 2) as u64);
        let waynodes= osm::osmway_new(total_way_nodes);

        let mut total_way_node_index: u64 = 0;
        for way_index in 0..exported_roads.len() {
            let node_count: u64 = exported_roads[way_index].points.len() as u64;
            let offset_way_index: u64 = way_index as u64 * 2;
            osm::osmway_set_to_valhalla(ways, offset_way_index, offset_way_index + 1, 1, node_count as u64);
            osm::osmway_set_to_valhalla(ways, offset_way_index + 1, offset_way_index + 2, 1, node_count as u64);

            // Valhalla complains when road is only one way, so for now we export it twice, this is the first time...
            for (point_index, point) in exported_roads[way_index].points.iter().enumerate() {
                let (encoded_lat, encoded_lon) = utils::encode_lat_lon(point.point.lat, point.point.lon);
                let intersection: u64 = 1;
                
                osm::osmwaynode_set_to_valhalla(waynodes, total_way_node_index, offset_way_index, point_index as u64, point.index as u64, encoded_lon, encoded_lat, intersection);
                total_way_node_index += 1;
            }

            // ... and this is the second time.
            for (point_index, point) in exported_roads[way_index].points.iter().rev().enumerate() {
                let (encoded_lat, encoded_lon) = utils::encode_lat_lon(point.point.lat, point.point.lon);
                let intersection: u64 = 1;
                
                osm::osmwaynode_set_to_valhalla(waynodes, total_way_node_index, offset_way_index, point_index as u64, point.index as u64, encoded_lon, encoded_lat, intersection);
                total_way_node_index += 1;
            }
        }

        osm::write_osmways_to_file(ways, (exported_roads.len() * 2) as u64, &format!("{}/ways.bin", output_dir));
        osm::write_osmwaynodes_to_file(waynodes, total_way_nodes, &format!("{}/way_nodes.bin", output_dir));
    }
}

pub fn convert_overture_to_valhalla(input_dir : &str, output_dir: &str) -> std::io::Result<()>
{
    let segment_path = format!("{}/segment.parquet", input_dir);
    let connector_path = format!("{}/connector.parquet", input_dir);
    let overture_data = import_overture_data(Path::new(&segment_path), Path::new(&connector_path))?;

    let mut exported_roads: Vec<ExportedRoad> = Vec::new();
    let mut next_index = 1;
    for (index, segment) in overture_data.segments.iter().enumerate() {
        println!("Processing segment {} / {}: {}", index + 1, overture_data.segments.len(), segment.name);

        exported_roads.push(process_segment(segment, &overture_data.connectors, &mut next_index));
    }

    export_roads(&exported_roads, output_dir);

    Ok(())
}
