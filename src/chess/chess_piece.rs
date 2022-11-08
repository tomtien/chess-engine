use std::fmt::{self, Display};

use bitflags::bitflags;

use super::InvalidNotationError;

bitflags! {
    pub struct ChessPiece:u8 {
        const BLACK =   0b10000000;
        const WHITE =   0b01000000;
        const KING =    0b00100000;
        const QUEEN =   0b00010000;
        const ROOK =    0b00001000;
        const BISHOP =  0b00000100;
        const KNIGHT =  0b00000010;
        const PAWN =    0b00000001;
        const COLOR_BITMASK = 0b11000000;
        const COLOR_OFFSET = 6;
        const PIECE_BITMASK = 0b00111111;
        const PIECE_OFFSET = 0;
    }
}
impl ChessPiece {
    pub fn new_from_notation(code: &str) -> Result<Self, InvalidNotationError> {
        let mut piece = Self::empty();
        if code.to_uppercase().eq(code) {
            piece |= Self::WHITE
        } else {
            piece |= Self::BLACK
        }

        match code.to_lowercase().as_str() {
            "k" => piece |= Self::KING,
            "q" => piece |= Self::QUEEN,
            "r" => piece |= Self::ROOK,
            "b" => piece |= Self::BISHOP,
            "n" => piece |= Self::KNIGHT,
            "p" => piece |= Self::PAWN,
            _ => return Err(InvalidNotationError),
        }

        Ok(piece)
    }
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repr: String = "".to_string();
        match *self & ChessPiece::PIECE_BITMASK {
            ChessPiece::PAWN => repr = "p".to_string(),
            ChessPiece::KNIGHT => repr = "n".to_string(),
            ChessPiece::BISHOP => repr = "b".to_string(),
            ChessPiece::ROOK => repr = "r".to_string(),
            ChessPiece::QUEEN => repr = "q".to_string(),
            ChessPiece::KING => repr = "k".to_string(),
            _ => (),
        }

        if self.contains(ChessPiece::WHITE) {
            repr = repr.to_uppercase().to_string();
        }

        write!(f, "{}", repr)
    }
}
