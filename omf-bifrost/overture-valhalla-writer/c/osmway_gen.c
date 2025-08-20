#include "osmway.h"

#include <stdio.h>
#include <stdint.h>

void osmway_set_osmwayid_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint64_t osmwayid_;
   s[index].osmwayid_ = value;
}

uint64_t osmway_get_osmwayid_(const struct OSMWay * s, uint64_t index)
{
   // uint64_t osmwayid_;
   return s[index].osmwayid_;
}

void osmway_set_ref_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t ref_index_;
   s[index].ref_index_ = value;
}

uint64_t osmway_get_ref_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t ref_index_;
   return s[index].ref_index_;
}

void osmway_set_ref_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t ref_lang_index_;
   s[index].ref_lang_index_ = value;
}

uint64_t osmway_get_ref_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t ref_lang_index_;
   return s[index].ref_lang_index_;
}

void osmway_set_ref_left_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t ref_left_index_;
   s[index].ref_left_index_ = value;
}

uint64_t osmway_get_ref_left_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t ref_left_index_;
   return s[index].ref_left_index_;
}

void osmway_set_ref_left_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t ref_left_lang_index_;
   s[index].ref_left_lang_index_ = value;
}

uint64_t osmway_get_ref_left_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t ref_left_lang_index_;
   return s[index].ref_left_lang_index_;
}

void osmway_set_ref_right_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t ref_right_index_;
   s[index].ref_right_index_ = value;
}

uint64_t osmway_get_ref_right_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t ref_right_index_;
   return s[index].ref_right_index_;
}

void osmway_set_ref_right_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t ref_right_lang_index_;
   s[index].ref_right_lang_index_ = value;
}

uint64_t osmway_get_ref_right_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t ref_right_lang_index_;
   return s[index].ref_right_lang_index_;
}

void osmway_set_int_ref_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t int_ref_index_;
   s[index].int_ref_index_ = value;
}

uint64_t osmway_get_int_ref_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t int_ref_index_;
   return s[index].int_ref_index_;
}

void osmway_set_int_ref_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t int_ref_lang_index_;
   s[index].int_ref_lang_index_ = value;
}

uint64_t osmway_get_int_ref_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t int_ref_lang_index_;
   return s[index].int_ref_lang_index_;
}

void osmway_set_int_ref_left_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t int_ref_left_index_;
   s[index].int_ref_left_index_ = value;
}

uint64_t osmway_get_int_ref_left_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t int_ref_left_index_;
   return s[index].int_ref_left_index_;
}

void osmway_set_int_ref_left_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t int_ref_left_lang_index_;
   s[index].int_ref_left_lang_index_ = value;
}

uint64_t osmway_get_int_ref_left_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t int_ref_left_lang_index_;
   return s[index].int_ref_left_lang_index_;
}

void osmway_set_int_ref_right_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t int_ref_right_index_;
   s[index].int_ref_right_index_ = value;
}

uint64_t osmway_get_int_ref_right_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t int_ref_right_index_;
   return s[index].int_ref_right_index_;
}

void osmway_set_int_ref_right_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t int_ref_right_lang_index_;
   s[index].int_ref_right_lang_index_ = value;
}

uint64_t osmway_get_int_ref_right_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t int_ref_right_lang_index_;
   return s[index].int_ref_right_lang_index_;
}

void osmway_set_name_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_index_;
   s[index].name_index_ = value;
}

uint64_t osmway_get_name_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_index_;
   return s[index].name_index_;
}

void osmway_set_name_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_lang_index_;
   s[index].name_lang_index_ = value;
}

uint64_t osmway_get_name_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_lang_index_;
   return s[index].name_lang_index_;
}

void osmway_set_name_left_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_left_index_;
   s[index].name_left_index_ = value;
}

uint64_t osmway_get_name_left_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_left_index_;
   return s[index].name_left_index_;
}

void osmway_set_name_left_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_left_lang_index_;
   s[index].name_left_lang_index_ = value;
}

uint64_t osmway_get_name_left_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_left_lang_index_;
   return s[index].name_left_lang_index_;
}

void osmway_set_name_right_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_right_index_;
   s[index].name_right_index_ = value;
}

uint64_t osmway_get_name_right_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_right_index_;
   return s[index].name_right_index_;
}

void osmway_set_name_right_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_right_lang_index_;
   s[index].name_right_lang_index_ = value;
}

uint64_t osmway_get_name_right_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_right_lang_index_;
   return s[index].name_right_lang_index_;
}

void osmway_set_name_forward_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_forward_index_;
   s[index].name_forward_index_ = value;
}

uint64_t osmway_get_name_forward_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_forward_index_;
   return s[index].name_forward_index_;
}

void osmway_set_name_forward_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_forward_lang_index_;
   s[index].name_forward_lang_index_ = value;
}

uint64_t osmway_get_name_forward_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_forward_lang_index_;
   return s[index].name_forward_lang_index_;
}

void osmway_set_name_backward_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_backward_index_;
   s[index].name_backward_index_ = value;
}

uint64_t osmway_get_name_backward_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_backward_index_;
   return s[index].name_backward_index_;
}

void osmway_set_name_backward_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t name_backward_lang_index_;
   s[index].name_backward_lang_index_ = value;
}

uint64_t osmway_get_name_backward_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t name_backward_lang_index_;
   return s[index].name_backward_lang_index_;
}

void osmway_set_alt_name_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t alt_name_index_;
   s[index].alt_name_index_ = value;
}

uint64_t osmway_get_alt_name_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t alt_name_index_;
   return s[index].alt_name_index_;
}

void osmway_set_alt_name_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t alt_name_lang_index_;
   s[index].alt_name_lang_index_ = value;
}

uint64_t osmway_get_alt_name_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t alt_name_lang_index_;
   return s[index].alt_name_lang_index_;
}

void osmway_set_alt_name_left_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t alt_name_left_index_;
   s[index].alt_name_left_index_ = value;
}

uint64_t osmway_get_alt_name_left_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t alt_name_left_index_;
   return s[index].alt_name_left_index_;
}

void osmway_set_alt_name_left_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t alt_name_left_lang_index_;
   s[index].alt_name_left_lang_index_ = value;
}

