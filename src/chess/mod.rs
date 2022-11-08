pub mod chess_board;
pub mod chess_color;
pub mod chess_coordinate;
pub mod chess_move;
pub mod chess_piece;
use std::fmt;

pub const CHESS_BOARD_WIDTH: usize = 8;
pub const CHESS_BOARD_HEIGHT: usize = 8;
pub const CHESS_BOARD_SIZE: usize = CHESS_BOARD_WIDTH * CHESS_BOARD_HEIGHT;

pub const CHESS_COLORS: usize = 2;

#[derive(Debug)]
pub struct InvalidFENError;
impl fmt::Display for InvalidFENError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The FEN provided is invalid")
    }
}
#[derive(Debug)]
pub struct InvalidNotationError;
impl fmt::Display for InvalidNotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The Notation provided is invalid")
    }
}
