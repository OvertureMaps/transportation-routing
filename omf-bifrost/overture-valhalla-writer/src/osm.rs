use std::ffi::CString;
use std::ffi::c_void;

#[link(name = "c_lib")]
#[allow(dead_code)]
unsafe extern "C" {
    pub fn osmnode_new(count : u64) -> *mut c_void;
    pub fn osmnode_clear(s : *mut c_void, count : u64);
    pub fn osmnode_import(fileName : *const i8, buffer: *mut *mut c_void, count: *mut u64);
    pub fn osmnode_export(s : *mut c_void, count : u64, fileName : *const i8) -> u64;
    pub fn osmnode_free(s : *mut c_void);
    pub fn osmnode_dump(s : *const c_void, index : u64);
    pub fn osmnode_set_to_valhalla(s : *mut c_void, index : u64, osmid : u64, lng7 : u64, lat7 : u64, intersection : u64);

    pub fn osmwaynode_new(count : u64) -> *mut c_void;
    pub fn osmwaynode_clear(s : *mut c_void, count : u64);
    pub fn osmwaynode_import(fileName : *const i8, buffer: *mut *mut c_void, count: *mut u64);
    pub fn osmwaynode_export(s : *mut c_void, count : u64, fileName : *const i8);
    pub fn osmwaynode_free(s : *mut c_void);
    pub fn osmwaynode_dump(s : *const c_void, index : u64);
    pub fn osmwaynode_set_to_valhalla(s : *mut c_void, index : u64, way_index : u64, way_shape_node_index : u64, osmid : u64, lng7 : u64, lat7 : u64, intersection : u64);

    pub fn osmway_new(count : u64) -> *mut c_void;
    pub fn osmway_clear(s : *mut c_void, count : u64);
    pub fn osmway_import(fileName : *const i8, buffer: *mut *mut c_void, count: *mut u64);
    pub fn osmway_export(s : *mut c_void, count : u64, fileName : *const i8);
    pub fn osmway_free(s : *mut c_void);
    pub fn osmway_set_to_valhalla(s : *mut c_void, index : u64, osmid : u64, name_index : u64, nodecount : u64);

    pub fn osmwaynode_set_node(s : *mut c_void, index : u64, value : *const c_void);
    pub fn osmwaynode_get_node(s : *const c_void, index : u64, value : *mut c_void);
}

pub fn write_osmnodes_to_file(s : *mut c_void, count : u64, file_name : &str)
{
    unsafe {
        let c_str = CString::new(file_name).expect("Failed to create CString");
        osmnode_export(s, count, c_str.as_ptr());
    }
}

pub fn write_osmwaynodes_to_file(s : *mut c_void, count : u64, file_name : &str)
{
    unsafe {
        let c_str = CString::new(file_name).expect("Failed to create CString");
        osmwaynode_export(s, count, c_str.as_ptr());
    }
}

pub fn write_osmways_to_file(s : *mut c_void, count : u64, file_name : &str)
{
    unsafe {
        let c_str = CString::new(file_name).expect("Failed to create CString");
        osmway_export(s, count, c_str.as_ptr());
    }
}