uint64_t osmway_get_alt_name_left_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t alt_name_left_lang_index_;
   return s[index].alt_name_left_lang_index_;
}

void osmway_set_alt_name_right_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t alt_name_right_index_;
   s[index].alt_name_right_index_ = value;
}

uint64_t osmway_get_alt_name_right_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t alt_name_right_index_;
   return s[index].alt_name_right_index_;
}

void osmway_set_alt_name_right_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t alt_name_right_lang_index_;
   s[index].alt_name_right_lang_index_ = value;
}

uint64_t osmway_get_alt_name_right_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t alt_name_right_lang_index_;
   return s[index].alt_name_right_lang_index_;
}

void osmway_set_official_name_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t official_name_index_;
   s[index].official_name_index_ = value;
}

uint64_t osmway_get_official_name_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t official_name_index_;
   return s[index].official_name_index_;
}

void osmway_set_official_name_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t official_name_lang_index_;
   s[index].official_name_lang_index_ = value;
}

uint64_t osmway_get_official_name_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t official_name_lang_index_;
   return s[index].official_name_lang_index_;
}

void osmway_set_official_name_left_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t official_name_left_index_;
   s[index].official_name_left_index_ = value;
}

uint64_t osmway_get_official_name_left_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t official_name_left_index_;
   return s[index].official_name_left_index_;
}

void osmway_set_official_name_left_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t official_name_left_lang_index_;
   s[index].official_name_left_lang_index_ = value;
}

uint64_t osmway_get_official_name_left_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t official_name_left_lang_index_;
   return s[index].official_name_left_lang_index_;
}

void osmway_set_official_name_right_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t official_name_right_index_;
   s[index].official_name_right_index_ = value;
}

uint64_t osmway_get_official_name_right_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t official_name_right_index_;
   return s[index].official_name_right_index_;
}

void osmway_set_official_name_right_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t official_name_right_lang_index_;
   s[index].official_name_right_lang_index_ = value;
}

uint64_t osmway_get_official_name_right_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t official_name_right_lang_index_;
   return s[index].official_name_right_lang_index_;
}

void osmway_set_tunnel_name_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tunnel_name_index_;
   s[index].tunnel_name_index_ = value;
}

uint64_t osmway_get_tunnel_name_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tunnel_name_index_;
   return s[index].tunnel_name_index_;
}

void osmway_set_tunnel_name_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tunnel_name_lang_index_;
   s[index].tunnel_name_lang_index_ = value;
}

uint64_t osmway_get_tunnel_name_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tunnel_name_lang_index_;
   return s[index].tunnel_name_lang_index_;
}

void osmway_set_tunnel_name_left_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tunnel_name_left_index_;
   s[index].tunnel_name_left_index_ = value;
}

uint64_t osmway_get_tunnel_name_left_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tunnel_name_left_index_;
   return s[index].tunnel_name_left_index_;
}

void osmway_set_tunnel_name_left_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tunnel_name_left_lang_index_;
   s[index].tunnel_name_left_lang_index_ = value;
}

uint64_t osmway_get_tunnel_name_left_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tunnel_name_left_lang_index_;
   return s[index].tunnel_name_left_lang_index_;
}

void osmway_set_tunnel_name_right_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tunnel_name_right_index_;
   s[index].tunnel_name_right_index_ = value;
}

uint64_t osmway_get_tunnel_name_right_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tunnel_name_right_index_;
   return s[index].tunnel_name_right_index_;
}

void osmway_set_tunnel_name_right_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tunnel_name_right_lang_index_;
   s[index].tunnel_name_right_lang_index_ = value;
}

uint64_t osmway_get_tunnel_name_right_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tunnel_name_right_lang_index_;
   return s[index].tunnel_name_right_lang_index_;
}

void osmway_set_fwd_turn_lanes_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t fwd_turn_lanes_index_;
   s[index].fwd_turn_lanes_index_ = value;
}

uint64_t osmway_get_fwd_turn_lanes_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t fwd_turn_lanes_index_;
   return s[index].fwd_turn_lanes_index_;
}

void osmway_set_bwd_turn_lanes_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t bwd_turn_lanes_index_;
   s[index].bwd_turn_lanes_index_ = value;
}

uint64_t osmway_get_bwd_turn_lanes_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t bwd_turn_lanes_index_;
   return s[index].bwd_turn_lanes_index_;
}

void osmway_set_fwd_jct_base_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t fwd_jct_base_index_;
   s[index].fwd_jct_base_index_ = value;
}

uint64_t osmway_get_fwd_jct_base_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t fwd_jct_base_index_;
   return s[index].fwd_jct_base_index_;
}

void osmway_set_bwd_jct_base_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t bwd_jct_base_index_;
   s[index].bwd_jct_base_index_ = value;
}

uint64_t osmway_get_bwd_jct_base_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t bwd_jct_base_index_;
   return s[index].bwd_jct_base_index_;
}

void osmway_set_fwd_jct_overlay_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t fwd_jct_overlay_index_;
   s[index].fwd_jct_overlay_index_ = value;
}

uint64_t osmway_get_fwd_jct_overlay_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t fwd_jct_overlay_index_;
   return s[index].fwd_jct_overlay_index_;
}

void osmway_set_bwd_jct_overlay_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t bwd_jct_overlay_index_;
   s[index].bwd_jct_overlay_index_ = value;
}

uint64_t osmway_get_bwd_jct_overlay_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t bwd_jct_overlay_index_;
   return s[index].bwd_jct_overlay_index_;
}

void osmway_set_fwd_signboard_base_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t fwd_signboard_base_index_;
   s[index].fwd_signboard_base_index_ = value;
}

uint64_t osmway_get_fwd_signboard_base_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t fwd_signboard_base_index_;
   return s[index].fwd_signboard_base_index_;
}

void osmway_set_bwd_signboard_base_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t bwd_signboard_base_index_;
   s[index].bwd_signboard_base_index_ = value;
}

uint64_t osmway_get_bwd_signboard_base_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t bwd_signboard_base_index_;
   return s[index].bwd_signboard_base_index_;
}

void osmway_set_destination_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_index_;
   s[index].destination_index_ = value;
}

uint64_t osmway_get_destination_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_index_;
   return s[index].destination_index_;
}

