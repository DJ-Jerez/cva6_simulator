#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::core::include::ariane_pkg::bht_update_t;
use crate::core::include::ariane_pkg::bht_prediction_t;
use crate::core::include::ariane_pkg::INSTR_PER_FETCH;

const NR_ENTRIES: u64 = 1024;
const OFFSET: u64 = 1;
const NR_ROWS: u64 = NR_ENTRIES / INSTR_PER_FETCH as u64;
const ROW_ADDR_BITS: u64  = 1; //not sure to set at 0 or 1 since log of 1 is 0
const ROW_INDEX_BITS: u64 = 1; //questions about this
const PREDICTION_BITS: u64 = NR_ROWS + OFFSET + ROW_ADDR_BITS;

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


pub struct bht {
    bht_d: [bht_row; NR_ROWS as usize],
    bht_q: [bht_row; NR_ROWS as usize]
}
impl bht {
    pub fn tick (self, reset_ni: bool, flush_i: bool, debug_mode_i: bool, vpc_i: u64, bht_update_i: bht_update_t) -> bht_prediction_t{
        // let index: [bool: clog2(NR_ROWS)-1];
        // let update_pc: [bool: clog2(NR_ROWS)-1];
        let update_row_index: bool; //assuming ROW_INDEX_BITS is set to 1
        let saturation_counter: [bool;2];
        // let index = vpc_i[PREDICTION_BITS - 1:ROW_ADDR_BITS + OFFSET];
        // let update_pc = bht_update_i.pc[PREDICTION_BITS - 1:ROW_ADDR_BITS + OFFSET];
    }
}

