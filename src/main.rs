#![cfg_attr(debug_assertions, allow(dead_code, unused))]

mod display;
use std::{fmt::format, io::Write, marker::PhantomData, ops::Index, time::Duration};

use display::*;

mod memory;
use memory::*;

mod operations;

trait Chip8State {}

struct Uninitialized;
impl Chip8State for Uninitialized {}

struct Initialized;
impl Chip8State for Initialized {}

#[derive(Debug)]
struct Chip8<D: Chip8Display, State: Chip8State = Initialized> {
    framebuffer: D,
    registers: [Register<u8>; 16],
    index_register: Register<u16>,
    stack: [u16; 16],
    stack_pointer: u8,
    program_counter: u16,
    ram: Ram,
    delay_timer: u8,
    sound_timer: u8,
    state: PhantomData<State>,
}

impl<D: Chip8Display + Default> Default for Chip8<D, Uninitialized> {
    // at the time of writing this, setting custom default values for a struct is not possible with derive macros
    fn default() -> Self {
        Self {
            framebuffer: Default::default(),
            registers: Default::default(),
            index_register: Default::default(),
            stack: Default::default(),
            stack_pointer: Default::default(),
            program_counter: 0x200,
            ram: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            state: Default::default(),
        }
    }
}

impl<D: Chip8Display> Chip8<D, Uninitialized> {
    pub fn load_binary(mut self, bin: &[u8]) -> Chip8<D, Initialized> {
        //Todo: check bounds to see if bin fits in ram
        self.ram[0x200..0x200 + bin.len()].clone_from_slice(bin);
        Chip8 {
            framebuffer: self.framebuffer,
            registers: self.registers,
            index_register: self.index_register,
            stack: self.stack,
            stack_pointer: self.stack_pointer,
            program_counter: self.program_counter,
            ram: self.ram,
            delay_timer: self.delay_timer,
            sound_timer: self.sound_timer,
            state: PhantomData,
        }
    }
}

impl<D: Chip8Display> Chip8<D, Initialized> {
    pub fn cycle(&mut self) {
        let [op_1, op_2] =
            &self.ram[self.program_counter as usize..self.program_counter as usize + 2]
        else {
            unreachable!("We always grab exactly 2 bytes")
        };

        let code = op_1 & 0xF0;
        let variant = op_2 & 0x0F;

        match [code, variant, *op_1, *op_2] {
            [.., 0x00, 0xE0] => todo!("0 -> CLS"),
            [.., 0x00, 0xEE] => todo!("0 -> RET"),
            [0x00, ..] => {
                panic!("SYS opcode: This should not be used or implemented by modern interpreters")
            }

            [0x10, ..] => todo!("1 -> JP"),
            [0x20, ..] => todo!("2 -> CALL"),
            [0x30, ..] => todo!("3 -> SE"),
            [0x40, ..] => todo!("4 -> SNE"),
            [0x50, 0x00, ..] => todo!("5 -> SE"),
            [0x60, ..] => todo!("6 -> LD"),
            [0x70, ..] => todo!("7 -> ADD"),

            [0x80, 0x00, ..] => todo!("8/x0 -> LD"),
            [0x80, 0x01, ..] => todo!("8/x1 -> OR"),
            [0x80, 0x02, ..] => todo!("8/x2 -> AND"),
            [0x80, 0x03, ..] => todo!("8/x3 -> XOR"),
            [0x80, 0x04, ..] => todo!("8/x4 -> ADD"),
            [0x80, 0x05, ..] => todo!("8/x5 -> SUB"),
            [0x80, 0x06, ..] => todo!("8/x6 -> SHR"),
            [0x80, 0x07, ..] => todo!("8/x7 -> SUBN"),
            [0x80, 0x0E, ..] => todo!("8/xE -> SHL"),

            [0xA0, 0x00, ..] => todo!("A -> LD"),
            [0xB0, 0x00, ..] => todo!("B -> JP"),
            [0xC0, 0x00, ..] => todo!("C -> RND"),
            [0xD0, 0x00, ..] => todo!("D -> DRW"),

            [0xE0, _, _, 0x9E] => todo!("E/9E -> SKP"),
            [0xE0, _, _, 0xA1] => todo!("E/A1 -> SKNP"),

            [0xF0, _, _, 0x07] => todo!("F/07 -> LD"),
            [0xF0, _, _, 0xA7] => todo!("F/A7 -> LD"),
            [0xF0, _, _, 0x15] => todo!("F/15 -> LD"),
            [0xF0, _, _, 0x18] => todo!("F/18 -> LD"),
            [0xF0, _, _, 0x1E] => todo!("F/1E -> ADD"),
            [0xF0, _, _, 0x29] => todo!("F/29 -> LD"),
            [0xF0, _, _, 0x33] => todo!("F/33 -> LD"),
            [0xF0, _, _, 0x55] => todo!("F/55 -> LD"),
            [0xF0, _, _, 0x65] => todo!("F/65 -> LD"),

            _ => panic!("Invalid op code {:#04X}{:02X?}", op_1, op_2),
        }
    }
}

fn main() {
    let bin = include_bytes!("../test/bin/1-chip8-logo.ch8");

    let chip8: Chip8<display::DoubleBuffer, _> = Default::default();
    let mut chip8 = chip8.load_binary(bin);

    let framebuffer = chip8.framebuffer.get_frame_buffer_ref();

    std::thread::spawn(move || {
        loop {
            const TARGET_FPS: u16 = 60;
            const FRAME_TIMING: u16 = 1000 / TARGET_FPS;
            const HIGH_SYMBOL: char = '■';
            const LOW_SYMBOL: char = '□';

            let frame_start = std::time::Instant::now();

            let framebuffer = framebuffer.lock().unwrap_or_else(|e| e.into_inner());

            let output: String = framebuffer
                .iter()
                .flat_map(|a| (0..8).map(|i| (*a << i) & 0x40))
                .map(|bit| match bit {
                    0x00 => LOW_SYMBOL,
                    _ => HIGH_SYMBOL,
                })
                .collect::<Vec<char>>()
                .chunks_exact(FRAME_WIDTH)
                .map(|chars| chars.iter().collect::<String>())
                .fold(String::new(), |prev, cur| format!("{prev}{cur}\n"));
            print!("{output}\x1B[{}A", FRAME_HEIGHT);
            std::io::stdout().flush();

            let dif = std::time::Instant::now()
                .duration_since(frame_start)
                .as_millis();

            if dif >= FRAME_TIMING as u128 {
                continue;
            }
            let sleep = FRAME_TIMING as u128 - dif;
            std::thread::sleep(Duration::from_millis(sleep as u64));
        }
    });

    loop {
        chip8.cycle();
    }
}
