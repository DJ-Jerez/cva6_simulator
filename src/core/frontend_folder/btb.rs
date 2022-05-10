use crate::core::include::ariane_pkg::{
    btb_update_t,
    btb_prediction_t
};

const NR_ENTRIES: u64 = 8;
const NR_ROWS: u64 = NR_ENTRIES;
const OFFSET: u64 = 1;
const ROW_ADDR_BITS: u64 = 1;
const ROW_INDEX_BITS: u64 = 1;
const PREDICTION_BITS: u64 = 3 + OFFSET + ROW_ADDR_BITS; //3 is clog2(NR_ROWS)
const ANTIALIASING_BITS: u64 = 8;

pub struct btb{
    btb_d: [btb_prediction_t; NR_ROWS as usize],
    btb_q: [btb_prediction_t; NR_ROWS as usize],


}

impl btb {
    //  implement copy trait on btb_prediction_t before uncommenting
    // pub fn new() -> btb {
    //     let init_bpt: btb_prediction_t = btb_prediction_t::new(false, 0);
    //     btb {
    //         btb_d: [init_bpt; NR_ROWS as usize],
    //         btb_q: [init_bpt; NR_ROWS as usize]
    //     }
    // }

    pub fn tick(reset_ni: bool, flush_i: bool, debug_mode_i: bool, vpc_i: u64, btb_update_i: btb_update_t) -> btb_prediction_t {
        //stub
        btb_prediction_t::new(false, 0)

        //make sure to use mutable references when updating btb
    }

}
