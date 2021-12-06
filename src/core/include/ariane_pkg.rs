// mod riscv_pkgs;
#![allow(dead_code)]
#![allow(non_camel_case_types)]

const XLEN: u8 = 64;

#[derive(Eq, PartialEq)]


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
    pub operator: fu_op,
    operand_a: u64,
    operand_b: u64,
    imm: u64
    // trans_id //what is this

}

impl fu_data_t {
    pub fn new(Operator: fu_op, Oper_a: u64, Oper_b: u64, Imm: u64) -> fu_data_t{
        fu_data_t{
            operator: Operator,
            operand_a: Oper_a,
            operand_b: Oper_b,
            imm: Imm
        }
    }

    pub fn get_operand_a(&self) -> u64 {
        self.operand_a
    }

    pub fn get_operand_b(&self) -> u64 {
        self.operand_b
    }

    pub fn get_imm(&self) -> u64 {
        self.imm
    }
}
