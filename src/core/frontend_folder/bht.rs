use crate::core::include::ariane_pkg::bht_update_t;
use crate::core::include::ariane_pkg::bht_prediction_t;
use crate::core::include::ariane_pkg::INSTR_PER_FETCH;

const NR_ENTRIES :u64 = 1024;

pub struct bht_row {
    pub valid: bool,
    pub saturation_counter: [bool; 2]
}

impl bht_row {
    pub fn set(mut self, valid: bool, saturation_counter: [bool;2])  {
        self.valid = valid;
        self.saturation_counter = saturation_counter;
    }
}


pub fn tick (reset_ni: bool, flush_i: bool, debug_mode_i: bool, vpc_i: u64, bht_update_i: bht_update_t) -> bht_prediction_t{
    const OFFSET: u64 = 1;
    const NR_ROWS: u64 = NR_ENTRIES / INSTR_PER_FETCH as u64;
    const ROW_ADDR_BITS: u64  = 1; //not sure to set at 0 or 1 since log of 1 is 0
    const ROW_INDEX_BITS: u64 = 1;
    const  PREDICTION_BITS: u64 = NR_ROWS + OFFSET + ROW_ADDR_BITS;
    let mut bht_d: [bht_row; NR_ROWS];
    let mut bht_q: [bht_row; NR_ROWS];

    //haven't implemented index or update_pc
    let update_row_index: bool = 
}

