use crate::core::include::ariane_pkg::fu_data_t;
use crate::core::include::ariane_pkg::fu_op;
//adder lul (returns the result and alu_branch_res_o)
pub fn tick (fu_data_i: fu_data_t) -> (u64, bool) {
    let mut result: u64 = 0;

    // if fu_data_i.operator == fu_op::ADD {
    //     result = add(&fu_data_i);
    // }

    match fu_data_i.operator {
        fu_op::ADD => {result = add(&fu_data_i)},
        SUB => {},
        ADDW => {},
        SUBW => {},

        // logic operations
        XORL => {},
        ORL => {},
        ANDL => {},

        // shifts
        SRA => {},
        SRL => {},
        SLL => {},
        SRLW => {},
        SLLW => {},
        SRAW => {},

        // comparisons
        LTS => {},
        LTU => {},
        GES => {},
        GEU => {},
        EQ => {},
        NE => {},

        // jumps
        JALR => {},
        BRANCH => {},

        // set lower than operations
        SLTS => {},
        SLTU => {},
    }
    (result, false)
}


// pub fn tick () -> (u64, bool) { //stub
//
//     (5, true)
// }

fn add (fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() + fu_data_i.get_operand_b()
}
