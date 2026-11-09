use crate::memory::Memory;

use super::{A_NIBBLE_MASK, B_NIBBLE_MASK, NextInstruction};
use NextInstruction::*;

// 0x6akk LD Va, byte
pub fn load_literal_to_register(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let register = op_1 & A_NIBBLE_MASK;
    let value = op_2;

    memory
        .registers
        .get_mut(register as usize)
        .expect("Invalid Register")
        .load(value);

    Increment
}

// 0x8ab0 LD Va, Vb
pub fn load_register_to_register(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let b_register = *memory.registers[b as usize];

    memory.registers[a as usize].load(b_register);

    Increment
}

// 0xAnnn LD I, addr
pub fn load_literal_to_index(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let addr = u16::from_be_bytes([op_1 & A_NIBBLE_MASK, op_2]);
    memory.index_register.load(addr);
    Increment
}

// 0xFa55 LD [I], Va
pub fn store_multiple_at_index(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let index = *memory.index_register;

    for i in 0..a {
        let byte = *memory.registers[i as usize];
        memory.ram[index + i as u16] = byte;
    }

    Increment
}

// 0xFa65 LD Va, [I]
pub fn load_multiple_from_index(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let index = *memory.index_register;

    for i in 0..a {
        let byte = memory.ram[index + i as u16];
        memory.registers[i as usize].load(byte);
    }

    Increment
}

//0xFa33 LD B, Va
pub fn store_binary_coded_decimal(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let hundreds = a / 100;
    let tens = (a - (hundreds * 100)) / 10;
    let ones = (a - (hundreds * 100) - (tens * 10));
    memory.ram[*memory.index_register] = hundreds;
    memory.ram[*memory.index_register + 1] = tens;
    memory.ram[*memory.index_register + 2] = ones;
    Increment
}
