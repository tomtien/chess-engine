use std::fmt::{self, Display};

use bitflags::bitflags;

use super::InvalidNotationError;

bitflags! {
   pub struct ChessColor:u8{
    const WHITE = 1;
    const BLACK = 2;
   }
}

impl ChessColor {
    #[inline(always)]
    pub fn get_idx(&self) -> usize {
        (self.bits() - 1) as usize
    }
}
