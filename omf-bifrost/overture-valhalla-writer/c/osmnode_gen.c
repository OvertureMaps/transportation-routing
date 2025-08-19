#include "osmnode.h"

#include <stdio.h>
#include <stdint.h>

void osmnode_set_osmid_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t osmid_;
   s[index].osmid_ = value;
}

uint64_t osmnode_get_osmid_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t osmid_;
   return s[index].osmid_;
}

void osmnode_set_name_index_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t name_index_ : 21;
   s[index].name_index_ = value;
}

uint64_t osmnode_get_name_index_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t name_index_ : 21;
   return s[index].name_index_;
}

void osmnode_set_ref_index_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t ref_index_ : 21;
   s[index].ref_index_ = value;
}

uint64_t osmnode_get_ref_index_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t ref_index_ : 21;
   return s[index].ref_index_;
}

void osmnode_set_exit_to_index_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t exit_to_index_ : 21;
   s[index].exit_to_index_ = value;
}

uint64_t osmnode_get_exit_to_index_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t exit_to_index_ : 21;
   return s[index].exit_to_index_;
}

void osmnode_set_named_intersection_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t named_intersection_ : 1;
   s[index].named_intersection_ = value;
}

uint64_t osmnode_get_named_intersection_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t named_intersection_ : 1;
   return s[index].named_intersection_;
}

void osmnode_set_country_iso_index_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t country_iso_index_ : 21;
   s[index].country_iso_index_ = value;
}

uint64_t osmnode_get_country_iso_index_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t country_iso_index_ : 21;
   return s[index].country_iso_index_;
}

void osmnode_set_state_iso_index_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t state_iso_index_ : 21;
   s[index].state_iso_index_ = value;
}

uint64_t osmnode_get_state_iso_index_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t state_iso_index_ : 21;
   return s[index].state_iso_index_;
}

void osmnode_set_traffic_signal_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t traffic_signal_ : 1;
   s[index].traffic_signal_ = value;
}

uint64_t osmnode_get_traffic_signal_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t traffic_signal_ : 1;
   return s[index].traffic_signal_;
}

void osmnode_set_forward_signal_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t forward_signal_ : 1;
   s[index].forward_signal_ = value;
}

uint64_t osmnode_get_forward_signal_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t forward_signal_ : 1;
   return s[index].forward_signal_;
}

void osmnode_set_backward_signal_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t backward_signal_ : 1;
   s[index].backward_signal_ = value;
}

uint64_t osmnode_get_backward_signal_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t backward_signal_ : 1;
   return s[index].backward_signal_;
}

void osmnode_set_stop_sign_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t stop_sign_ : 1;
   s[index].stop_sign_ = value;
}

uint64_t osmnode_get_stop_sign_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t stop_sign_ : 1;
   return s[index].stop_sign_;
}

void osmnode_set_forward_stop_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t forward_stop_ : 1;
   s[index].forward_stop_ = value;
}

uint64_t osmnode_get_forward_stop_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t forward_stop_ : 1;
   return s[index].forward_stop_;
}

void osmnode_set_backward_stop_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t backward_stop_ : 1;
   s[index].backward_stop_ = value;
}

uint64_t osmnode_get_backward_stop_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t backward_stop_ : 1;
   return s[index].backward_stop_;
}

void osmnode_set_yield_sign_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t yield_sign_ : 1;
   s[index].yield_sign_ = value;
}

uint64_t osmnode_get_yield_sign_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t yield_sign_ : 1;
   return s[index].yield_sign_;
}

void osmnode_set_forward_yield_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t forward_yield_ : 1;
   s[index].forward_yield_ = value;
}

uint64_t osmnode_get_forward_yield_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t forward_yield_ : 1;
   return s[index].forward_yield_;
}

void osmnode_set_backward_yield_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t backward_yield_ : 1;
   s[index].backward_yield_ = value;
}

