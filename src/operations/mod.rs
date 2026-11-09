mod arithmatic;
pub use arithmatic::*;
mod logical;
pub use logical::*;
mod skip;
pub use skip::*;
mod load;
pub use load::*;

use std::num::NonZero;

use crate::{
    Chip8,
    display::{Chip8Display, FrameBuffer},
    memory::Memory,
};

pub enum NextInstruction {
    Increment,
    Jump(NonZero<u16>),
}
use NextInstruction::*;

//FROM: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
// a - A 4-bit value, the lower 4 bits of the high byte of the instruction
// b - A 4-bit value, the upper 4 bits of the low byte of the instruction
// kk or byte - An 8-bit value, the lowest 8 bits of the instruction

const A_NIBBLE_MASK: u8 = 0b00001111;
const B_NIBBLE_MASK: u8 = 0b11110000;

// 0x00E0 CLS
pub fn clear_screen<D: Chip8Display>(framebuffer: &mut D) -> NextInstruction {
    framebuffer.clear();
    Increment
}

// 0x1nnn JP addr
pub fn jump(op_1: u8, op_2: u8) -> NextInstruction {
    let addr = u16::from_be_bytes([op_1 & A_NIBBLE_MASK, op_2]);
    Jump(NonZero::new(addr).expect("Can not jump to null pointer"))
}

// 0x2nnn CALL addr
pub fn call_subroutine(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    let addr = u16::from_be_bytes([op_1 & A_NIBBLE_MASK, op_2]);
    memory.stack.push(memory.program_counter);
    Jump(NonZero::new(addr).expect("Can not jump to null pointer"))
}

// 0x00EE RET
pub fn return_subroutine(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    Jump(memory.stack.pop())
}

// 0xDabn DRW Va, Vb, nibble
pub fn draw<D: Chip8Display>(
    memory: &mut Memory,
    framebuffer: &mut D,
    op_1: u8,
    op_2: u8,
) -> NextInstruction {
    let a = op_1 & A_NIBBLE_MASK;
    let b = (op_2 & B_NIBBLE_MASK) >> 4;
    let n = op_2 & A_NIBBLE_MASK;
    let i = *memory.index_register;

    let x_pos = memory.registers.get(a as usize).expect("Invalid Register");
    let y_pos = memory.registers.get(b as usize).expect("Invalid Register");

    let sprite = &memory.ram[i as usize..(i + n as u16) as usize];

    let intersected = (0..n)
        .map(|row| framebuffer.draw(**x_pos, **y_pos + row, sprite[row as usize].reverse_bits()))
        .fold(false, |prev, cur| prev || cur) as u8;

    memory.registers[0xF].load(intersected);

    framebuffer.flush();

    Increment
}
