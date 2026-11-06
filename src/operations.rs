use crate::{Chip8, display::Chip8Display};

pub fn clear_screen<D: Chip8Display>(chip: &mut Chip8<D>, op1: u8, op2: u8) {
    chip.framebuffer.clear();
}