void osmway_set_destination_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_lang_index_;
   s[index].destination_lang_index_ = value;
}

uint64_t osmway_get_destination_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_lang_index_;
   return s[index].destination_lang_index_;
}

void osmway_set_destination_forward_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_forward_index_;
   s[index].destination_forward_index_ = value;
}

uint64_t osmway_get_destination_forward_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_forward_index_;
   return s[index].destination_forward_index_;
}

void osmway_set_destination_backward_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_backward_index_;
   s[index].destination_backward_index_ = value;
}

uint64_t osmway_get_destination_backward_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_backward_index_;
   return s[index].destination_backward_index_;
}

void osmway_set_destination_forward_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_forward_lang_index_;
   s[index].destination_forward_lang_index_ = value;
}

uint64_t osmway_get_destination_forward_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_forward_lang_index_;
   return s[index].destination_forward_lang_index_;
}

void osmway_set_destination_backward_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_backward_lang_index_;
   s[index].destination_backward_lang_index_ = value;
}

uint64_t osmway_get_destination_backward_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_backward_lang_index_;
   return s[index].destination_backward_lang_index_;
}

void osmway_set_destination_ref_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_ref_index_;
   s[index].destination_ref_index_ = value;
}

uint64_t osmway_get_destination_ref_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_ref_index_;
   return s[index].destination_ref_index_;
}

void osmway_set_destination_ref_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_ref_lang_index_;
   s[index].destination_ref_lang_index_ = value;
}

uint64_t osmway_get_destination_ref_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_ref_lang_index_;
   return s[index].destination_ref_lang_index_;
}

void osmway_set_destination_ref_to_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_ref_to_index_;
   s[index].destination_ref_to_index_ = value;
}

uint64_t osmway_get_destination_ref_to_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_ref_to_index_;
   return s[index].destination_ref_to_index_;
}

void osmway_set_destination_ref_to_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_ref_to_lang_index_;
   s[index].destination_ref_to_lang_index_ = value;
}

uint64_t osmway_get_destination_ref_to_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_ref_to_lang_index_;
   return s[index].destination_ref_to_lang_index_;
}

void osmway_set_destination_int_ref_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_int_ref_index_;
   s[index].destination_int_ref_index_ = value;
}

uint64_t osmway_get_destination_int_ref_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_int_ref_index_;
   return s[index].destination_int_ref_index_;
}

void osmway_set_destination_int_ref_to_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_int_ref_to_index_;
   s[index].destination_int_ref_to_index_ = value;
}

uint64_t osmway_get_destination_int_ref_to_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_int_ref_to_index_;
   return s[index].destination_int_ref_to_index_;
}

void osmway_set_destination_street_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_street_index_;
   s[index].destination_street_index_ = value;
}

uint64_t osmway_get_destination_street_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_street_index_;
   return s[index].destination_street_index_;
}

void osmway_set_destination_street_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_street_lang_index_;
   s[index].destination_street_lang_index_ = value;
}

uint64_t osmway_get_destination_street_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_street_lang_index_;
   return s[index].destination_street_lang_index_;
}

void osmway_set_destination_street_to_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_street_to_index_;
   s[index].destination_street_to_index_ = value;
}

uint64_t osmway_get_destination_street_to_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_street_to_index_;
   return s[index].destination_street_to_index_;
}

void osmway_set_destination_street_to_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_street_to_lang_index_;
   s[index].destination_street_to_lang_index_ = value;
}

uint64_t osmway_get_destination_street_to_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_street_to_lang_index_;
   return s[index].destination_street_to_lang_index_;
}

void osmway_set_junction_name_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t junction_name_index_;
   s[index].junction_name_index_ = value;
}

uint64_t osmway_get_junction_name_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t junction_name_index_;
   return s[index].junction_name_index_;
}

void osmway_set_junction_name_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t junction_name_lang_index_;
   s[index].junction_name_lang_index_ = value;
}

uint64_t osmway_get_junction_name_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t junction_name_lang_index_;
   return s[index].junction_name_lang_index_;
}

void osmway_set_junction_ref_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t junction_ref_index_;
   s[index].junction_ref_index_ = value;
}

uint64_t osmway_get_junction_ref_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t junction_ref_index_;
   return s[index].junction_ref_index_;
}

void osmway_set_junction_ref_lang_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t junction_ref_lang_index_;
   s[index].junction_ref_lang_index_ = value;
}

uint64_t osmway_get_junction_ref_lang_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t junction_ref_lang_index_;
   return s[index].junction_ref_lang_index_;
}

void osmway_set_level_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t level_index_;
   s[index].level_index_ = value;
}

uint64_t osmway_get_level_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t level_index_;
   return s[index].level_index_;
}

void osmway_set_level_ref_index_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t level_ref_index_;
   s[index].level_ref_index_ = value;
}

uint64_t osmway_get_level_ref_index_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t level_ref_index_;
   return s[index].level_ref_index_;
}

void osmway_set_duration_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t duration_;
   s[index].duration_ = value;
}

uint64_t osmway_get_duration_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t duration_;
   return s[index].duration_;
}

void osmway_set_destination_only_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t destination_only_ : 1;
   s[index].destination_only_ = value;
}

uint64_t osmway_get_destination_only_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t destination_only_ : 1;
   return s[index].destination_only_;
}

void osmway_set_no_thru_traffic_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t no_thru_traffic_ : 1;
   s[index].no_thru_traffic_ = value;
}

uint64_t osmway_get_no_thru_traffic_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t no_thru_traffic_ : 1;
   return s[index].no_thru_traffic_;
}

void osmway_set_oneway_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t oneway_ : 1;
   s[index].oneway_ = value;
}

uint64_t osmway_get_oneway_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t oneway_ : 1;
   return s[index].oneway_;
}

void osmway_set_oneway_reverse_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t oneway_reverse_ : 1;
   s[index].oneway_reverse_ = value;
}

uint64_t osmway_get_oneway_reverse_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t oneway_reverse_ : 1;
   return s[index].oneway_reverse_;
}

void osmway_set_roundabout_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t roundabout_ : 1;
   s[index].roundabout_ = value;
}

uint64_t osmway_get_roundabout_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t roundabout_ : 1;
   return s[index].roundabout_;
}

