use std::sync::{Arc, Mutex};

pub mod sprites;

type FramePackFormat = u64;
pub const FRAME_WIDTH: usize = 64;
pub const FRAME_HEIGHT: usize = 32;

const SIZEOF_FRAME_PACK_FORMAT: usize = size_of::<FramePackFormat>() * 8;
const FRAME_RESOLUTION: usize = FRAME_WIDTH * FRAME_HEIGHT;
const FRAME_BYTES: usize = FRAME_RESOLUTION as usize / SIZEOF_FRAME_PACK_FORMAT;
// const FRAME_BYTES: usize = FRAME_WIDTH * FRAME_HEIGHT;

// asserts that frame can be packed neatly into the frame pack format
const _: () = const { assert!(FRAME_RESOLUTION == FRAME_BYTES * SIZEOF_FRAME_PACK_FORMAT) };

pub trait Chip8Display {
    fn clear(&mut self) -> ();
    fn draw(&mut self, x: u8, y: u8, row: u8) -> bool;
    fn flush(&mut self) -> ();
}

#[derive(Debug)]
pub struct FrameBuffer([FramePackFormat; FRAME_BYTES]);

impl FrameBuffer {
    fn clear(&mut self) {
        self.0.fill(0);
    }
    fn replace(&mut self, new: &FrameBuffer) {
        self.0.copy_from_slice(&new.0);
    }
}

impl std::default::Default for FrameBuffer {
    fn default() -> Self {
        Self([0; FRAME_BYTES])
    }
}

impl std::ops::Deref for FrameBuffer {
    type Target = [FramePackFormat; FRAME_BYTES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FrameBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
pub struct DoubleBuffer {
    current_frame: Arc<Mutex<FrameBuffer>>,
    draw_frame: Box<FrameBuffer>,
}

impl DoubleBuffer {
    pub fn get_frame_buffer_ref(&self) -> Arc<Mutex<FrameBuffer>> {
        self.current_frame.clone()
    }
    pub fn display_next_frame(&mut self) {
        self.current_frame
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .replace(&self.draw_frame);
    }
}
impl Chip8Display for DoubleBuffer {
    fn clear(&mut self) -> () {
        self.draw_frame.clear();
        self.display_next_frame();
    }

    fn draw(&mut self, x: u8, y: u8, row: u8) -> bool {
        let y = y as usize % FRAME_HEIGHT;
        let new_row = self.draw_frame[y] ^ (row as u64).rotate_left(x as u32);

        let intersected = new_row & self.draw_frame[y] > 0;

        self.draw_frame[y] = new_row;

        //return if intersected
        intersected
    }

    fn flush(&mut self) -> () {
        self.display_next_frame();
    }
}
