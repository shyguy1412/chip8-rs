use std::num::NonZero;

use crate::sprites::HEX_DIGITS;

#[derive(Debug)]
pub struct Ram([u8; 4096]);
impl std::default::Default for Ram {
    fn default() -> Self {
        let mut ram = [0; 4096];
        ram[..HEX_DIGITS.len()].clone_from_slice(&HEX_DIGITS);
        Self(ram)
    }
}

impl std::ops::Index<u16> for Ram {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl std::ops::IndexMut<u16> for Ram {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Ram {
    type Output = [u8];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<std::ops::Range<usize>> for Ram {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Default)]
pub struct Register<T: Default>(T);

impl<T: Default> std::ops::Deref for Register<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default> Register<T> {
    pub fn load(&mut self, new: T) {
        self.0 = new;
    }
}

#[derive(Debug, Default)]
pub struct Stack {
    stack: [u16; 16],
    stack_pointer: u8,
}

impl Stack {
    pub fn push(&mut self, addr: NonZero<u16>) {
        //increment by 2 because we want to return to the next instruction
        self.stack[self.stack_pointer as usize] = addr.get() + 2;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> NonZero<u16> {
        self.stack_pointer -= 1;
        //We can unwrap safely because only NonZero can be pushed
        NonZero::new(self.stack[self.stack_pointer as usize]).unwrap()
    }
}

#[derive(Debug)]
pub struct Memory {
    pub ram: Ram,
    pub registers: [Register<u8>; 16],
    pub index_register: Register<u16>,
    pub stack: Stack,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: NonZero<u16>,
}

impl std::default::Default for Memory {
    fn default() -> Self {
        Self {
            ram: Default::default(),
            registers: Default::default(),
            index_register: Default::default(),
            stack: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            program_counter: NonZero::new(0x200).unwrap(),
        }
    }
}

impl Memory {
    pub fn load_binary() {}
    pub fn get_next_operation(&self) -> [u8; 2] {
        let op_1 = self.ram[self.program_counter.get()];
        let op_2 = self.ram[self.program_counter.get() + 1];
        [op_1, op_2]
    }
}