void osmway_set_ferry_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t ferry_ : 1;
   s[index].ferry_ = value;
}

uint64_t osmway_get_ferry_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t ferry_ : 1;
   return s[index].ferry_;
}

void osmway_set_rail_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t rail_ : 1;
   s[index].rail_ = value;
}

uint64_t osmway_get_rail_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t rail_ : 1;
   return s[index].rail_;
}

void osmway_set_surface_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t surface_ : 3;
   s[index].surface_ = value;
}

uint64_t osmway_get_surface_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t surface_ : 3;
   return s[index].surface_;
}

void osmway_set_tunnel_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tunnel_ : 1;
   s[index].tunnel_ = value;
}

uint64_t osmway_get_tunnel_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tunnel_ : 1;
   return s[index].tunnel_;
}

void osmway_set_toll_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t toll_ : 1;
   s[index].toll_ = value;
}

uint64_t osmway_get_toll_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t toll_ : 1;
   return s[index].toll_;
}

void osmway_set_bridge_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t bridge_ : 1;
   s[index].bridge_ = value;
}

uint64_t osmway_get_bridge_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t bridge_ : 1;
   return s[index].bridge_;
}

void osmway_set_seasonal_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t seasonal_ : 1;
   s[index].seasonal_ = value;
}

uint64_t osmway_get_seasonal_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t seasonal_ : 1;
   return s[index].seasonal_;
}

void osmway_set_drive_on_right_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t drive_on_right_ : 1;
   s[index].drive_on_right_ = value;
}

uint64_t osmway_get_drive_on_right_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t drive_on_right_ : 1;
   return s[index].drive_on_right_;
}

void osmway_set_bike_network_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t bike_network_ : 4;
   s[index].bike_network_ = value;
}

uint64_t osmway_get_bike_network_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t bike_network_ : 4;
   return s[index].bike_network_;
}

void osmway_set_exit_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t exit_ : 1;
   s[index].exit_ = value;
}

uint64_t osmway_get_exit_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t exit_ : 1;
   return s[index].exit_;
}

void osmway_set_tagged_speed_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tagged_speed_ : 1;
   s[index].tagged_speed_ = value;
}

uint64_t osmway_get_tagged_speed_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tagged_speed_ : 1;
   return s[index].tagged_speed_;
}

void osmway_set_forward_tagged_speed_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t forward_tagged_speed_ : 1;
   s[index].forward_tagged_speed_ = value;
}

uint64_t osmway_get_forward_tagged_speed_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t forward_tagged_speed_ : 1;
   return s[index].forward_tagged_speed_;
}

void osmway_set_backward_tagged_speed_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t backward_tagged_speed_ : 1;
   s[index].backward_tagged_speed_ = value;
}

uint64_t osmway_get_backward_tagged_speed_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t backward_tagged_speed_ : 1;
   return s[index].backward_tagged_speed_;
}

void osmway_set_tagged_lanes_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t tagged_lanes_ : 1;
   s[index].tagged_lanes_ = value;
}

uint64_t osmway_get_tagged_lanes_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t tagged_lanes_ : 1;
   return s[index].tagged_lanes_;
}

void osmway_set_forward_tagged_lanes_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t forward_tagged_lanes_ : 1;
   s[index].forward_tagged_lanes_ = value;
}

uint64_t osmway_get_forward_tagged_lanes_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t forward_tagged_lanes_ : 1;
   return s[index].forward_tagged_lanes_;
}

void osmway_set_backward_tagged_lanes_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t backward_tagged_lanes_ : 1;
   s[index].backward_tagged_lanes_ = value;
}

uint64_t osmway_get_backward_tagged_lanes_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t backward_tagged_lanes_ : 1;
   return s[index].backward_tagged_lanes_;
}

void osmway_set_truck_route_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t truck_route_ : 1;
   s[index].truck_route_ = value;
}

uint64_t osmway_get_truck_route_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t truck_route_ : 1;
   return s[index].truck_route_;
}

void osmway_set_sidewalk_right_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t sidewalk_right_ : 1;
   s[index].sidewalk_right_ = value;
}

uint64_t osmway_get_sidewalk_right_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t sidewalk_right_ : 1;
   return s[index].sidewalk_right_;
}

void osmway_set_sidewalk_left_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t sidewalk_left_ : 1;
   s[index].sidewalk_left_ = value;
}

uint64_t osmway_get_sidewalk_left_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t sidewalk_left_ : 1;
   return s[index].sidewalk_left_;
}

void osmway_set_sac_scale_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t sac_scale_ : 3;
   s[index].sac_scale_ = value;
}

uint64_t osmway_get_sac_scale_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t sac_scale_ : 3;
   return s[index].sac_scale_;
}

void osmway_set_road_class_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t road_class_ : 3;
   s[index].road_class_ = value;
}

uint64_t osmway_get_road_class_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t road_class_ : 3;
   return s[index].road_class_;
}

void osmway_set_link_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t link_ : 1;
   s[index].link_ = value;
}

uint64_t osmway_get_link_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t link_ : 1;
   return s[index].link_;
}

void osmway_set_use_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t use_ : 6;
   s[index].use_ = value;
}

uint64_t osmway_get_use_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t use_ : 6;
   return s[index].use_;
}

void osmway_set_lanes_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t lanes_ : 4;
   s[index].lanes_ = value;
}

uint64_t osmway_get_lanes_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t lanes_ : 4;
   return s[index].lanes_;
}

void osmway_set_forward_lanes_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t forward_lanes_ : 4;
   s[index].forward_lanes_ = value;
}

uint64_t osmway_get_forward_lanes_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t forward_lanes_ : 4;
   return s[index].forward_lanes_;
}

void osmway_set_backward_lanes_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t backward_lanes_ : 4;
   s[index].backward_lanes_ = value;
}

uint64_t osmway_get_backward_lanes_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t backward_lanes_ : 4;
   return s[index].backward_lanes_;
}

void osmway_set_turn_channel_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t turn_channel_ : 1;
   s[index].turn_channel_ = value;
}

uint64_t osmway_get_turn_channel_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t turn_channel_ : 1;
   return s[index].turn_channel_;
}

void osmway_set_wheelchair_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t wheelchair_ : 1;
   s[index].wheelchair_ = value;
}

