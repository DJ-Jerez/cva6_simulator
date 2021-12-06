//run cargo build and cargo run with 2>/dev/null to supress errors and warnings for that run

#![allow(non_snake_case)]
mod core;

use crate::core::{
    alu,
    include::ariane_pkg,
    include::ariane_pkg::fu_data_t
};

fn main() {
    let fu_op_a = ariane_pkg::fu_op::ADD;
    let opers = fu_data_t::new(fu_op_a, 3, 2, 0);
    //println!("opers: {:?}", opers);

    let data: (u64, bool) = alu::tick(opers);

    println!("{:?}", data);
}
