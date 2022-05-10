use crate::core::include::ariane_pkg::bht_update_t;
use crate::core::include::ariane_pkg::bht_prediction_t;


pub fn tick (reset_ni: bool, flush_i: bool, debug_mode_i: bool, vpc_i: u64, bht_update_i: bht_update_t) -> bht_prediction_t{
    bht_prediction_t::new(false, false)
}