uint64_t osmnode_get_backward_yield_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t backward_yield_ : 1;
   return s[index].backward_yield_;
}

void osmnode_set_minor_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t minor_ : 1;
   s[index].minor_ = value;
}

uint64_t osmnode_get_minor_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t minor_ : 1;
   return s[index].minor_;
}

void osmnode_set_direction_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t direction_ : 1;
   s[index].direction_ = value;
}

uint64_t osmnode_get_direction_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t direction_ : 1;
   return s[index].direction_;
}

void osmnode_set_spare_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint64_t spare_ : 11;
   s[index].spare_ = value;
}

uint64_t osmnode_get_spare_(const struct OSMNode * s, uint64_t index)
{
   // uint64_t spare_ : 11;
   return s[index].spare_;
}

void osmnode_set_access_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t access_ : 12;
   s[index].access_ = value;
}

uint64_t osmnode_get_access_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t access_ : 12;
   return s[index].access_;
}

void osmnode_set_type_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t type_ : 4;
   s[index].type_ = value;
}

uint64_t osmnode_get_type_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t type_ : 4;
   return s[index].type_;
}

void osmnode_set_intersection_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t intersection_ : 1;
   s[index].intersection_ = value;
}

uint64_t osmnode_get_intersection_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t intersection_ : 1;
   return s[index].intersection_;
}

void osmnode_set_non_link_edge_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t non_link_edge_ : 1;
   s[index].non_link_edge_ = value;
}

uint64_t osmnode_get_non_link_edge_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t non_link_edge_ : 1;
   return s[index].non_link_edge_;
}

void osmnode_set_link_edge_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t link_edge_ : 1;
   s[index].link_edge_ = value;
}

uint64_t osmnode_get_link_edge_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t link_edge_ : 1;
   return s[index].link_edge_;
}

void osmnode_set_shortlink_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t shortlink_ : 1;
   s[index].shortlink_ = value;
}

uint64_t osmnode_get_shortlink_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t shortlink_ : 1;
   return s[index].shortlink_;
}

void osmnode_set_non_ferry_edge_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t non_ferry_edge_ : 1;
   s[index].non_ferry_edge_ = value;
}

uint64_t osmnode_get_non_ferry_edge_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t non_ferry_edge_ : 1;
   return s[index].non_ferry_edge_;
}

void osmnode_set_ferry_edge_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t ferry_edge_ : 1;
   s[index].ferry_edge_ = value;
}

uint64_t osmnode_get_ferry_edge_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t ferry_edge_ : 1;
   return s[index].ferry_edge_;
}

void osmnode_set_flat_loop_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t flat_loop_ : 1;
   s[index].flat_loop_ = value;
}

uint64_t osmnode_get_flat_loop_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t flat_loop_ : 1;
   return s[index].flat_loop_;
}

void osmnode_set_urban_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t urban_ : 1;
   s[index].urban_ = value;
}

uint64_t osmnode_get_urban_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t urban_ : 1;
   return s[index].urban_;
}

void osmnode_set_tagged_access_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t tagged_access_ : 1;
   s[index].tagged_access_ = value;
}

uint64_t osmnode_get_tagged_access_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t tagged_access_ : 1;
   return s[index].tagged_access_;
}

void osmnode_set_private_access_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t private_access_ : 1;
   s[index].private_access_ = value;
}

uint64_t osmnode_get_private_access_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t private_access_ : 1;
   return s[index].private_access_;
}

void osmnode_set_cash_only_toll_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t cash_only_toll_ : 1;
   s[index].cash_only_toll_ = value;
}

uint64_t osmnode_get_cash_only_toll_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t cash_only_toll_ : 1;
   return s[index].cash_only_toll_;
}

void osmnode_set_spare1_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t spare1_ : 5;
   s[index].spare1_ = value;
}

uint64_t osmnode_get_spare1_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t spare1_ : 5;
   return s[index].spare1_;
}

void osmnode_set_bss_info_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t bss_info_;
   s[index].bss_info_ = value;
}