uint64_t osmway_get_wheelchair_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t wheelchair_ : 1;
   return s[index].wheelchair_;
}

void osmway_set_wheelchair_tag_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t wheelchair_tag_ : 1;
   s[index].wheelchair_tag_ = value;
}

uint64_t osmway_get_wheelchair_tag_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t wheelchair_tag_ : 1;
   return s[index].wheelchair_tag_;
}

void osmway_set_has_user_tags_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t has_user_tags_ : 1;
   s[index].has_user_tags_ = value;
}

uint64_t osmway_get_has_user_tags_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t has_user_tags_ : 1;
   return s[index].has_user_tags_;
}

void osmway_set_has_pronunciation_tags_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t has_pronunciation_tags_ : 1;
   s[index].has_pronunciation_tags_ = value;
}

uint64_t osmway_get_has_pronunciation_tags_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t has_pronunciation_tags_ : 1;
   return s[index].has_pronunciation_tags_;
}

void osmway_set_internal_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t internal_ : 1;
   s[index].internal_ = value;
}

uint64_t osmway_get_internal_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t internal_ : 1;
   return s[index].internal_;
}

void osmway_set_hov_type_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t hov_type_ : 1;
   s[index].hov_type_ = value;
}

uint64_t osmway_get_hov_type_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t hov_type_ : 1;
   return s[index].hov_type_;
}

void osmway_set_indoor_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t indoor_ : 1;
   s[index].indoor_ = value;
}

uint64_t osmway_get_indoor_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t indoor_ : 1;
   return s[index].indoor_;
}

void osmway_set_pedestrian_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t pedestrian_forward_ : 1;
   s[index].pedestrian_forward_ = value;
}

uint64_t osmway_get_pedestrian_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t pedestrian_forward_ : 1;
   return s[index].pedestrian_forward_;
}

void osmway_set_pedestrian_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint32_t pedestrian_backward_ : 1;
   s[index].pedestrian_backward_ = value;
}

uint64_t osmway_get_pedestrian_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint32_t pedestrian_backward_ : 1;
   return s[index].pedestrian_backward_;
}

void osmway_set_auto_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t auto_forward_ : 1;
   s[index].auto_forward_ = value;
}

uint64_t osmway_get_auto_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t auto_forward_ : 1;
   return s[index].auto_forward_;
}

void osmway_set_bus_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t bus_forward_ : 1;
   s[index].bus_forward_ = value;
}

uint64_t osmway_get_bus_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t bus_forward_ : 1;
   return s[index].bus_forward_;
}

void osmway_set_taxi_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t taxi_forward_ : 1;
   s[index].taxi_forward_ = value;
}

uint64_t osmway_get_taxi_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t taxi_forward_ : 1;
   return s[index].taxi_forward_;
}

void osmway_set_truck_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t truck_forward_ : 1;
   s[index].truck_forward_ = value;
}

uint64_t osmway_get_truck_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t truck_forward_ : 1;
   return s[index].truck_forward_;
}

void osmway_set_motorcycle_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t motorcycle_forward_ : 1;
   s[index].motorcycle_forward_ = value;
}

uint64_t osmway_get_motorcycle_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t motorcycle_forward_ : 1;
   return s[index].motorcycle_forward_;
}

void osmway_set_emergency_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t emergency_forward_ : 1;
   s[index].emergency_forward_ = value;
}

uint64_t osmway_get_emergency_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t emergency_forward_ : 1;
   return s[index].emergency_forward_;
}

void osmway_set_hov_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t hov_forward_ : 1;
   s[index].hov_forward_ = value;
}

uint64_t osmway_get_hov_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t hov_forward_ : 1;
   return s[index].hov_forward_;
}

void osmway_set_moped_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t moped_forward_ : 1;
   s[index].moped_forward_ = value;
}

uint64_t osmway_get_moped_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t moped_forward_ : 1;
   return s[index].moped_forward_;
}

void osmway_set_auto_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t auto_backward_ : 1;
   s[index].auto_backward_ = value;
}

uint64_t osmway_get_auto_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t auto_backward_ : 1;
   return s[index].auto_backward_;
}

void osmway_set_bus_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t bus_backward_ : 1;
   s[index].bus_backward_ = value;
}

uint64_t osmway_get_bus_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t bus_backward_ : 1;
   return s[index].bus_backward_;
}

void osmway_set_taxi_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t taxi_backward_ : 1;
   s[index].taxi_backward_ = value;
}

uint64_t osmway_get_taxi_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t taxi_backward_ : 1;
   return s[index].taxi_backward_;
}

void osmway_set_truck_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t truck_backward_ : 1;
   s[index].truck_backward_ = value;
}

uint64_t osmway_get_truck_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t truck_backward_ : 1;
   return s[index].truck_backward_;
}

void osmway_set_motorcycle_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t motorcycle_backward_ : 1;
   s[index].motorcycle_backward_ = value;
}

uint64_t osmway_get_motorcycle_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t motorcycle_backward_ : 1;
   return s[index].motorcycle_backward_;
}

void osmway_set_emergency_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t emergency_backward_ : 1;
   s[index].emergency_backward_ = value;
}

uint64_t osmway_get_emergency_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t emergency_backward_ : 1;
   return s[index].emergency_backward_;
}

void osmway_set_hov_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t hov_backward_ : 1;
   s[index].hov_backward_ = value;
}

uint64_t osmway_get_hov_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t hov_backward_ : 1;
   return s[index].hov_backward_;
}

void osmway_set_moped_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t moped_backward_ : 1;
   s[index].moped_backward_ = value;
}

uint64_t osmway_get_moped_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t moped_backward_ : 1;
   return s[index].moped_backward_;
}

void osmway_set_cycle_lane_right_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t cycle_lane_right_ : 2;
   s[index].cycle_lane_right_ = value;
}

uint64_t osmway_get_cycle_lane_right_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t cycle_lane_right_ : 2;
   return s[index].cycle_lane_right_;
}

void osmway_set_cycle_lane_left_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t cycle_lane_left_ : 2;
   s[index].cycle_lane_left_ = value;
}

uint64_t osmway_get_cycle_lane_left_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t cycle_lane_left_ : 2;
   return s[index].cycle_lane_left_;
}

