use crate::core::include::ariane_pkg::fu_data_t;
use crate::core::include::ariane_pkg::fu_op;
//adder lul (returns the result and alu_branch_res_o)
pub fn tick (fu_data_i: fu_data_t) -> (u64, bool) {
    let mut result: u64 = 0;

    // if fu_data_i.operator == fu_op::ADD {
    //     result = add(&fu_data_i);
    // }

    match &fu_data_i.operator {
        fu_op::ADD => {result = add(&fu_data_i)},
        fu_op::SUB => {result = sub(&fu_data_i)},
        fu_op::ADDW => {},
        fu_op::SUBW => {},

        // logic operations
        fu_op::XORL => {result = xorl(&fu_data_i)},
        fu_op::ORL => {},
        fu_op::ANDL => {},

        // shifts
        fu_op::SRA => {},
        fu_op::SRL => {},
        fu_op::SLL => {},
        fu_op::SRLW => {},
        fu_op::SLLW => {},
        fu_op::SRAW => {},

        // comparisons
        fu_op::LTS => {},
        fu_op::LTU => {},
        fu_op::GES => {},
        fu_op::GEU => {},
        fu_op::EQ => {},
        fu_op::NE => {},

        // jumps
        fu_op::JALR => {},
        fu_op::BRANCH => {},

        // set lower than operations
        fu_op::SLTS => {},
        fu_op::SLTU => {},
    }
    (result, false)
}


// pub fn tick () -> (u64, bool) { //stub
//
//     (5, true)
// }

fn add(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() + fu_data_i.get_operand_b()
}

fn sub(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() - fu_data_i.get_operand_b()
    // 69
}

fn xorl(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() ^ fu_data_i.get_operand_b()
}
