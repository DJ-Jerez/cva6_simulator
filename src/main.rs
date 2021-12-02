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
    let data: (u64, bool) = alu::tick(opers);

    println!("{:?}", data);
}