uint64_t osmnode_get_bss_info_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t bss_info_;
   return s[index].bss_info_;
}

void osmnode_set_linguistic_info_index_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t linguistic_info_index_;
   s[index].linguistic_info_index_ = value;
}

uint64_t osmnode_get_linguistic_info_index_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t linguistic_info_index_;
   return s[index].linguistic_info_index_;
}

void osmnode_set_lng7_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t lng7_;
   s[index].lng7_ = value;
}

uint64_t osmnode_get_lng7_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t lng7_;
   return s[index].lng7_;
}

void osmnode_set_lat7_(struct OSMNode * s, uint64_t index, uint64_t value)
{
   // uint32_t lat7_;
   s[index].lat7_ = value;
}

uint64_t osmnode_get_lat7_(const struct OSMNode * s, uint64_t index)
{
   // uint32_t lat7_;
   return s[index].lat7_;
}

static void print_field(const char * field_name, uint64_t value)
{
   printf("%s: %lu\n", field_name, value);
}

void osmnode_dump(const struct OSMNode * s, uint64_t index)
{
   // Dumping fields for OSMNode
   print_field("osmid_", (uint64_t)s[index].osmid_);
   print_field("name_index_", (uint64_t)s[index].name_index_);
   print_field("ref_index_", (uint64_t)s[index].ref_index_);
   print_field("exit_to_index_", (uint64_t)s[index].exit_to_index_);
   print_field("named_intersection_", (uint64_t)s[index].named_intersection_);
   print_field("country_iso_index_", (uint64_t)s[index].country_iso_index_);
   print_field("state_iso_index_", (uint64_t)s[index].state_iso_index_);
   print_field("traffic_signal_", (uint64_t)s[index].traffic_signal_);
   print_field("forward_signal_", (uint64_t)s[index].forward_signal_);
   print_field("backward_signal_", (uint64_t)s[index].backward_signal_);
   print_field("stop_sign_", (uint64_t)s[index].stop_sign_);
   print_field("forward_stop_", (uint64_t)s[index].forward_stop_);
   print_field("backward_stop_", (uint64_t)s[index].backward_stop_);
   print_field("yield_sign_", (uint64_t)s[index].yield_sign_);
   print_field("forward_yield_", (uint64_t)s[index].forward_yield_);
   print_field("backward_yield_", (uint64_t)s[index].backward_yield_);
   print_field("minor_", (uint64_t)s[index].minor_);
   print_field("direction_", (uint64_t)s[index].direction_);
   print_field("spare_", (uint64_t)s[index].spare_);
   print_field("access_", (uint64_t)s[index].access_);
   print_field("type_", (uint64_t)s[index].type_);
   print_field("intersection_", (uint64_t)s[index].intersection_);
   print_field("non_link_edge_", (uint64_t)s[index].non_link_edge_);
   print_field("link_edge_", (uint64_t)s[index].link_edge_);
   print_field("shortlink_", (uint64_t)s[index].shortlink_);
   print_field("non_ferry_edge_", (uint64_t)s[index].non_ferry_edge_);
   print_field("ferry_edge_", (uint64_t)s[index].ferry_edge_);
   print_field("flat_loop_", (uint64_t)s[index].flat_loop_);
   print_field("urban_", (uint64_t)s[index].urban_);
   print_field("tagged_access_", (uint64_t)s[index].tagged_access_);
   print_field("private_access_", (uint64_t)s[index].private_access_);
   print_field("cash_only_toll_", (uint64_t)s[index].cash_only_toll_);
   print_field("spare1_", (uint64_t)s[index].spare1_);
   print_field("bss_info_", (uint64_t)s[index].bss_info_);
   print_field("linguistic_info_index_", (uint64_t)s[index].linguistic_info_index_);
   print_field("lng7_", (uint64_t)s[index].lng7_);
   print_field("lat7_", (uint64_t)s[index].lat7_);
}

