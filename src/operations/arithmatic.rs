use crate::memory::Memory;

use super::{A_NIBBLE_MASK, B_NIBBLE_MASK, NextInstruction};
use NextInstruction::*;

// 0x7akk ADD Va, byte
pub fn add_literal(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let register = &mut memory.registers[a as usize];

    let (result, overflow) = register.overflowing_add(op_2);
    register.load(result);

    memory.registers[0xF].load(overflow as u8);

    Increment
}

// 0x8ab4 ADD Va, Vb
pub fn add_registers(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    let (result, overflow) = a_register.overflowing_add(b_register);
    memory.registers[a as usize].load(result);

    memory.registers[0xF].load(overflow as u8);

    Increment
}

// 0xFa1E ADD I, Va
pub fn add_index_pointer(memory: &mut Memory, op_1: u8, _: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let a_register = *memory.registers[a as usize];

    let (result, overflow) = memory.index_register.overflowing_add(a_register as u16);

    memory.index_register.load(result);

    memory.registers[0xF].load(overflow as u8);

    Increment
}

// 0x8ab5 SUB Va, Vb
pub fn sub_registers(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    let (result, overflow) = a_register.overflowing_sub(b_register);

    memory.registers[a as usize].load(result);
    memory.registers[0xF].load(!overflow as u8);

    Increment
}

// 0x8ab7 SUBN Va, Vb
pub fn subn_registers(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let a_register = *memory.registers[a as usize];
    let b_register = *memory.registers[b as usize];

    let (result, overflow) = b_register.overflowing_sub(a_register);

    memory.registers[a as usize].load(result);

    memory.registers[0xF].load(!overflow as u8);

    Increment
}
