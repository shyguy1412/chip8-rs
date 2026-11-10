// #![cfg_attr(debug_assertions, allow(dead_code, unused))]

mod display;
use std::{
    io::Write,
    marker::PhantomData,
    sync::{Arc, Mutex},
    time::Duration,
};

use display::*;

mod memory;
use memory::*;

use crate::{operations::NextInstruction, v_blank::VBlank};

mod operations;
mod v_blank;

trait Chip8State {}

struct Uninitialized;
impl Chip8State for Uninitialized {}

struct Initialized;
impl Chip8State for Initialized {}

static V_BLANK_INTERRUPT: VBlank = VBlank::new();

#[derive(Debug)]
struct Chip8<D: Chip8Display = DoubleBuffer, State: Chip8State = Uninitialized> {
    framebuffer: D,
    memory: Memory,
    state: PhantomData<State>,
}

impl Default for Chip8 {
    // at the time of writing this, setting custom default values for a struct is not possible with derive macros
    fn default() -> Self {
        Self {
            framebuffer: Default::default(),
            memory: Default::default(),
            state: Default::default(),
        }
    }
}

impl<D: Chip8Display> Chip8<D, Uninitialized> {
    pub fn load_binary(mut self, bin: &[u8]) -> Chip8<D, Initialized> {
        //Todo: check bounds to see if bin fits in ram
        self.memory.ram[0x200..0x200 + bin.len()].clone_from_slice(bin);
        Chip8 {
            framebuffer: self.framebuffer,
            memory: self.memory,
            state: PhantomData,
        }
    }
}

impl<D: Chip8Display> Chip8<D, Initialized> {
    pub fn cycle(&mut self) {
        let [op_1, op_2] = self.memory.get_next_operation();

        let next_instruction = self.perform_operation(op_1, op_2);

        self.memory.program_counter = match next_instruction {
            NextInstruction::Increment => self
                .memory
                .program_counter
                .checked_add(2)
                .expect("program_counter overflow"),
            NextInstruction::Jump(new) => new,
        };
    }

    pub fn perform_operation(&mut self, op_1: u8, op_2: u8) -> NextInstruction {
        let code = op_1 & 0xF0;
        let variant = op_2 & 0x0F;

        let framebuffer = &mut self.framebuffer;
        let memory = &mut self.memory;

        // let hex = format!("{:#04X}{:02X?}", op_1, op_2);

        // if hex == "0xD011" {
        //     drop(());
        // }

        match [code, variant, op_1, op_2] {
            [.., 0x00, 0xE0] => operations::clear_screen(framebuffer),
            [.., 0x00, 0xEE] => operations::return_subroutine(memory, op_1, op_2),
            //noop
            [0x00, ..] => NextInstruction::Increment,

            [0x10, ..] => operations::jump(op_1, op_2),
            [0x20, ..] => operations::call_subroutine(memory, op_1, op_2),
            [0x30, ..] => operations::skip_if_equal_literal(memory, op_1, op_2),
            [0x40, ..] => operations::skip_if_not_equal_literal(memory, op_1, op_2),
            [0x50, 0x00, ..] => operations::skip_if_equal_register(memory, op_1, op_2),
            [0x60, ..] => operations::load_literal_to_register(memory, op_1, op_2),
            [0x70, ..] => operations::add_literal(memory, op_1, op_2),

            [0x80, 0x00, ..] => operations::load_register_to_register(memory, op_1, op_2),
            [0x80, 0x01, ..] => operations::or_register(memory, op_1, op_2),
            [0x80, 0x02, ..] => operations::and_register(memory, op_1, op_2),
            [0x80, 0x03, ..] => operations::xor_register(memory, op_1, op_2),
            [0x80, 0x04, ..] => operations::add_registers(memory, op_1, op_2),
            [0x80, 0x05, ..] => operations::sub_registers(memory, op_1, op_2),
            [0x80, 0x06, ..] => operations::shift_right(memory, op_1, op_2),
            [0x80, 0x07, ..] => operations::subn_registers(memory, op_1, op_2),
            [0x80, 0x0E, ..] => operations::shift_left(memory, op_1, op_2),

            [0x90, 0x00, ..] => operations::skip_if_not_equal_register(memory, op_1, op_2),

            [0xA0, ..] => operations::load_literal_to_index(memory, op_1, op_2),
            [0xB0, ..] => operations::jump_with_offset(memory, op_1, op_2),
            [0xC0, ..] => todo!("C -> RND"),
            [0xD0, ..] => operations::draw(memory, framebuffer, op_1, op_2),

            [0xE0, _, _, 0x9E] => operations::skip_if_button_pressed(memory, op_1, op_2),
            [0xE0, _, _, 0xA1] => operations::skip_if_button_not_pressed(memory, op_1, op_2),

            [0xF0, _, _, 0x07] => operations::load_delay_timer(memory, op_1, op_2),
            [0xF0, _, _, 0x0A] => {
                std::thread::park();
                NextInstruction::Increment
            }
            [0xF0, _, _, 0x15] => operations::set_delay_timer(memory, op_1, op_2),
            [0xF0, _, _, 0x18] => todo!("F/18 -> LD"),
            [0xF0, _, _, 0x1E] => operations::add_index_pointer(memory, op_1, op_2),
            [0xF0, _, _, 0x29] => todo!("F/29 -> LD"),
            [0xF0, _, _, 0x33] => operations::store_binary_coded_decimal(memory, op_1, op_2),
            [0xF0, _, _, 0x55] => operations::store_multiple_at_index(memory, op_1, op_2),
            [0xF0, _, _, 0x65] => operations::load_multiple_from_index(memory, op_1, op_2),

            _ => panic!("Invalid op code {:#04X}{:02X?}", op_1, op_2),
        }
    }
}

