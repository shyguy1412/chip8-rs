use crate::sprites::HEX_DIGITS;

#[derive(Debug)]
pub struct Ram(Box<[u8; 4096]>);
impl std::default::Default for Ram {
    fn default() -> Self {
        let mut ram = Box::new([0; 4096]);
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
