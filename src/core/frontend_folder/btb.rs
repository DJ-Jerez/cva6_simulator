#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::core::include::ariane_pkg::{
    btb_update_t,
    btb_prediction_t
};

const NR_ENTRIES: u64 = 8;
const NR_ROWS: u64 = NR_ENTRIES;
const OFFSET: u64 = 1;
const ROW_ADDR_BITS: u64 = 0;
const ROW_INDEX_BITS: u64 = 1;
const PREDICTION_BITS: u64 = 3 + OFFSET + ROW_ADDR_BITS; //3 is clog2(NR_ROWS)
const ANTIALIASING_BITS: u64 = 8;

pub struct btb{
    btb_d: [btb_prediction_t; NR_ROWS as usize], //input; could just update q instead
    btb_q: [btb_prediction_t; NR_ROWS as usize], //output


}

impl btb {
    //  implement copy trait on btb_prediction_t before uncommenting
    pub fn new() -> btb {
        let init_bpt: btb_prediction_t = btb_prediction_t::new(false, 0);
        btb {
            btb_d: [init_bpt; NR_ROWS as usize],
            btb_q: [init_bpt; NR_ROWS as usize]
        }
    }

    pub fn update(&mut self, btb_update_i: &btb_update_t, update_pc: u64) {
        println!("Update:");
        if btb_update_i.valid {
            self.btb_d[update_pc as usize].valid = true;
            self.btb_d[update_pc as usize].target_address = btb_update_i.target_address;
            println!("Valid: {0}, Target Address: {1}", self.btb_d[update_pc as usize].valid, self.btb_d[update_pc as usize].target_address);
        }
    }

    pub fn tick(&mut self, reset_ni: bool, flush_i: bool, debug_mode_i: bool, vpc_i: u64, btb_update_i: btb_update_t) -> btb_prediction_t {
        // workaround; grabbing only the bits used for indexing the btb
        let index: u64 = (vpc_i % (1 << PREDICTION_BITS)) >> (ROW_ADDR_BITS + OFFSET);
        println!("Index: {}", index);
        println!("Prediction bits: {}", PREDICTION_BITS);
        println!("ROW_ADDR_BITS + OFFSET: {}", ROW_ADDR_BITS + OFFSET);
        let update_pc: u64 = (btb_update_i.pc % (1 << PREDICTION_BITS)) >> (ROW_ADDR_BITS + OFFSET);
        println!("Update pc: {}", update_pc);

        //update the table on mis-predict
        self.update(&btb_update_i, update_pc);

        //do sequential operation
        if !reset_ni {
            for mut row in self.btb_q {
                row.valid = false;
                row.target_address = 0;
            }
        }
        else {
            if flush_i {
                for mut row in self.btb_q {
                    row.valid = false;
                }
            } else {
                self.btb_q = self.btb_d;
                self.btb_d = [btb_prediction_t::new(false, 0); NR_ROWS as usize]

            }
        }

        //give prediction
        self.btb_q[index as usize]

        //stub
        // btb_prediction_t::new(false, 0)

        //make sure to use mutable references when updating btb
    }

}
