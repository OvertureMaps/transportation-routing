// This file contains definitions for OSMWay, OSMNode, and OSMWayNode
// The structs have been copied from the following files in the Valhalla repo:
// - https://github.com/valhalla/valhalla/blob/master/valhalla/mjolnir/osmnode.h
// - https://github.com/valhalla/valhalla/blob/master/valhalla/mjolnir/osmdata.h
// - https://github.com/valhalla/valhalla/blob/master/valhalla/mjolnir/osmway.h

#include <stdint.h>

struct OSMWay {
  // OSM way Id
  uint64_t osmwayid_;

  // Reference name (highway numbers)
  uint32_t ref_index_;
  uint32_t ref_lang_index_;
  uint32_t ref_left_index_;
  uint32_t ref_left_lang_index_;
  uint32_t ref_right_index_;
  uint32_t ref_right_lang_index_;

  uint32_t int_ref_index_;
  uint32_t int_ref_lang_index_;
  uint32_t int_ref_left_index_;
  uint32_t int_ref_left_lang_index_;
  uint32_t int_ref_right_index_;
  uint32_t int_ref_right_lang_index_;

  // Names
  uint32_t name_index_;
  uint32_t name_lang_index_;
  uint32_t name_left_index_;
  uint32_t name_left_lang_index_;
  uint32_t name_right_index_;
  uint32_t name_right_lang_index_;

  uint32_t name_forward_index_;
  uint32_t name_forward_lang_index_;
  uint32_t name_backward_index_;
  uint32_t name_backward_lang_index_;

  uint32_t alt_name_index_;
  uint32_t alt_name_lang_index_;
  uint32_t alt_name_left_index_;
  uint32_t alt_name_left_lang_index_;
  uint32_t alt_name_right_index_;
  uint32_t alt_name_right_lang_index_;

  uint32_t official_name_index_;
  uint32_t official_name_lang_index_;
  uint32_t official_name_left_index_;
  uint32_t official_name_left_lang_index_;
  uint32_t official_name_right_index_;
  uint32_t official_name_right_lang_index_;

  uint32_t tunnel_name_index_;
  uint32_t tunnel_name_lang_index_;
  uint32_t tunnel_name_left_index_;
  uint32_t tunnel_name_left_lang_index_;
  uint32_t tunnel_name_right_index_;
  uint32_t tunnel_name_right_lang_index_;

  // Turn lanes
  uint32_t fwd_turn_lanes_index_;
  uint32_t bwd_turn_lanes_index_;

  // Guidance views
  uint32_t fwd_jct_base_index_;
  uint32_t bwd_jct_base_index_;

  uint32_t fwd_jct_overlay_index_;
  uint32_t bwd_jct_overlay_index_;

  uint32_t fwd_signboard_base_index_;
  uint32_t bwd_signboard_base_index_;

  // Sign Destination information
  uint32_t destination_index_;
  uint32_t destination_lang_index_;
  uint32_t destination_forward_index_;
  uint32_t destination_backward_index_;
  uint32_t destination_forward_lang_index_;
  uint32_t destination_backward_lang_index_;
  uint32_t destination_ref_index_;
  uint32_t destination_ref_lang_index_;
  uint32_t destination_ref_to_index_;
  uint32_t destination_ref_to_lang_index_;
  uint32_t destination_int_ref_index_;
  uint32_t destination_int_ref_to_index_;
  uint32_t destination_street_index_;
  uint32_t destination_street_lang_index_;
  uint32_t destination_street_to_index_;
  uint32_t destination_street_to_lang_index_;
  uint32_t junction_name_index_;
  uint32_t junction_name_lang_index_;
  uint32_t junction_ref_index_;
  uint32_t junction_ref_lang_index_;

  // level and level:ref of the way
  uint32_t level_index_;
  uint32_t level_ref_index_;

  // Bike network information. TODO - these are not yet used.
  //  uint32_t bike_national_ref_index_;
  //  uint32_t bike_regional_ref_index_;
  //  uint32_t bike_local_ref_index_;

  // duration of a ferry in seconds
  uint32_t duration_;

  // Way attributes
  uint32_t destination_only_ : 1;
  uint32_t no_thru_traffic_ : 1;
  uint32_t oneway_ : 1;
  uint32_t oneway_reverse_ : 1;
  uint32_t roundabout_ : 1;
  uint32_t ferry_ : 1;
  uint32_t rail_ : 1;
  uint32_t surface_ : 3;
  uint32_t tunnel_ : 1;
  uint32_t toll_ : 1;
  uint32_t bridge_ : 1;
  uint32_t seasonal_ : 1;
  uint32_t drive_on_right_ : 1;
  uint32_t bike_network_ : 4;
  uint32_t exit_ : 1;
  uint32_t tagged_speed_ : 1;
  uint32_t forward_tagged_speed_ : 1;
  uint32_t backward_tagged_speed_ : 1;
  uint32_t tagged_lanes_ : 1;
  uint32_t forward_tagged_lanes_ : 1;
  uint32_t backward_tagged_lanes_ : 1;
  uint32_t truck_route_ : 1;
  uint32_t sidewalk_right_ : 1;
  uint32_t sidewalk_left_ : 1;
  uint32_t sac_scale_ : 3;

