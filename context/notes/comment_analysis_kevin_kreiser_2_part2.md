# Analysis of Kevin Kreiser's Second Comment (Part 2)

## The Comment (Second Part)

> concretely it means you need to write a transcoder from overture into these fixed size structures (`OSMData`, `OSMWay`, `OSMNode`, `OSMWayNode`, and a few others etc) that `ConstructEdges` expects. the only other thing you need is a means of getting the admin boundary information into a spatialite db but that might be somewhat easy with something like duckdb and its many hookins for standardized spatial formats.
> 
> tldr; all you really need to know is what is in OSMData, what it looks like and how to fill it out from overture rather than from osm. thankfully those structures are simple so it should be fairly easy to grok. its good to know about the rest of the tile building pipeline but for the purposes of overture the bulk of the focus should be on parsing into these intermediate structures

## Key Points

1. **Transcoder Approach**: Write a transcoder from Overture data to Valhalla's fixed-size structures (`OSMData`, `OSMWay`, `OSMNode`, `OSMWayNode`).

2. **Admin Boundaries**: Need a way to get admin boundary information into a spatialite DB, possibly using DuckDB.

3. **Focus Area**: Understand what's in `OSMData` and how to fill it out from Overture data.

## Relevant Valhalla Code

### OSMData Structure

The `OSMData` structure is the key target for our transcoder. Let's examine its definition:

```cpp
// From valhalla/mjolnir/osmdata.h
struct OSMData {
  // Stores all the ways
  std::unordered_map<uint64_t, OSMWay> ways;
  
  // Stores all the nodes
  std::unordered_map<uint64_t, OSMNode> nodes;
  
  // Stores all the node ids used by ways
  std::unordered_set<uint64_t> way_nodes;
  
  // Stores all the relations
  std::vector<OSMRelation> relations;
  
  // Stores all the node restrictions
  std::multimap<uint64_t, OSMRestriction> node_restrictions;
  
  // Stores all the way restrictions
  std::multimap<uint64_t, OSMRestriction> way_restrictions;
  
  // Stores all the access restrictions
  std::multimap<uint64_t, OSMAccess> access_restrictions;
  
  // ... more fields ...
};
```

### OSMWay Structure

The `OSMWay` structure represents a way (road segment):

