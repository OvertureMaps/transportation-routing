use std::path::Path;

fn encode_lat_lon(decoded_lat: f64, decoded_lon: f64) -> (u32, u32) {
    let encoded_lat = ((decoded_lat + 90.0) * 10f64.powi(7)) as u32;
    let encoded_lon = ((decoded_lon + 180.0) * 10f64.powi(7)) as u32;
    (encoded_lat, encoded_lon)
}

#[expect(non_camel_case_types, non_upper_case_globals)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[repr(transparent)]
pub struct OsmNode(ffi::OSMNode);

impl OsmNode {
    pub fn as_ptr(&self) -> *const ffi::OSMNode {
        &self.0 as *const ffi::OSMNode
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<ffi::OSMNode>();
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }
}

#[repr(transparent)]
pub struct OsmWayNode(ffi::OSMWayNode);

impl OsmWayNode {
    pub fn default() -> OsmWayNode {
        OsmWayNode(ffi::OSMWayNode::default())
    }

    pub fn as_ptr(&self) -> *const ffi::OSMWayNode {
        &self.0 as *const ffi::OSMWayNode
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<ffi::OSMWayNode>();
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }
}

#[repr(transparent)]
pub struct OsmWay(ffi::OSMWay);

impl OsmWay {
    pub fn default() -> OsmWay {
        OsmWay(ffi::OSMWay::default())
    }

    pub fn as_ptr(&self) -> *const ffi::OSMWay {
        &self.0 as *const ffi::OSMWay
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<ffi::OSMWay>();
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    pub fn simple_valhalla(osmid:u64, name_index:u32, nodecount:u16) -> Self
    {
        let mut way = OsmWay::default();
        way.0.osmwayid_ = osmid;
        way.0.name_index_ = name_index;
        way.0.nodecount_ = nodecount;

        // TODO: could also be 0, ("kPavedSmooth")? See "graphconstants.h" in Valhalla
        way.0.set_surface_(3); // kCompacted

        // TODO: not all countries drive on the right
        way.0.set_drive_on_right_(1);

        // TODO: could also be 6, ("kResidential") or 0 ("kMotorway")? See "graphconstants.h" in Valhalla
        way.0.set_road_class_(7); // kServiceOther

        // TODO: might want to use 0 here ("kRoad)?
        way.0.set_use_(25); // "kFootway" ("enum class Use : uint8_t")

        // TODO: Can we leave this 0 for Overture->Valhalla conversion?
        way.0.set_has_user_tags_(0);

        // TODO: Have a second look at this, does this mean pedestrian-only?
        way.0.set_pedestrian_forward_(1);
        way.0.set_pedestrian_backward_(1);

        // TODO: get this from Overture data
        way.0.speed_ = 25; // 25 km/h

        way
    }    
}

pub trait OsmWayVecExt {
    fn write_to_file(&self, path: &Path) -> std::io::Result<()>;
}

impl OsmWayVecExt for Vec<OsmWay> {
    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        let ptr = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<OsmWay>() * self.len();
        let bytes = unsafe { std::slice::from_raw_parts(ptr, size) };
        std::fs::write(path, bytes)
    }
}

impl OsmNode {
}

pub trait OsmNodeVecExt {
    fn write_to_file(&self, path: &Path) -> std::io::Result<()>;
}

impl OsmNodeVecExt for Vec<OsmNode> {
    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        let ptr = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<OsmNode>() * self.len();
        let bytes = unsafe { std::slice::from_raw_parts(ptr, size) };
        std::fs::write(path, bytes)
    }
}

impl OsmWayNode {
    pub fn simple_valhalla(way_index : u32, way_shape_node_index : u32, osmid: u64, lng: f64, lat: f64, intersection: u32) -> Self
    {
        let mut waynode = OsmWayNode::default();
        waynode.0.way_index = way_index;
        waynode.0.way_shape_node_index = way_shape_node_index;

        waynode.0.node.osmid_ = osmid;

        let (lat7, lng7) = encode_lat_lon(lat, lng);
        waynode.0.node.lng7_ = lng7;
        waynode.0.node.lat7_ = lat7;
        waynode.0.node.set_intersection_(intersection);

        // TODO: could also be 4095 ("kAllAccess")? See "graphconstants.h" in Valhalla
        // TODO: get from Overture data
        waynode.0.node.set_access_(2047);

        waynode
    }
}

pub trait OsmWayNodeVecExt {
    fn write_to_file(&self, path: &Path) -> std::io::Result<()>;
}

impl OsmWayNodeVecExt for Vec<OsmWayNode> {
    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        let ptr = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<OsmWayNode>() * self.len();
        let bytes = unsafe { std::slice::from_raw_parts(ptr, size) };
        std::fs::write(path, bytes)
    }
}
