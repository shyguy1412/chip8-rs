use crate::memory::Memory;

use super::{A_NIBBLE_MASK, B_NIBBLE_MASK, NextInstruction};
use NextInstruction::*;

// 0x3akk SE Va, byte
pub fn skip_if_equal_literal(memory: &Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let register = *memory.registers[a as usize];
    match register == op_2 {
        true => Jump(
            memory
                .program_counter
                .checked_add(4)
                .expect("Program Counter Overflow"),
        ),
        false => Increment,
    }
}

// 0x4akk SNE Va, byte
pub fn skip_if_not_equal_literal(memory: &Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let register = *memory.registers[a as usize];
    match register != op_2 {
        true => Jump(
            memory
                .program_counter
                .checked_add(4)
                .expect("Program Counter Overflow"),
        ),
        false => Increment,
    }
}

// 0x5ab0 SE Va, Vb
pub fn skip_if_equal_register(memory: &Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    match a_register == b_register {
        true => Jump(
            memory
                .program_counter
                .checked_add(4)
                .expect("Program Counter Overflow"),
        ),
        false => Increment,
    }
}

// 0x9ab0 SNE Va, Vb
pub fn skip_if_not_equal_register(memory: &Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    match a_register != b_register {
        true => Jump(
            memory
                .program_counter
                .checked_add(4)
                .expect("Program Counter Overflow"),
        ),
        false => Increment,
    }
}