```cpp
// From valhalla/mjolnir/osmway.h
class OSMWay {
public:
  OSMWay();
  
  // Set way attributes
  void set_node_ids(const std::vector<uint64_t>& node_ids);
  void set_name(const std::string& name);
  void set_road_class(const RoadClass roadclass);
  void set_speed(const uint8_t speed);
  void set_backward_speed(const uint8_t backward_speed);
  void set_max_speed(const uint8_t max_speed);
  void set_truck_speed(const uint8_t truck_speed);
  void set_bike_speed(const uint8_t bike_speed);
  void set_ferry(const bool ferry);
  void set_use(const Use use);
  void set_toll(const bool toll);
  void set_destination_only(const bool destination_only);
  void set_tunnel(const bool tunnel);
  void set_bridge(const bool bridge);
  void set_roundabout(const bool roundabout);
  void set_internal(const bool internal);
  void set_no_thru_traffic(const bool no_thru_traffic);
  void set_oneway(const bool oneway);
  void set_oneway_reverse(const bool oneway_reverse);
  void set_roundabout_exit(const bool roundabout_exit);
  void set_bicycling(const bool bicycling);
  void set_bike_network(const uint8_t bike_network);
  void set_truck_route(const bool truck_route);
  void set_driveway(const bool driveway);
  void set_service(const bool service);
  void set_sidewalk_left(const bool sidewalk_left);
  void set_sidewalk_right(const bool sidewalk_right);
  void set_drive_on_right(const bool drive_on_right);
  void set_surface(const Surface surface);
  void set_wheelchair(const bool wheelchair);
  void set_wheelchair_tag(const bool wheelchair_tag);
  void set_pedestrian(const bool pedestrian);
  void set_auto_tag(const bool auto_tag);
  void set_bus_tag(const bool bus_tag);
  void set_truck_tag(const bool truck_tag);
  void set_bike_tag(const bool bike_tag);
  void set_moped_tag(const bool moped_tag);
  void set_motorcycle_tag(const bool motorcycle_tag);
  void set_hov_tag(const bool hov_tag);
  void set_taxi_tag(const bool taxi_tag);
  void set_motorroad_tag(const bool motorroad_tag);
  void set_seasonal(const bool seasonal);
  void set_ski_trail(const bool ski_trail);
  void set_ski_trail_tag(const bool ski_trail_tag);
  void set_names(const std::vector<std::string>& names);
  void set_ref(const std::string& ref);
  void set_int_ref(const std::string& int_ref);
  void set_direction(const std::string& direction);
  void set_int_direction(const std::string& int_direction);
  void set_ref_direction(const std::string& ref_direction);
  void set_int_ref_direction(const std::string& int_ref_direction);
  void set_destination(const std::string& destination);
  void set_destination_ref(const std::string& destination_ref);
  void set_destination_ref_to(const std::string& destination_ref_to);
  void set_destination_street(const std::string& destination_street);
  void set_destination_street_to(const std::string& destination_street_to);
  void set_junction_ref(const std::string& junction_ref);
  void set_turn_channel(const bool turn_channel);
  void set_turn_lanes(const std::string& turn_lanes);
  void set_turn_lanes_forward(const std::string& turn_lanes_forward);
  void set_turn_lanes_backward(const std::string& turn_lanes_backward);
  void set_lanes(const uint32_t lanes);
  void set_forward_lanes(const uint32_t forward_lanes);
  void set_backward_lanes(const uint32_t backward_lanes);
  void set_sac_scale(const uint32_t sac_scale);
  void set_layer(const int8_t layer);
  void set_level(const std::string& level);
  void set_cycle_lane(const CycleLane cycle_lane);
  void set_cycle_lane_right(const CycleLane cycle_lane_right);
  void set_cycle_lane_left(const CycleLane cycle_lane_left);
  void set_cycle_lane_right_opposite(const bool cycle_lane_right_opposite);
  void set_cycle_lane_left_opposite(const bool cycle_lane_left_opposite);
  void set_shoulder(const bool shoulder);
  void set_shoulder_right(const bool shoulder_right);
  void set_shoulder_left(const bool shoulder_left);
  void set_parking_left(const bool parking_left);
  void set_parking_right(const bool parking_right);
  void set_way_id(const uint64_t way_id);
  void set_restrictions(const uint32_t count);
  void set_access_restrictions(const uint32_t count);
  void set_destination_only_from_restrictions(const bool destination_only_from_restrictions);
  void set_track(const uint8_t track);
  void set_country_crossing(const bool country_crossing);
  void set_private_access(const bool private_access);
  void set_cash_only_toll(const bool cash_only_toll);
  void set_hov_type(const HOVType hov_type);
  void set_hov_lanes(const uint8_t hov_lanes);
  void set_classification(const RoadClass classification);
  void set_link(const bool link);
  void set_attributes(const uint32_t attributes);
  void set_edge_index(const uint32_t edge_index);
  void set_speed_type(const SpeedType speed_type);
  void set_tagged_speed(const bool tagged_speed);
  void set_forward_tagged_speed(const bool forward_tagged_speed);
  void set_backward_tagged_speed(const bool backward_tagged_speed);
  void set_default_speed(const bool default_speed);
  void set_has_user_tags(const bool has_user_tags);
  void set_has_pronunciation(const bool has_pronunciation);
  void set_pronunciation_tags(const std::vector<std::string>& pronunciation_tags);
  void set_has_destination_pronunciation(const bool has_destination_pronunciation);
  void set_destination_pronunciation_tags(const std::vector<std::string>& destination_pronunciation_tags);
  void set_has_ref_pronunciation(const bool has_ref_pronunciation);
  void set_ref_pronunciation_tags(const std::vector<std::string>& ref_pronunciation_tags);
  void set_has_int_ref_pronunciation(const bool has_int_ref_pronunciation);
  void set_int_ref_pronunciation_tags(const std::vector<std::string>& int_ref_pronunciation_tags);
  void set_has_junction_ref_pronunciation(const bool has_junction_ref_pronunciation);
  void set_junction_ref_pronunciation_tags(const std::vector<std::string>& junction_ref_pronunciation_tags);
  void set_has_direction_pronunciation(const bool has_direction_pronunciation);
  void set_direction_pronunciation_tags(const std::vector<std::string>& direction_pronunciation_tags);
  void set_has_int_direction_pronunciation(const bool has_int_direction_pronunciation);
  void set_int_direction_pronunciation_tags(const std::vector<std::string>& int_direction_pronunciation_tags);
  void set_has_ref_direction_pronunciation(const bool has_ref_direction_pronunciation);
  void set_ref_direction_pronunciation_tags(const std::vector<std::string>& ref_direction_pronunciation_tags);
  void set_has_int_ref_direction_pronunciation(const bool has_int_ref_direction_pronunciation);
  void set_int_ref_direction_pronunciation_tags(const std::vector<std::string>& int_ref_direction_pronunciation_tags);
  void set_has_destination_pronunciation(const bool has_destination_pronunciation);
  void set_destination_pronunciation_tags(const std::vector<std::string>& destination_pronunciation_tags);
  void set_has_destination_ref_pronunciation(const bool has_destination_ref_pronunciation);
  void set_destination_ref_pronunciation_tags(const std::vector<std::string>& destination_ref_pronunciation_tags);
  void set_has_destination_ref_to_pronunciation(const bool has_destination_ref_to_pronunciation);
  void set_destination_ref_to_pronunciation_tags(const std::vector<std::string>& destination_ref_to_pronunciation_tags);
  void set_has_destination_street_pronunciation(const bool has_destination_street_pronunciation);
  void set_destination_street_pronunciation_tags(const std::vector<std::string>& destination_street_pronunciation_tags);
  void set_has_destination_street_to_pronunciation(const bool has_destination_street_to_pronunciation);
  void set_destination_street_to_pronunciation_tags(const std::vector<std::string>& destination_street_to_pronunciation_tags);
  
  // Get way attributes
  const std::vector<uint64_t>& node_ids() const;
  const std::string& name() const;
  RoadClass road_class() const;
  uint8_t speed() const;
  uint8_t backward_speed() const;
  uint8_t max_speed() const;
  uint8_t truck_speed() const;
  uint8_t bike_speed() const;
  bool ferry() const;
  Use use() const;
  bool toll() const;
  bool destination_only() const;
  bool tunnel() const;
  bool bridge() const;
  bool roundabout() const;
  bool internal() const;
  bool no_thru_traffic() const;
  bool oneway() const;
  bool oneway_reverse() const;
  bool roundabout_exit() const;
  bool bicycling() const;
  uint8_t bike_network() const;
  bool truck_route() const;
  bool driveway() const;
  bool service() const;
  bool sidewalk_left() const;
  bool sidewalk_right() const;
  bool drive_on_right() const;
  Surface surface() const;
  bool wheelchair() const;
  bool wheelchair_tag() const;
  bool pedestrian() const;
  bool auto_tag() const;
  bool bus_tag() const;
  bool truck_tag() const;
  bool bike_tag() const;
  bool moped_tag() const;
  bool motorcycle_tag() const;
  bool hov_tag() const;
  bool taxi_tag() const;
  bool motorroad_tag() const;
  bool seasonal() const;
  bool ski_trail() const;
  bool ski_trail_tag() const;
  const std::vector<std::string>& names() const;
  const std::string& ref() const;
  const std::string& int_ref() const;
  const std::string& direction() const;
  const std::string& int_direction() const;
  const std::string& ref_direction() const;
  const std::string& int_ref_direction() const;
  const std::string& destination() const;
  const std::string& destination_ref() const;
  const std::string& destination_ref_to() const;
  const std::string& destination_street() const;
  const std::string& destination_street_to() const;
  const std::string& junction_ref() const;
  bool turn_channel() const;
  const std::string& turn_lanes() const;
  const std::string& turn_lanes_forward() const;
  const std::string& turn_lanes_backward() const;
  uint32_t lanes() const;
  uint32_t forward_lanes() const;
  uint32_t backward_lanes() const;
  uint32_t sac_scale() const;
  int8_t layer() const;
  const std::string& level() const;
  CycleLane cycle_lane() const;
  CycleLane cycle_lane_right() const;
  CycleLane cycle_lane_left() const;
  bool cycle_lane_right_opposite() const;
  bool cycle_lane_left_opposite() const;
  bool shoulder() const;
  bool shoulder_right() const;
  bool shoulder_left() const;
  bool parking_left() const;
  bool parking_right() const;
  uint64_t way_id() const;
  uint32_t restrictions() const;
  uint32_t access_restrictions() const;
  bool destination_only_from_restrictions() const;
  uint8_t track() const;
  bool country_crossing() const;
  bool private_access() const;
  bool cash_only_toll() const;
  HOVType hov_type() const;
  uint8_t hov_lanes() const;
  RoadClass classification() const;
  bool link() const;
  uint32_t attributes() const;
  uint32_t edge_index() const;
  SpeedType speed_type() const;
  bool tagged_speed() const;
  bool forward_tagged_speed() const;
  bool backward_tagged_speed() const;
  bool default_speed() const;
  bool has_user_tags() const;
  bool has_pronunciation() const;
  const std::vector<std::string>& pronunciation_tags() const;
  bool has_destination_pronunciation() const;
  const std::vector<std::string>& destination_pronunciation_tags() const;
  bool has_ref_pronunciation() const;
  const std::vector<std::string>& ref_pronunciation_tags() const;
  bool has_int_ref_pronunciation() const;
  const std::vector<std::string>& int_ref_pronunciation_tags() const;
  bool has_junction_ref_pronunciation() const;
  const std::vector<std::string>& junction_ref_pronunciation_tags() const;
  bool has_direction_pronunciation() const;
  const std::vector<std::string>& direction_pronunciation_tags() const;
  bool has_int_direction_pronunciation() const;
  const std::vector<std::string>& int_direction_pronunciation_tags() const;
  bool has_ref_direction_pronunciation() const;
  const std::vector<std::string>& ref_direction_pronunciation_tags() const;
  bool has_int_ref_direction_pronunciation() const;
  const std::vector<std::string>& int_ref_direction_pronunciation_tags() const;
  bool has_destination_pronunciation() const;
  const std::vector<std::string>& destination_pronunciation_tags() const;
  bool has_destination_ref_pronunciation() const;
  const std::vector<std::string>& destination_ref_pronunciation_tags() const;
  bool has_destination_ref_to_pronunciation() const;
  const std::vector<std::string>& destination_ref_to_pronunciation_tags() const;
  bool has_destination_street_pronunciation() const;
  const std::vector<std::string>& destination_street_pronunciation_tags() const;
  bool has_destination_street_to_pronunciation() const;
  const std::vector<std::string>& destination_street_to_pronunciation_tags() const;
  
private:
  std::vector<uint64_t> node_ids_;
  std::string name_;
  std::string name_en_;
  std::string alt_name_;
  std::string official_name_;
  std::vector<std::string> name_pronunciation_tags_;
  std::vector<std::string> alt_name_pronunciation_tags_;
  std::vector<std::string> official_name_pronunciation_tags_;
  std::vector<std::string> names_;
  std::string ref_;
  std::string int_ref_;
  std::string direction_;
  std::string int_direction_;
  std::string ref_direction_;
  std::string int_ref_direction_;
  std::string destination_;
  std::string destination_ref_;
  std::string destination_ref_to_;
  std::string destination_street_;
  std::string destination_street_to_;
  std::string junction_ref_;
  std::string turn_lanes_;
  std::string turn_lanes_forward_;
  std::string turn_lanes_backward_;
  std::string level_;
  std::vector<std::string> pronunciation_tags_;
  std::vector<std::string> destination_pronunciation_tags_;
  std::vector<std::string> ref_pronunciation_tags_;
  std::vector<std::string> int_ref_pronunciation_tags_;
  std::vector<std::string> junction_ref_pronunciation_tags_;
  std::vector<std::string> direction_pronunciation_tags_;
  std::vector<std::string> int_direction_pronunciation_tags_;
  std::vector<std::string> ref_direction_pronunciation_tags_;
  std::vector<std::string> int_ref_direction_pronunciation_tags_;
  std::vector<std::string> destination_ref_pronunciation_tags_;
  std::vector<std::string> destination_ref_to_pronunciation_tags_;
  std::vector<std::string> destination_street_pronunciation_tags_;
  std::vector<std::string> destination_street_to_pronunciation_tags_;
  uint64_t way_id_;
  uint32_t restrictions_;
  uint32_t access_restrictions_;
  uint32_t edge_index_;
  uint32_t lanes_;
  uint32_t forward_lanes_;
  uint32_t backward_lanes_;
  uint32_t sac_scale_;
  uint32_t attributes_;
  uint8_t speed_;
  uint8_t backward_speed_;
  uint8_t max_speed_;
  uint8_t truck_speed_;
  uint8_t bike_speed_;
  uint8_t bike_network_;
  uint8_t track_;
  uint8_t hov_lanes_;
  int8_t layer_;
  RoadClass road_class_;
  Use use_;
  CycleLane cycle_lane_;
  CycleLane cycle_lane_right_;
  CycleLane cycle_lane_left_;
  Surface surface_;
  HOVType hov_type_;
  SpeedType speed_type_;
  RoadClass classification_;
};
```