  // Classification
  uint32_t road_class_ : 3; // Importance of the road/path
  uint32_t link_ : 1;       // *link tag - Ramp or turn channel
  uint32_t use_ : 6;        // Use / form
  uint32_t lanes_ : 4;
  uint32_t forward_lanes_ : 4;
  uint32_t backward_lanes_ : 4;
  uint32_t turn_channel_ : 1; // *link tag - turn channel (no ramp)
  uint32_t wheelchair_ : 1;
  uint32_t wheelchair_tag_ : 1;
  uint32_t has_user_tags_ : 1;
  uint32_t has_pronunciation_tags_ : 1;
  uint32_t internal_ : 1;
  uint32_t hov_type_ : 1;
  uint32_t indoor_ : 1;
  uint32_t pedestrian_forward_ : 1;
  uint32_t pedestrian_backward_ : 1;

  // Access
  uint16_t auto_forward_ : 1;
  uint16_t bus_forward_ : 1;
  uint16_t taxi_forward_ : 1;
  uint16_t truck_forward_ : 1;
  uint16_t motorcycle_forward_ : 1;
  uint16_t emergency_forward_ : 1;
  uint16_t hov_forward_ : 1;
  uint16_t moped_forward_ : 1;
  uint16_t auto_backward_ : 1;
  uint16_t bus_backward_ : 1;
  uint16_t taxi_backward_ : 1;
  uint16_t truck_backward_ : 1;
  uint16_t motorcycle_backward_ : 1;
  uint16_t emergency_backward_ : 1;
  uint16_t hov_backward_ : 1;
  uint16_t moped_backward_ : 1;

  // Attributes specific to biking
  uint16_t cycle_lane_right_ : 2;
  uint16_t cycle_lane_left_ : 2;
  uint16_t cycle_lane_right_opposite_ : 1;
  uint16_t cycle_lane_left_opposite_ : 1;
  uint16_t shoulder_right_ : 1;
  uint16_t shoulder_left_ : 1;
  uint16_t dismount_ : 1;
  uint16_t use_sidepath_ : 1;
  uint16_t bike_forward_ : 1;
  uint16_t bike_backward_ : 1;
  uint16_t lit_ : 1;
  uint16_t destination_only_hgv_ : 1;
  uint16_t spare2_ : 2;

  uint16_t nodecount_;

  // max speed limit in kilometers per hour
  uint8_t speed_limit_;

  // average speed if exists, else advisory speed if exists, else max_speed if exists,
  // else categorized speed in kilometers per hour
  uint8_t speed_;

  // Speed in kilometers per hour
  uint8_t backward_speed_;

  // Speed in kilometers per hour
  uint8_t forward_speed_;

  // Truck speed in kilometers per hour
  uint8_t truck_speed_;
  uint8_t truck_speed_forward_;
  uint8_t truck_speed_backward_;

  // layer index(Z-level) of the way relatively to other levels
  int8_t layer_;
};

struct OSMNode {
  // The osm id of the node
  uint64_t osmid_;

  // Store node names in a separate list (so they don't require as many indexes)
  uint64_t name_index_ : 21;
  uint64_t ref_index_ : 21;
  uint64_t exit_to_index_ : 21;
  uint64_t named_intersection_ : 1;

  uint64_t country_iso_index_ : 21;
  uint64_t state_iso_index_ : 21;
  uint64_t traffic_signal_ : 1;
  uint64_t forward_signal_ : 1;
  uint64_t backward_signal_ : 1;
  uint64_t stop_sign_ : 1;
  uint64_t forward_stop_ : 1;
  uint64_t backward_stop_ : 1;
  uint64_t yield_sign_ : 1;
  uint64_t forward_yield_ : 1;
  uint64_t backward_yield_ : 1;
  uint64_t minor_ : 1;
  uint64_t direction_ : 1;
  uint64_t spare_ : 11;

  uint32_t access_ : 12;
  uint32_t type_ : 4;
  uint32_t intersection_ : 1;
  uint32_t non_link_edge_ : 1;
  uint32_t link_edge_ : 1;
  uint32_t shortlink_ : 1; // Link edge < kMaxInternalLength
  uint32_t non_ferry_edge_ : 1;
  uint32_t ferry_edge_ : 1;
  uint32_t flat_loop_ : 1; // A node which on a section of a way that is doubled back on itself
  uint32_t urban_ : 1;
  uint32_t tagged_access_ : 1; // Was access originally tagged?
  uint32_t private_access_ : 1;
  uint32_t cash_only_toll_ : 1;
  uint32_t spare1_ : 5;

  // bss information
  uint32_t bss_info_;

  // linguistic information
  uint32_t linguistic_info_index_;

  // Lat,lng of the node at fixed 7digit precision
  uint32_t lng7_;
  uint32_t lat7_;
};

struct OSMWayNode {
  struct OSMNode node;
  uint32_t way_index;
  uint32_t way_shape_node_index;
};

