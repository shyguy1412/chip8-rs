use crate::memory::Memory;

use super::{A_NIBBLE_MASK, B_NIBBLE_MASK, NextInstruction};
use NextInstruction::*;

// 0xEa9E SKP Va
pub fn skip_if_button_pressed(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    //TODO: Read input
    Increment
}

// 0xEaA1 SKNP Va
pub fn skip_if_button_not_pressed(memory: &mut Memory, op_1: u8, op_2: u8) -> NextInstruction {
    //TODO: Read input
    Jump(
        memory
            .program_counter
            .checked_add(4)
            .expect("Program Counter Overflow"),
    )
}
