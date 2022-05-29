#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::core::include::ariane_pkg::bht_update_t;
use crate::core::include::ariane_pkg::bht_prediction_t;
use crate::core::include::ariane_pkg::INSTR_PER_FETCH;
use crate::core::include::ariane_pkg::RVC;

const NR_ENTRIES: u64 = 1024;
const OFFSET: u64 = if RVC { 1 } else { 2 };
const NR_ROWS: u64 = NR_ENTRIES / INSTR_PER_FETCH as u64;
const ROW_ADDR_BITS: u64  = 0; //should be 0; used to index the row
const ROW_INDEX_BITS: u64 = 0;
const PREDICTION_BITS: u64 = NR_ROWS + OFFSET + ROW_ADDR_BITS;

const fn num_bits<T>() -> usize { std::mem::size_of::<T>()  * 8 }

// This function returns the ceiling of a log base 2 number
pub fn clog2(x: &u64) -> u64 { 
    assert!(x > &0);
    let ans = num_bits::<u64>() as u64 - x.leading_zeros() as u64 - 1;
    let y = x << x.leading_zeros() + 1;

    if y == 0 {
        return ans;
    }
    else {
        return ans + 1;
    }
}


pub struct bht_row {
    pub valid: bool,
    pub saturation_counter: [u8; 2]
}

impl bht_row {
    pub fn set(mut self, valid: bool, saturation_counter: [u8; 2])  {
        self.valid = valid;
        self.saturation_counter = saturation_counter;
    }
}


pub struct bht {
    bht_d: [bht_row; NR_ROWS as usize], //can be local (doesn't need to preserve state)
    bht_q: [bht_row; NR_ROWS as usize]
}
impl bht {
    pub fn tick (self, reset_ni: bool, flush_i: bool, debug_mode_i: bool, vpc_i: u64, bht_update_i: bht_update_t) -> bht_prediction_t{
        let NR_ROWS_log = log_2(&NR_ROWS);
        let index = NR_ROWS_log - 1;
        let update_pc = NR_ROWS_log - 1;
        let update_row_index: bool; //assuming ROW_INDEX_BITS is set to 1
        let saturation_counter: [bool;2];


        let index = vpc_i[PREDICTION_BITS - 1:ROW_ADDR_BITS + OFFSET];
        let update_pc = bht_update_i.pc[PREDICTION_BITS - 1:ROW_ADDR_BITS + OFFSET];
        let mut x: f32= 5.5;
        x.log10();
    }
}
