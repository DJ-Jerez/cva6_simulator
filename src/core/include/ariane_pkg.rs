// mod riscv_pkgs;

const XLEN: u8 = 64;

pub enum fu_op { //literally copy pasted from ariane_pkg
    // basic ALU op
    ADD, SUB, ADDW, SUBW,
    // logic operations
    XORL, ORL, ANDL,
    // shifts
    SRA, SRL, SLL, SRLW, SLLW, SRAW,
    // comparisons
    LTS, LTU, GES, GEU, EQ, NE,
    // jumps
    JALR, BRANCH,
    // set lower than operations
    SLTS, SLTU
}

pub struct fu_data_t {
    // fu: fu_t,
    operator: fu_op,
    operand_a: i64,
    operand_b: i64,
    imm: i64
    // trans_id //what is this

}