fn main() {
    // let bin = include_bytes!("../test/bin/1-chip8-logo.ch8");
    // let bin = include_bytes!("../test/bin/2-ibm-logo.ch8");
    // let bin = include_bytes!("../test/bin/3-corax+.ch8");
    // let bin = include_bytes!("../test/bin/4-flags.ch8");
    let bin = include_bytes!("../test/bin/5-quirks.ch8");
    // // automatically enable chip8 test for test 5-quirks
    // chip8.memory.ram[512] = 1;
    // let bin = include_bytes!("../test/bin/6-keypad.ch8");

    let mut chip8 = Chip8::default().load_binary(bin);
    chip8.memory.ram[0x1FF] = 1;

    let framebuffer = chip8.framebuffer.get_frame_buffer_ref();
    let delay_timer = chip8.memory.delay_timer.clone();
    let sound_timer = chip8.memory.sound_timer.clone();

    timer_thread(framebuffer, delay_timer, sound_timer);

    loop {
        chip8.cycle();
    }
}

fn timer_thread(
    framebuffer: Arc<Mutex<FrameBuffer>>,
    delay_timer: Arc<Mutex<u8>>,
    sound_timer: Arc<Mutex<u8>>,
) -> () {
    std::thread::spawn(move || {
        loop {
            const TARGET_FPS: u16 = 60;
            const FRAME_TIMING: u16 = 1000 / TARGET_FPS;

            let frame_start = std::time::Instant::now();

            process_timer(&*delay_timer);
            process_timer(&*sound_timer);

            draw_frame(&*framebuffer);

            let dif = std::time::Instant::now()
                .duration_since(frame_start)
                .as_millis();

            if dif >= FRAME_TIMING as u128 {
                continue;
            }
            let sleep = FRAME_TIMING as u128 - dif;
            std::thread::sleep(Duration::from_millis(sleep as u64));
            V_BLANK_INTERRUPT.send();
        }
    });
}

fn process_timer(timer: &Mutex<u8>) {
    let mut timer = timer.lock().unwrap_or_else(|e| e.into_inner());
    *timer = timer.saturating_sub(1);
}

fn draw_frame(framebuffer: &Mutex<FrameBuffer>) {
    const HIGH_SYMBOL: char = '■';
    const LOW_SYMBOL: char = '□';

    let framebuffer = framebuffer.lock().unwrap_or_else(|e| e.into_inner());

    let output: String = framebuffer
        .iter()
        .map(|row| row.reverse_bits())
        .flat_map(|a| (0..64).map(move |i| a & 0x1 << i))
        .map(|bit| match bit {
            0x00 => LOW_SYMBOL,
            _ => HIGH_SYMBOL,
        })
        .collect::<Vec<char>>()
        .chunks_exact(FRAME_WIDTH)
        .map(|chars| chars.iter().collect::<String>())
        .fold(String::new(), |prev, cur| format!("{prev}{cur}\n"));
    println!();
    print!("{output}\x1B[{}A", FRAME_HEIGHT + 1);
    let _ = std::io::stdout().flush();
}
