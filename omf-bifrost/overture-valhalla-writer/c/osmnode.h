#ifndef OSMNODE_H
#define OSMNODE_H

#include <stdint.h>

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

void osmnode_set_to_valhalla(struct OSMNode * s, uint64_t index, uint64_t osmid, uint64_t lng7, uint64_t lat7, uint64_t intersection);

#endif /* OSMNODE_H */
