#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::core::include::ariane_pkg::bht_update_t;
use crate::core::include::ariane_pkg::bht_prediction_t;
use crate::core::include::ariane_pkg::INSTR_PER_FETCH;
use crate::core::include::ariane_pkg::RVC;

// This function returns the ceiling of a log base 2 number
const fn num_bits<T>() -> usize { std::mem::size_of::<T>()  * 8 }

pub fn clog2(x: &u64) -> u64 { 
    assert!(x > &0);
    let ans = num_bits::<u64>() as u64 - x.leading_zeros() as u64 - 1;
    let y = x << x.leading_zeros() + 1;

    if y == 0 {
        ans
    }
    else {
        ans + 1
    }
}


const NR_ENTRIES: u64 = 1024;
const OFFSET: u64 = if RVC { 1 } else { 2 };
const NR_ROWS: u64 = NR_ENTRIES / INSTR_PER_FETCH as u64;
const ROW_ADDR_BITS: u64  = 0; //clog2(&(INSTR_PER_FETCH as u64));
const ROW_INDEX_BITS: u64 = if RVC { 0 /*clog2(&(INSTR_PER_FETCH as u64))*/ } else { 1 };
const PREDICTION_BITS: u64 = 10 + OFFSET + ROW_ADDR_BITS; //10 is clog2(NR_ROWS)

#[derive(Copy, Clone)]
pub struct bht_row {
    pub valid: bool,
    pub saturation_counter: u8,
}

impl bht_row {
    pub fn new(valid: bool, saturation_counter: u8) -> bht_row {
        bht_row {
            valid,
            saturation_counter
        }
    }
}


pub struct bht {
    bht_d: [bht_row; NR_ROWS as usize], //can be local (doesn't need to preserve state)
    bht_q: [bht_row; NR_ROWS as usize]
}

impl bht {
    pub fn new() -> bht {
        let init_bpt: bht_row = bht_row::new(false, 0);
        bht {
            bht_d: [init_bpt; NR_ROWS as usize],
            bht_q: [init_bpt; NR_ROWS as usize]
        }
    }

    pub fn update(&mut self, bht_update_i: &bht_update_t, update_pc: u64, debug_mode_i: bool) {
        self.bht_d = self.bht_q;
        let sat_counter = self.bht_q[update_pc as usize].saturation_counter;

        println!("Update:");
        if bht_update_i.valid && !debug_mode_i {
            self.bht_d[update_pc as usize].valid = true;

            if sat_counter == 2 {
                if !bht_update_i.taken {
                    self.bht_d[update_pc as usize].saturation_counter = sat_counter - 1;
                }
            }
            else if sat_counter == 0 {
                if bht_update_i.taken {
                    self.bht_d[update_pc as usize].saturation_counter = sat_counter + 1;
                }
            }
            else {
                if bht_update_i.taken {
                    self.bht_d[update_pc as usize].saturation_counter = sat_counter + 1;
                }
                else {
                    self.bht_d[update_pc as usize].saturation_counter = sat_counter - 1;
                }
            }
        }
        println!("Valid: {0}, Saturation Counter: {1}", self.bht_d[update_pc as usize].valid, self.bht_d[update_pc as usize].saturation_counter);

    }

    pub fn tick (&mut self, reset_ni: bool, flush_i: bool, debug_mode_i: bool, vpc_i: u64, bht_update_i: bht_update_t) -> bht_prediction_t{
        let index: u64 = (vpc_i % (1 << PREDICTION_BITS)) >> (ROW_ADDR_BITS + OFFSET);
        println!("Index: {}", index);
        println!("Prediction bits: {}", PREDICTION_BITS);
        println!("ROW_ADDR_BITS + OFFSET: {}", ROW_ADDR_BITS + OFFSET);
        let update_pc: u64 = (bht_update_i.pc % (1 << PREDICTION_BITS)) >> (ROW_ADDR_BITS + OFFSET);
        println!("Update pc: {}", update_pc);
    
        self.update(&bht_update_i, update_pc, debug_mode_i);

        if !reset_ni {
            for i in 0..NR_ROWS { //not sure out to implement update_row_index with current structure
                    self.bht_q[i as usize].valid = false;
                    self.bht_q[i as usize].saturation_counter = 0; //confused on SV what <= '0 means
            }
        }
        else {
            if flush_i {
                for i in 0..NR_ROWS {
                    self.bht_d[i as usize].valid = false;
                    self.bht_d[i as usize].saturation_counter = 2;
                }
            }
        }
        self.bht_q = self.bht_d;
        bht_prediction_t::new(false, false) //confused what to return
    }
}
