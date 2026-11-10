use crate::memory::Memory;

use super::NextInstruction;
use NextInstruction::*;

// 0xEa9E SKP Va
pub fn skip_if_button_pressed(_: &mut Memory, _: u8, _: u8) -> NextInstruction {
    //TODO: Read input
    Increment
}

// 0xEaA1 SKNP Va
pub fn skip_if_button_not_pressed(memory: &mut Memory, _: u8, _: u8) -> NextInstruction {
    //TODO: Read input
    Jump(
        memory
            .program_counter
            .checked_add(4)
            .expect("Program Counter Overflow"),
    )
}
