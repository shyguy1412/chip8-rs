use crate::memory::Memory;

use super::{A_NIBBLE_MASK, B_NIBBLE_MASK, NextInstruction};
use NextInstruction::*;

// 0x8ab1 OR Va, Vb
pub fn or_register(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    memory.registers[a as usize].load(a_register | b_register);

    Increment
}

// 0x8ab2 AND Va, Vb
pub fn and_register(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    memory.registers[a as usize].load(a_register & b_register);

    Increment
}

// 0x8ab3 AND Va, Vb
pub fn xor_register(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    memory.registers[a as usize].load(a_register ^ b_register);

    Increment
}

// 0x8ab6 SHR Vx {, Vy}
pub fn shift_right(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let a_register = *memory.registers[a as usize];

    memory.registers[0xF].load(a_register & 1);

    memory.registers[a as usize].load(a_register >> 1);
    Increment
}

// 0x8abE SHL Vx {, Vy}
pub fn shift_left(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let a_register = *memory.registers[a as usize];

    memory.registers[0xF].load(a_register & 128);

    memory.registers[a as usize].load(a_register << 1);
    Increment
}
