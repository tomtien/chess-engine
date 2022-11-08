use std::cmp;
use std::fmt::{self, Display, Write};

use crate::chess::chess_coordinate::notation_to_idx;
use crate::chess::chess_move::{ChessMove, MoveFlags};
use crate::chess::chess_piece::ChessPiece;
use crate::chess::InvalidFENError;
use crate::chess::{CHESS_BOARD_HEIGHT, CHESS_BOARD_SIZE, CHESS_BOARD_WIDTH};

pub const DIRECTION_OFFSETS: [i32; 8] = [
    -(CHESS_BOARD_WIDTH as i32),     //N
    1,                               //E
    CHESS_BOARD_WIDTH as i32,        //S
    -1,                              //W
    -(CHESS_BOARD_WIDTH as i32 + 1), //NW
    -(CHESS_BOARD_WIDTH as i32 - 1), //NE
    CHESS_BOARD_WIDTH as i32 - 1,    //SW
    CHESS_BOARD_WIDTH as i32 + 1,    //SE
];

pub const KNIGHT_OFFSETS: [i32; 8] = [
    -(CHESS_BOARD_WIDTH as i32 * 2) + 1,
    -(CHESS_BOARD_WIDTH as i32) + 2,
    CHESS_BOARD_WIDTH as i32 + 2,
    CHESS_BOARD_WIDTH as i32 * 2 + 1,
    CHESS_BOARD_WIDTH as i32 * 2 - 1,
    CHESS_BOARD_WIDTH as i32 - 2,
    -(CHESS_BOARD_WIDTH as i32) - 2,
    -(CHESS_BOARD_WIDTH as i32 * 2) - 1,
];

// pub const KNIGHT_OFFSETS: [i32; 8] = [
//     -(CHESS_BOARD_WIDTH as i32 * 2 - 1), //NE
//     CHESS_BOARD_WIDTH as i32 + 2,
//     -(CHESS_BOARD_WIDTH as i32 - 2), //SE
//     CHESS_BOARD_WIDTH as i32 * 2 + 1,
//     CHESS_BOARD_WIDTH as i32 * 2 - 1, //SW
//     -(CHESS_BOARD_WIDTH as i32 + 2),
//     CHESS_BOARD_WIDTH as i32 - 2, //TW
//     -(CHESS_BOARD_WIDTH as i32 * 2 + 1),
// ];

