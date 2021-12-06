use crate::core::include::ariane_pkg::fu_data_t;
use crate::core::include::ariane_pkg::fu_op;
//adder lul (returns the result and alu_branch_res_o)
pub fn tick (fu_data_i: fu_data_t) -> (u64, bool) {
    let mut result: u64 = 0;
    let mut branch_res_op: bool = true;

    match &fu_data_i.operator {
        fu_op::ADD => {result = add(&fu_data_i)},
        fu_op::SUB => {result = sub(&fu_data_i)},
        fu_op::ADDW => {result = addw(&fu_data_i) as u64},
        fu_op::SUBW => {result = subw(&fu_data_i) as u64},

        // logic operations
        fu_op::XORL => {result = xorl(&fu_data_i)},
        fu_op::ORL => {result = orl(&fu_data_i)},
        fu_op::ANDL => {result = andl(&fu_data_i)},

        // shifts
        fu_op::SRA => {},
        fu_op::SRL => {result = srl(&fu_data_i)},
        fu_op::SLL => {result = sll(&fu_data_i)},
        fu_op::SRAW => {},
        fu_op::SRLW => {},
        fu_op::SLLW => {},

        // comparisons
        fu_op::LTS => {branch_res_op = lts(&fu_data_i)},
        fu_op::LTU => {branch_res_op = ltu(&fu_data_i)},
        fu_op::GES => {branch_res_op = !lts(&fu_data_i)},
        fu_op::GEU => {branch_res_op = !ltu(&fu_data_i)},
        fu_op::EQ => {branch_res_op = eq(&fu_data_i)},
        fu_op::NE => {branch_res_op = !eq(&fu_data_i)},

        // jumps
        fu_op::JALR => {}, //jump and link return; return current instruction + i
        fu_op::BRANCH => {},

        // set lower than operations
        fu_op::SLTS => {},
        fu_op::SLTU => {},
    }
    (result, branch_res_op)
}


// pub fn tick () -> (u64, bool) { //stub
//
//     (5, true)
// }

//-----------------------------------------------------//
//          Basic Arithmetic
//-----------------------------------------------------//

fn add(fu_data_i: &fu_data_t) -> u64 {  //TODO: handle sum overflow
    fu_data_i.get_operand_a() + fu_data_i.get_operand_b()
}

fn sub(fu_data_i: &fu_data_t) -> u64 { //handles negative result case with two's compliment overflow
    let op_a: u64 = fu_data_i.get_operand_a();
    let op_b: u64 = fu_data_i.get_operand_b();
    let b_comp: u64 = u64::MAX - op_b + 1;

    //if op_a is larger than op_b, proceed as normal
    if op_a >= op_b {
        op_a - op_b
    }
    //if op_b is larger than op_a, add op_b's two's compliment instead
    else {
        op_a + b_comp
    }
}

fn addw(fu_data_i: &fu_data_t) -> u32 {
    let word_a = fu_data_i.get_operand_a() as u32;
    let word_b = fu_data_i.get_operand_b() as u32;
    word_a + word_b
}

fn subw(fu_data_i: &fu_data_t) -> u32 {
    let word_a = fu_data_i.get_operand_a() as u32;
    let word_b = fu_data_i.get_operand_b() as u32;
    let b_comp: u32 = u32::MAX - word_b + 1;

    //same jazz as in sub, but with 32 bit unsigned ints
    if word_a >= word_b {
        word_a - word_b
    }
    else {
        word_a + b_comp
    }
}

//-----------------------------------------------------//
//          Logic Operations
//-----------------------------------------------------//

fn xorl(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() ^ fu_data_i.get_operand_b()
}

fn orl(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() | fu_data_i.get_operand_b()
}

fn andl(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() & fu_data_i.get_operand_b()
}

//-----------------------------------------------------//
//                      Shifts
//-----------------------------------------------------//

fn sll(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() << fu_data_i.get_operand_b()
}
//arithmetic shifts copy the initial leftmost bit and append them to the end; logical shifts always append 0

fn srl(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() >> fu_data_i.get_operand_b()
}

//-----------------------------------------------------//
//                    Comparisons
//-----------------------------------------------------//

fn eq(fu_data_i: &fu_data_t) -> bool {
    let result = sub(&fu_data_i);
    result == 0
}

fn ltu(fu_data_i: &fu_data_t) -> bool {
    fu_data_i.get_operand_a() < fu_data_i.get_operand_b()
}


fn lts(fu_data_i: &fu_data_t) -> bool {
    const NEGATIVE_THRESHOLD :u64 = u64::MAX >> 1;
    let a_neg: bool = fu_data_i.get_operand_a() >= NEGATIVE_THRESHOLD;
    let b_neg: bool = fu_data_i.get_operand_b() >= NEGATIVE_THRESHOLD;

    if (a_neg && !b_neg){
        true
    } else if (!a_neg && b_neg){
        false
    }else{
        fu_data_i.get_operand_a() < fu_data_i.get_operand_b()
    }
}
//-----------------------------------------------------//
//                      Jumps
//-----------------------------------------------------//