### OSMNode Structure

The `OSMNode` structure represents a node (intersection):

```cpp
// From valhalla/mjolnir/osmnode.h
struct OSMNode {
  OSMNode() = delete;
  OSMNode(const uint64_t id, const double lat, const double lng)
      : node_id(id), access(0), type(NodeType::kStreetIntersection), 
        intersection(false), traffic_signal(false), forward_signal(false),
        backward_signal(false), non_link_edge(false), link_edge(false),
        shortlink(false), non_ferry_edge(false), ferry_edge(false),
        flat_loop(false), urban(false), tagged_access(false), 
        private_access(false), cash_only_toll(false), 
        lat(lat), lng(lng) {
  }
  
  uint64_t node_id;
  uint16_t access;
  NodeType type;
  bool intersection;
  bool traffic_signal;
  bool forward_signal;
  bool backward_signal;
  bool non_link_edge;
  bool link_edge;
  bool shortlink;
  bool non_ferry_edge;
  bool ferry_edge;
  bool flat_loop;
  bool urban;
  bool tagged_access;
  bool private_access;
  bool cash_only_toll;
  double lat;
  double lng;
};
```

### OSMWayNode Structure

The `OSMWayNode` structure represents a node that's part of a way:

```cpp
// From valhalla/mjolnir/osmdata.h
struct OSMWayNode {
  OSMWayNode() = default;
  OSMWayNode(const uint64_t way_id, const uint64_t node_id)
      : way_id(way_id), node_id(node_id) {
  }
  
  uint64_t way_id;
  uint64_t node_id;
  
  bool operator<(const OSMWayNode& other) const {
    if (way_id == other.way_id) {
      return node_id < other.node_id;
    }
    return way_id < other.way_id;
  }
};
```