void osmway_set_cycle_lane_right_opposite_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t cycle_lane_right_opposite_ : 1;
   s[index].cycle_lane_right_opposite_ = value;
}

uint64_t osmway_get_cycle_lane_right_opposite_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t cycle_lane_right_opposite_ : 1;
   return s[index].cycle_lane_right_opposite_;
}

void osmway_set_cycle_lane_left_opposite_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t cycle_lane_left_opposite_ : 1;
   s[index].cycle_lane_left_opposite_ = value;
}

uint64_t osmway_get_cycle_lane_left_opposite_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t cycle_lane_left_opposite_ : 1;
   return s[index].cycle_lane_left_opposite_;
}

void osmway_set_shoulder_right_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t shoulder_right_ : 1;
   s[index].shoulder_right_ = value;
}

uint64_t osmway_get_shoulder_right_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t shoulder_right_ : 1;
   return s[index].shoulder_right_;
}

void osmway_set_shoulder_left_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t shoulder_left_ : 1;
   s[index].shoulder_left_ = value;
}

uint64_t osmway_get_shoulder_left_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t shoulder_left_ : 1;
   return s[index].shoulder_left_;
}

void osmway_set_dismount_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t dismount_ : 1;
   s[index].dismount_ = value;
}

uint64_t osmway_get_dismount_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t dismount_ : 1;
   return s[index].dismount_;
}

void osmway_set_use_sidepath_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t use_sidepath_ : 1;
   s[index].use_sidepath_ = value;
}

uint64_t osmway_get_use_sidepath_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t use_sidepath_ : 1;
   return s[index].use_sidepath_;
}

void osmway_set_bike_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t bike_forward_ : 1;
   s[index].bike_forward_ = value;
}

uint64_t osmway_get_bike_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t bike_forward_ : 1;
   return s[index].bike_forward_;
}

void osmway_set_bike_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t bike_backward_ : 1;
   s[index].bike_backward_ = value;
}

uint64_t osmway_get_bike_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t bike_backward_ : 1;
   return s[index].bike_backward_;
}

void osmway_set_lit_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t lit_ : 1;
   s[index].lit_ = value;
}

uint64_t osmway_get_lit_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t lit_ : 1;
   return s[index].lit_;
}

void osmway_set_destination_only_hgv_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t destination_only_hgv_ : 1;
   s[index].destination_only_hgv_ = value;
}

uint64_t osmway_get_destination_only_hgv_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t destination_only_hgv_ : 1;
   return s[index].destination_only_hgv_;
}

void osmway_set_spare2_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t spare2_ : 2;
   s[index].spare2_ = value;
}

uint64_t osmway_get_spare2_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t spare2_ : 2;
   return s[index].spare2_;
}

void osmway_set_nodecount_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint16_t nodecount_;
   s[index].nodecount_ = value;
}

uint64_t osmway_get_nodecount_(const struct OSMWay * s, uint64_t index)
{
   // uint16_t nodecount_;
   return s[index].nodecount_;
}

void osmway_set_speed_limit_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint8_t speed_limit_;
   s[index].speed_limit_ = value;
}

uint64_t osmway_get_speed_limit_(const struct OSMWay * s, uint64_t index)
{
   // uint8_t speed_limit_;
   return s[index].speed_limit_;
}

void osmway_set_speed_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint8_t speed_;
   s[index].speed_ = value;
}

uint64_t osmway_get_speed_(const struct OSMWay * s, uint64_t index)
{
   // uint8_t speed_;
   return s[index].speed_;
}

void osmway_set_backward_speed_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint8_t backward_speed_;
   s[index].backward_speed_ = value;
}

uint64_t osmway_get_backward_speed_(const struct OSMWay * s, uint64_t index)
{
   // uint8_t backward_speed_;
   return s[index].backward_speed_;
}

void osmway_set_forward_speed_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint8_t forward_speed_;
   s[index].forward_speed_ = value;
}

uint64_t osmway_get_forward_speed_(const struct OSMWay * s, uint64_t index)
{
   // uint8_t forward_speed_;
   return s[index].forward_speed_;
}

void osmway_set_truck_speed_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint8_t truck_speed_;
   s[index].truck_speed_ = value;
}

uint64_t osmway_get_truck_speed_(const struct OSMWay * s, uint64_t index)
{
   // uint8_t truck_speed_;
   return s[index].truck_speed_;
}

void osmway_set_truck_speed_forward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint8_t truck_speed_forward_;
   s[index].truck_speed_forward_ = value;
}

uint64_t osmway_get_truck_speed_forward_(const struct OSMWay * s, uint64_t index)
{
   // uint8_t truck_speed_forward_;
   return s[index].truck_speed_forward_;
}

void osmway_set_truck_speed_backward_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // uint8_t truck_speed_backward_;
   s[index].truck_speed_backward_ = value;
}

uint64_t osmway_get_truck_speed_backward_(const struct OSMWay * s, uint64_t index)
{
   // uint8_t truck_speed_backward_;
   return s[index].truck_speed_backward_;
}

void osmway_set_layer_(struct OSMWay * s, uint64_t index, uint64_t value)
{
   // int8_t layer_;
   s[index].layer_ = value;
}

uint64_t osmway_get_layer_(const struct OSMWay * s, uint64_t index)
{
   // int8_t layer_;
   return s[index].layer_;
}

static void print_field(const char * field_name, uint64_t value)
{
   printf("%s: %lu\n", field_name, value);
}

