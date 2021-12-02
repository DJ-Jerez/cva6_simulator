#![allow(non_snake_case)]
mod core;

use crate::core::alu;

fn main() {
    println!("Hello, world!");
    let data: (u64, bool) = alu::tick();
    println!("{:?}", data);
}