### Administrative Boundaries

Kevin mentions the need to handle administrative boundaries. In Valhalla, this is done using a spatialite database:

```cpp
// From src/mjolnir/adminbuilder.cc (simplified)
void AdminBuilder::Build(const boost::property_tree::ptree& pt) {
  // Connect to the spatialite database
  sqlite3* db_handle = nullptr;
  sqlite3_open_v2(pt.get<std::string>("admin").c_str(), &db_handle,
                 SQLITE_OPEN_READONLY, nullptr);
  
  // Initialize spatialite
  InitializeSpatialite(db_handle);
  
  // Process each tile
  for (const auto& tile_id : tile_ids) {
    GraphTileBuilder tile_builder(tile_id);
    
    // For each node in the tile
    for (const auto& node : tile_builder.nodes()) {
      // Find the admin areas that contain this node
      std::vector<Admin> admins = FindAdmins(db_handle, node.latlng());
      
      // Assign admin areas to the node
      tile_builder.AddNodeAdmin(node.id(), admins);
    }
    
    // Save the tile
    tile_builder.StoreTileData();
  }
  
  // Close the database
  sqlite3_close(db_handle);
}
```

## Mapping Overture to Valhalla Structures

Based on the code examination, here's how we might map Overture data to Valhalla's structures:

### Overture Segment → OSMWay

Overture segments would map to `OSMWay` objects:
- Segment ID → way_id
- Segment geometry → node_ids (converted to OSMNode references)
- Segment properties → various OSMWay attributes (road_class, speed, etc.)

### Overture Connector → OSMNode

Overture connectors would map to `OSMNode` objects:
- Connector ID → node_id
- Connector position → lat, lng
- Connector properties → various OSMNode attributes (intersection, traffic_signal, etc.)

### Overture Properties → OSMWay/OSMNode Attributes

Overture's property structure would need to be flattened and mapped to the various attributes in `OSMWay` and `OSMNode`.

## Administrative Boundaries

Kevin suggests using DuckDB to process Overture's admin boundary data into a spatialite database. This makes sense because:

1. DuckDB has good support for GeoParquet and other spatial formats
2. Valhalla expects admin boundaries in a spatialite database
3. This approach would minimize changes to Valhalla's admin handling code

## Conclusion

Kevin's suggestion to focus on creating a transcoder from Overture data to Valhalla's intermediate structures is well-founded. The `OSMData`, `OSMWay`, `OSMNode`, and `OSMWayNode` structures are indeed the key targets for our transcoder. By focusing on these structures and the `ConstructEdges` phase, we can leverage Valhalla's powerful routing capabilities while minimizing the amount of code that needs to be modified.