void osmway_dump(const struct OSMWay * s, uint64_t index)
{
   // Dumping fields for OSMWay
   print_field("osmwayid_", (uint64_t)s[index].osmwayid_);
   print_field("ref_index_", (uint64_t)s[index].ref_index_);
   print_field("ref_lang_index_", (uint64_t)s[index].ref_lang_index_);
   print_field("ref_left_index_", (uint64_t)s[index].ref_left_index_);
   print_field("ref_left_lang_index_", (uint64_t)s[index].ref_left_lang_index_);
   print_field("ref_right_index_", (uint64_t)s[index].ref_right_index_);
   print_field("ref_right_lang_index_", (uint64_t)s[index].ref_right_lang_index_);
   print_field("int_ref_index_", (uint64_t)s[index].int_ref_index_);
   print_field("int_ref_lang_index_", (uint64_t)s[index].int_ref_lang_index_);
   print_field("int_ref_left_index_", (uint64_t)s[index].int_ref_left_index_);
   print_field("int_ref_left_lang_index_", (uint64_t)s[index].int_ref_left_lang_index_);
   print_field("int_ref_right_index_", (uint64_t)s[index].int_ref_right_index_);
   print_field("int_ref_right_lang_index_", (uint64_t)s[index].int_ref_right_lang_index_);
   print_field("name_index_", (uint64_t)s[index].name_index_);
   print_field("name_lang_index_", (uint64_t)s[index].name_lang_index_);
   print_field("name_left_index_", (uint64_t)s[index].name_left_index_);
   print_field("name_left_lang_index_", (uint64_t)s[index].name_left_lang_index_);
   print_field("name_right_index_", (uint64_t)s[index].name_right_index_);
   print_field("name_right_lang_index_", (uint64_t)s[index].name_right_lang_index_);
   print_field("name_forward_index_", (uint64_t)s[index].name_forward_index_);
   print_field("name_forward_lang_index_", (uint64_t)s[index].name_forward_lang_index_);
   print_field("name_backward_index_", (uint64_t)s[index].name_backward_index_);
   print_field("name_backward_lang_index_", (uint64_t)s[index].name_backward_lang_index_);
   print_field("alt_name_index_", (uint64_t)s[index].alt_name_index_);
   print_field("alt_name_lang_index_", (uint64_t)s[index].alt_name_lang_index_);
   print_field("alt_name_left_index_", (uint64_t)s[index].alt_name_left_index_);
   print_field("alt_name_left_lang_index_", (uint64_t)s[index].alt_name_left_lang_index_);
   print_field("alt_name_right_index_", (uint64_t)s[index].alt_name_right_index_);
   print_field("alt_name_right_lang_index_", (uint64_t)s[index].alt_name_right_lang_index_);
   print_field("official_name_index_", (uint64_t)s[index].official_name_index_);
   print_field("official_name_lang_index_", (uint64_t)s[index].official_name_lang_index_);
   print_field("official_name_left_index_", (uint64_t)s[index].official_name_left_index_);
   print_field("official_name_left_lang_index_", (uint64_t)s[index].official_name_left_lang_index_);
   print_field("official_name_right_index_", (uint64_t)s[index].official_name_right_index_);
   print_field("official_name_right_lang_index_", (uint64_t)s[index].official_name_right_lang_index_);
   print_field("tunnel_name_index_", (uint64_t)s[index].tunnel_name_index_);
   print_field("tunnel_name_lang_index_", (uint64_t)s[index].tunnel_name_lang_index_);
   print_field("tunnel_name_left_index_", (uint64_t)s[index].tunnel_name_left_index_);
   print_field("tunnel_name_left_lang_index_", (uint64_t)s[index].tunnel_name_left_lang_index_);
   print_field("tunnel_name_right_index_", (uint64_t)s[index].tunnel_name_right_index_);
   print_field("tunnel_name_right_lang_index_", (uint64_t)s[index].tunnel_name_right_lang_index_);
   print_field("fwd_turn_lanes_index_", (uint64_t)s[index].fwd_turn_lanes_index_);
   print_field("bwd_turn_lanes_index_", (uint64_t)s[index].bwd_turn_lanes_index_);
   print_field("fwd_jct_base_index_", (uint64_t)s[index].fwd_jct_base_index_);
   print_field("bwd_jct_base_index_", (uint64_t)s[index].bwd_jct_base_index_);
   print_field("fwd_jct_overlay_index_", (uint64_t)s[index].fwd_jct_overlay_index_);
   print_field("bwd_jct_overlay_index_", (uint64_t)s[index].bwd_jct_overlay_index_);
   print_field("fwd_signboard_base_index_", (uint64_t)s[index].fwd_signboard_base_index_);
   print_field("bwd_signboard_base_index_", (uint64_t)s[index].bwd_signboard_base_index_);
   print_field("destination_index_", (uint64_t)s[index].destination_index_);
   print_field("destination_lang_index_", (uint64_t)s[index].destination_lang_index_);
   print_field("destination_forward_index_", (uint64_t)s[index].destination_forward_index_);
   print_field("destination_backward_index_", (uint64_t)s[index].destination_backward_index_);
   print_field("destination_forward_lang_index_", (uint64_t)s[index].destination_forward_lang_index_);
   print_field("destination_backward_lang_index_", (uint64_t)s[index].destination_backward_lang_index_);
   print_field("destination_ref_index_", (uint64_t)s[index].destination_ref_index_);
   print_field("destination_ref_lang_index_", (uint64_t)s[index].destination_ref_lang_index_);
   print_field("destination_ref_to_index_", (uint64_t)s[index].destination_ref_to_index_);
   print_field("destination_ref_to_lang_index_", (uint64_t)s[index].destination_ref_to_lang_index_);
   print_field("destination_int_ref_index_", (uint64_t)s[index].destination_int_ref_index_);
   print_field("destination_int_ref_to_index_", (uint64_t)s[index].destination_int_ref_to_index_);
   print_field("destination_street_index_", (uint64_t)s[index].destination_street_index_);
   print_field("destination_street_lang_index_", (uint64_t)s[index].destination_street_lang_index_);
   print_field("destination_street_to_index_", (uint64_t)s[index].destination_street_to_index_);
   print_field("destination_street_to_lang_index_", (uint64_t)s[index].destination_street_to_lang_index_);
   print_field("junction_name_index_", (uint64_t)s[index].junction_name_index_);
   print_field("junction_name_lang_index_", (uint64_t)s[index].junction_name_lang_index_);
   print_field("junction_ref_index_", (uint64_t)s[index].junction_ref_index_);
   print_field("junction_ref_lang_index_", (uint64_t)s[index].junction_ref_lang_index_);
   print_field("level_index_", (uint64_t)s[index].level_index_);
   print_field("level_ref_index_", (uint64_t)s[index].level_ref_index_);
   print_field("duration_", (uint64_t)s[index].duration_);
   print_field("destination_only_", (uint64_t)s[index].destination_only_);
   print_field("no_thru_traffic_", (uint64_t)s[index].no_thru_traffic_);
   print_field("oneway_", (uint64_t)s[index].oneway_);
   print_field("oneway_reverse_", (uint64_t)s[index].oneway_reverse_);
   print_field("roundabout_", (uint64_t)s[index].roundabout_);
   print_field("ferry_", (uint64_t)s[index].ferry_);
   print_field("rail_", (uint64_t)s[index].rail_);
   print_field("surface_", (uint64_t)s[index].surface_);
   print_field("tunnel_", (uint64_t)s[index].tunnel_);
   print_field("toll_", (uint64_t)s[index].toll_);
   print_field("bridge_", (uint64_t)s[index].bridge_);
   print_field("seasonal_", (uint64_t)s[index].seasonal_);
   print_field("drive_on_right_", (uint64_t)s[index].drive_on_right_);
   print_field("bike_network_", (uint64_t)s[index].bike_network_);
   print_field("exit_", (uint64_t)s[index].exit_);
   print_field("tagged_speed_", (uint64_t)s[index].tagged_speed_);
   print_field("forward_tagged_speed_", (uint64_t)s[index].forward_tagged_speed_);
   print_field("backward_tagged_speed_", (uint64_t)s[index].backward_tagged_speed_);
   print_field("tagged_lanes_", (uint64_t)s[index].tagged_lanes_);
   print_field("forward_tagged_lanes_", (uint64_t)s[index].forward_tagged_lanes_);
   print_field("backward_tagged_lanes_", (uint64_t)s[index].backward_tagged_lanes_);
   print_field("truck_route_", (uint64_t)s[index].truck_route_);
   print_field("sidewalk_right_", (uint64_t)s[index].sidewalk_right_);
   print_field("sidewalk_left_", (uint64_t)s[index].sidewalk_left_);
   print_field("sac_scale_", (uint64_t)s[index].sac_scale_);
   print_field("road_class_", (uint64_t)s[index].road_class_);
   print_field("link_", (uint64_t)s[index].link_);
   print_field("use_", (uint64_t)s[index].use_);
   print_field("lanes_", (uint64_t)s[index].lanes_);
   print_field("forward_lanes_", (uint64_t)s[index].forward_lanes_);
   print_field("backward_lanes_", (uint64_t)s[index].backward_lanes_);
   print_field("turn_channel_", (uint64_t)s[index].turn_channel_);
   print_field("wheelchair_", (uint64_t)s[index].wheelchair_);
   print_field("wheelchair_tag_", (uint64_t)s[index].wheelchair_tag_);
   print_field("has_user_tags_", (uint64_t)s[index].has_user_tags_);
   print_field("has_pronunciation_tags_", (uint64_t)s[index].has_pronunciation_tags_);
   print_field("internal_", (uint64_t)s[index].internal_);
   print_field("hov_type_", (uint64_t)s[index].hov_type_);
   print_field("indoor_", (uint64_t)s[index].indoor_);
   print_field("pedestrian_forward_", (uint64_t)s[index].pedestrian_forward_);
   print_field("pedestrian_backward_", (uint64_t)s[index].pedestrian_backward_);
   print_field("auto_forward_", (uint64_t)s[index].auto_forward_);
   print_field("bus_forward_", (uint64_t)s[index].bus_forward_);
   print_field("taxi_forward_", (uint64_t)s[index].taxi_forward_);
   print_field("truck_forward_", (uint64_t)s[index].truck_forward_);
   print_field("motorcycle_forward_", (uint64_t)s[index].motorcycle_forward_);
   print_field("emergency_forward_", (uint64_t)s[index].emergency_forward_);
   print_field("hov_forward_", (uint64_t)s[index].hov_forward_);
   print_field("moped_forward_", (uint64_t)s[index].moped_forward_);
   print_field("auto_backward_", (uint64_t)s[index].auto_backward_);
   print_field("bus_backward_", (uint64_t)s[index].bus_backward_);
   print_field("taxi_backward_", (uint64_t)s[index].taxi_backward_);
   print_field("truck_backward_", (uint64_t)s[index].truck_backward_);
   print_field("motorcycle_backward_", (uint64_t)s[index].motorcycle_backward_);
   print_field("emergency_backward_", (uint64_t)s[index].emergency_backward_);
   print_field("hov_backward_", (uint64_t)s[index].hov_backward_);
   print_field("moped_backward_", (uint64_t)s[index].moped_backward_);
   print_field("cycle_lane_right_", (uint64_t)s[index].cycle_lane_right_);
   print_field("cycle_lane_left_", (uint64_t)s[index].cycle_lane_left_);
   print_field("cycle_lane_right_opposite_", (uint64_t)s[index].cycle_lane_right_opposite_);
   print_field("cycle_lane_left_opposite_", (uint64_t)s[index].cycle_lane_left_opposite_);
   print_field("shoulder_right_", (uint64_t)s[index].shoulder_right_);
   print_field("shoulder_left_", (uint64_t)s[index].shoulder_left_);
   print_field("dismount_", (uint64_t)s[index].dismount_);
   print_field("use_sidepath_", (uint64_t)s[index].use_sidepath_);
   print_field("bike_forward_", (uint64_t)s[index].bike_forward_);
   print_field("bike_backward_", (uint64_t)s[index].bike_backward_);
   print_field("lit_", (uint64_t)s[index].lit_);
   print_field("destination_only_hgv_", (uint64_t)s[index].destination_only_hgv_);
   print_field("spare2_", (uint64_t)s[index].spare2_);
   print_field("nodecount_", (uint64_t)s[index].nodecount_);
   print_field("speed_limit_", (uint64_t)s[index].speed_limit_);
   print_field("speed_", (uint64_t)s[index].speed_);
   print_field("backward_speed_", (uint64_t)s[index].backward_speed_);
   print_field("forward_speed_", (uint64_t)s[index].forward_speed_);
   print_field("truck_speed_", (uint64_t)s[index].truck_speed_);
   print_field("truck_speed_forward_", (uint64_t)s[index].truck_speed_forward_);
   print_field("truck_speed_backward_", (uint64_t)s[index].truck_speed_backward_);
   print_field("layer_", (uint64_t)s[index].layer_);
}