#[derive(Debug)]
pub struct ChessBoard {
    pub layout: [ChessPiece; CHESS_BOARD_SIZE],
    pub color_to_move: ChessPiece,
    pub can_catle_long: Vec<ChessPiece>,
    pub can_catle_short: Vec<ChessPiece>,
    pub en_passant_target: Option<u16>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    squares_to_edge: [[usize; 8]; CHESS_BOARD_SIZE],
    knight_jumps: Vec<Vec<usize>>,
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut instance = Self {
            layout: [ChessPiece::empty(); CHESS_BOARD_SIZE],
            color_to_move: ChessPiece::WHITE,
            can_catle_long: vec![ChessPiece::WHITE, ChessPiece::BLACK],
            can_catle_short: vec![ChessPiece::WHITE, ChessPiece::BLACK],
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            squares_to_edge: [[0; 8]; CHESS_BOARD_SIZE],
            knight_jumps: vec![vec![]; CHESS_BOARD_SIZE],
        };
        instance.precompute_board_data();
        instance
    }

    pub fn new_from_fen(fen: &str) -> Result<Self, InvalidFENError> {
        let mut layout = [ChessPiece::empty(); CHESS_BOARD_SIZE];
        let mut x = 0;
        let mut y = 0;

        let parts: Vec<&str> = fen.split_whitespace().collect();

        if parts.len() < 6 || parts.len() > 6 {
            return Err(InvalidFENError);
        }
        let placement_part = parts[0];
        let active_color_part = parts[1];
        let castling_part = parts[2];
        let en_passant_part = parts[3];
        let halfmove_part = parts[4];
        let fullmove_part = parts[5];

        let color_to_move: ChessPiece;
        let halfmove_clock: u32;
        let fullmove_number: u32;
        let mut can_catle_short = vec![];
        let mut can_catle_long = vec![];

        for piece_code in placement_part.chars() {
            let code_string = piece_code.to_string();
            let code = code_string.as_str();
            if let Ok(number) = code.parse::<usize>() {
                x += number;
                if number > CHESS_BOARD_SIZE {
                    return Err(InvalidFENError);
                }
            } else if code == "/" {
                x = 0;
                y += 1;
            } else {
                if let Ok(piece) = ChessPiece::new_from_notation(code) {
                    layout[x + y * 8] = piece;
                    x += 1;
                } else {
                    return Err(InvalidFENError);
                }
            }
        }

        match active_color_part {
            "w" => color_to_move = ChessPiece::WHITE,
            "b" => color_to_move = ChessPiece::BLACK,
            _ => return Err(InvalidFENError),
        }

        for chr in castling_part.chars() {
            let code = chr.to_string();
            if let Ok(piece) = ChessPiece::new_from_notation(&code) {
                match piece & ChessPiece::PIECE_BITMASK {
                    ChessPiece::KING => can_catle_short.push(piece & ChessPiece::COLOR_BITMASK),
                    ChessPiece::QUEEN => can_catle_long.push(piece & ChessPiece::COLOR_BITMASK),
                    _ => (),
                }
            }
        }
        let en_passant_target: Option<u16> = match notation_to_idx(en_passant_part) {
            Ok(idx) => Some(idx),
            Err(_) => None,
        };

        if let Ok(number) = halfmove_part.parse::<u32>() {
            halfmove_clock = number;
        } else {
            return Err(InvalidFENError);
        }
        if let Ok(number) = fullmove_part.parse::<u32>() {
            fullmove_number = number;
        } else {
            return Err(InvalidFENError);
        }
        let mut instance = Self {
            layout,
            color_to_move,
            can_catle_long,
            can_catle_short,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            squares_to_edge: [[0; 8]; CHESS_BOARD_SIZE],
            knight_jumps: vec![vec![]; CHESS_BOARD_SIZE],
        };
        instance.precompute_board_data();
        Ok(instance)
    }
    fn precompute_board_data(&mut self) {
        for idx in 0..CHESS_BOARD_SIZE {
            let x = idx % CHESS_BOARD_WIDTH;
            let y = idx / CHESS_BOARD_WIDTH;

            let north = y;
            let east = CHESS_BOARD_WIDTH - x - 1;
            let south = CHESS_BOARD_HEIGHT - y - 1;
            let west = x;
            let north_west = cmp::min(north, west);
            let norht_east = cmp::min(north, east);
            let south_west = cmp::min(south, west);
            let south_east = cmp::min(south, east);

            self.squares_to_edge[idx][0] = north;
            self.squares_to_edge[idx][1] = east;
            self.squares_to_edge[idx][2] = south;
            self.squares_to_edge[idx][3] = west;
            self.squares_to_edge[idx][4] = north_west;
            self.squares_to_edge[idx][5] = norht_east;
            self.squares_to_edge[idx][6] = south_west;
            self.squares_to_edge[idx][7] = south_east;

            for i in 0..8 {
                if self.squares_to_edge[idx][i / 2] >= 2 - ((i) % 2)
                    && self.squares_to_edge[idx][(i / 2 + 1) % 4] >= 2 - ((i + 1) % 2)
                {
                    let offset = KNIGHT_OFFSETS[i];
                    self.knight_jumps[idx].push((idx as i32 + offset) as usize);
                }
            }
        }
    }
    pub fn make_move_unchecked(&mut self, chess_move: &ChessMove) {
        let (start_sq, end_sq, flags) = chess_move.get_idx();

        let start_idx = start_sq as usize;
        let end_idx = end_sq as usize;

        let mut piece = self.layout[start_idx];
        let captured_piece = self.layout[end_idx];

        let piece_is_pawn = piece.contains(ChessPiece::PAWN);
        let opposite_color = self.color_to_move ^ ChessPiece::COLOR_BITMASK;

        //Check castling
        if piece.contains(ChessPiece::KING) {
            if flags == MoveFlags::CASTLE_LONG {
                let rook_end_idx = (start_idx as i32 + DIRECTION_OFFSETS[3]) as usize;
                let rook_start_idx = (start_idx as i32 + DIRECTION_OFFSETS[3] * 4) as usize;
                let rook = self.layout[rook_start_idx];
                self.layout[rook_end_idx] = rook;
                self.layout[rook_start_idx] = ChessPiece::empty();
            }
            if flags == MoveFlags::CASTLE_SHORT {
                let rook_end_idx = (start_idx as i32 + DIRECTION_OFFSETS[1]) as usize;
                let rook_start_idx = (start_idx as i32 + DIRECTION_OFFSETS[1] * 3) as usize;
                let rook = self.layout[rook_start_idx];
                self.layout[rook_end_idx] = rook;
                self.layout[rook_start_idx] = ChessPiece::empty();
            }

            self.can_catle_long
                .retain(|&color| color != self.color_to_move);

            self.can_catle_short
                .retain(|&color| color != self.color_to_move);
        }
        //Check if rook moves
        if piece.contains(ChessPiece::ROOK) {
            if start_idx % 8 < CHESS_BOARD_WIDTH / 2 {
                self.can_catle_long
                    .retain(|&color| color != self.color_to_move);
            } else if start_idx % 8 > CHESS_BOARD_WIDTH / 2 {
                self.can_catle_short
                    .retain(|&color| color != self.color_to_move);
            }
        }
        //Check if rook  is captured
        if captured_piece.contains(ChessPiece::ROOK) {
            if end_idx % 8 < CHESS_BOARD_WIDTH / 2 {
                self.can_catle_long.retain(|&color| color != opposite_color);
            } else if end_idx % 8 > CHESS_BOARD_WIDTH / 2 {
                self.can_catle_short
                    .retain(|&color| color != opposite_color);
            }
        }

        //Check for pormoton
        if piece_is_pawn
            && (end_idx < CHESS_BOARD_WIDTH || end_idx > CHESS_BOARD_SIZE - CHESS_BOARD_WIDTH - 1)
        {
            piece = ChessPiece::QUEEN | self.color_to_move;
        }

        //Check for en passant
        if let Some(en_passant_idx) = self.en_passant_target {
            if flags == MoveFlags::EN_PASSANT {
                let x = en_passant_idx % 8;
                let y = start_sq / 8;
                self.layout[(y * CHESS_BOARD_WIDTH as u16 + x) as usize] = ChessPiece::empty();
            }
        }
        //Check if pawn has moved twice
        if flags == MoveFlags::PAWN_TWO_FORWARD {
            let diff = (start_idx as i32 - end_idx as i32) / 2;
            self.en_passant_target = Some((start_idx as i32 - diff) as u16);
        } else {
            self.en_passant_target = None;
        }

        if !piece_is_pawn && captured_piece.is_empty() {
            self.halfmove_clock += 1;
        } else {
            self.halfmove_clock = 0;
        }

        self.layout[end_idx] = piece;
        self.layout[start_idx] = ChessPiece::empty();

        if self.color_to_move == ChessPiece::BLACK {
            self.fullmove_number += 1;
        }
        self.color_to_move = opposite_color;
    }
    pub fn generate_legal_moves(&self) -> Vec<ChessMove> {
        let mut moves = vec![];

        for idx in 0..CHESS_BOARD_SIZE {
            let square = self.layout[idx];

            if square.contains(self.color_to_move) {
                let start_pos = idx as u16;
                match square & ChessPiece::PIECE_BITMASK {
                    ChessPiece::KING => self.generate_king_moves(&mut moves, start_pos),
                    ChessPiece::QUEEN => self.generate_sliding_moves(&mut moves, start_pos, 0, 8),
                    ChessPiece::ROOK => self.generate_sliding_moves(&mut moves, start_pos, 0, 4),
                    ChessPiece::BISHOP => self.generate_sliding_moves(&mut moves, start_pos, 4, 8),
                    ChessPiece::KNIGHT => self.generate_knight_moves(&mut moves, start_pos),
                    ChessPiece::PAWN => self.generate_pawn_moves(
                        &mut moves,
                        start_pos,
                        (((square.bits() >> ChessPiece::COLOR_OFFSET.bits()) - 1) * 2) as usize,
                    ),
                    _ => (),
                }
            }
        }

        moves
    }
    fn generate_king_moves(&self, moves: &mut Vec<ChessMove>, start_pos: u16) {
        for i in 0..8 {
            let squares_to_edge = self.squares_to_edge[start_pos as usize][i];
            if squares_to_edge >= 1 {
                let end_pos = (start_pos as i32 + DIRECTION_OFFSETS[i]) as u16;
                if !self.layout[end_pos as usize].contains(self.color_to_move) {
                    moves.push(ChessMove::new(start_pos, end_pos, MoveFlags::empty()));
                }
            }
        }
        if self.can_catle_short.contains(&self.color_to_move) {
            let mut occupied = false;
            for i in 1..3 {
                let end_idx = (start_pos as i32 + DIRECTION_OFFSETS[1] * i) as u16;
                if self.layout[end_idx as usize] != ChessPiece::empty() {
                    occupied = true;
                    break;
                }
            }

            if !occupied {
                let end_pos = (start_pos as i32 + DIRECTION_OFFSETS[1] * 2) as u16;
                moves.push(ChessMove::new(start_pos, end_pos, MoveFlags::CASTLE_SHORT))
            }
        }
        if self.can_catle_long.contains(&self.color_to_move) {
            let mut occupied = false;
            for i in 1..4 {
                let end_idx = (start_pos as i32 + DIRECTION_OFFSETS[3] * i) as u16;
                if self.layout[end_idx as usize] != ChessPiece::empty() {
                    occupied = true;
                    break;
                }
            }
            if !occupied {
                let end_pos = (start_pos as i32 + DIRECTION_OFFSETS[3] * 2) as u16;
                moves.push(ChessMove::new(start_pos, end_pos, MoveFlags::CASTLE_LONG));
            }
        }
    }
    fn generate_knight_moves(&self, moves: &mut Vec<ChessMove>, start_pos: u16) {
        let jumps = &self.knight_jumps[start_pos as usize];
        for end_pos in jumps {
            if !self.layout[*end_pos].contains(self.color_to_move) {
                let chess_move = ChessMove::new(start_pos, *end_pos as u16, MoveFlags::empty());
                moves.push(chess_move);
            }
        }
    }
    fn generate_pawn_moves(&self, moves: &mut Vec<ChessMove>, start_pos: u16, offset: usize) {
        let squares_to_edge = self.squares_to_edge[start_pos as usize];

        if squares_to_edge[offset] < 1 {
            return;
        }

        let direction_offset = DIRECTION_OFFSETS[offset];
        let end_pos = (start_pos as i32 + direction_offset) as u16;

        if self.layout[end_pos as usize] == ChessPiece::empty() {
            moves.push(ChessMove::new(start_pos, end_pos, MoveFlags::empty()));
            if squares_to_edge[offset] == CHESS_BOARD_HEIGHT - 2 {
                let end_pos_2 = (start_pos as i32 + direction_offset * 2) as u16;

                if self.layout[end_pos_2 as usize] == ChessPiece::empty() {
                    moves.push(ChessMove::new(
                        start_pos,
                        end_pos_2,
                        MoveFlags::PAWN_TWO_FORWARD,
                    ));
                }
            }
        }

        if let Some(target) = self.en_passant_target {
            for i in 0..2 {
                let squares_to_side = squares_to_edge[4 + offset + i];
                if squares_to_side >= 1 {
                    let end_pos = (start_pos as i32 + DIRECTION_OFFSETS[4 + offset + i]) as u16;
                    if end_pos == target {
                        moves.push(ChessMove::new(start_pos, end_pos, MoveFlags::EN_PASSANT));
                    }
                }
            }
        }
        for i in 0..2 {
            let squares_to_side = squares_to_edge[4 + offset + i];
            if squares_to_side >= 1 {
                let end_pos = (start_pos as i32 + DIRECTION_OFFSETS[4 + offset + i]) as u16;
                if !self.layout[end_pos as usize].contains(self.color_to_move)
                    && !self.layout[end_pos as usize].is_empty()
                {
                    moves.push(ChessMove::new(start_pos, end_pos, MoveFlags::empty()));
                }
            }
        }
    }
    fn generate_sliding_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        start_pos: u16,
        start: usize,
        end: usize,
    ) {
        for direction_idx in start..end {
            let squares_to_edge = self.squares_to_edge[start_pos as usize][direction_idx];

            for num_squares in 1..squares_to_edge + 1 {
                let end_pos = (start_pos as i32
                    + (num_squares as i32 * DIRECTION_OFFSETS[direction_idx]))
                    as u16;
                let square = self.layout[end_pos as usize];

                if square.contains(self.color_to_move) {
                    break;
                }

                moves.push(ChessMove::new(start_pos, end_pos, MoveFlags::empty()));
                if !square.is_empty() && !square.contains(self.color_to_move) {
                    break;
                }
            }
        }
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("┌───")?;
        f.write_str(&"┬───".repeat(CHESS_BOARD_WIDTH - 2))?;
        f.write_str("┬───┐")?;
        f.write_char('\n')?;

        for y in 0..CHESS_BOARD_HEIGHT {
            f.write_str("│")?;
            for x in 0..CHESS_BOARD_WIDTH {
                let piece = self.layout[x + y * 8];
                if !piece.is_empty() {
                    f.write_fmt(format_args!(" {} │", &piece))?;
                } else {
                    f.write_str("   │")?;
                }
            }
            f.write_char('\n')?;
            if y < CHESS_BOARD_HEIGHT - 1 {
                f.write_str("├───")?;
                f.write_str(&"┼───".repeat(CHESS_BOARD_WIDTH - 2))?;
                f.write_str("┼───┤")?;
                f.write_char('\n')?;
            }
        }
        f.write_str("└───")?;
        f.write_str(&"┴───".repeat(CHESS_BOARD_WIDTH - 2))?;
        f.write_str("┴───┘")?;
        f.write_char('\n')?;
        f.write_str("  a   b   c   d   e   f   g   h")
    }
}
