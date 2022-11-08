use std::io::Write;
use std::io::{stdin, stdout};
use std::time::Instant;

mod chess;
use chess::chess_board::ChessBoard;
use chess::chess_coordinate::{idx_to_notation, notation_to_idx};

use chess::chess_piece::ChessPiece;

fn main() {
    println!("\nJMCHESS 0.1 BETA\n");

    let mut board =
        ChessBoard::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();

    let timer = Instant::now();
    let mut legal_moves = board.generate_legal_moves();
    println!(
        "Number of legal moves: {}\nGenerating legal Moves took {}μs\n",
        legal_moves.len(),
        timer.elapsed().as_micros()
    );

    display_board(&board);

    loop {
        let mut line = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 1 {
            if let Ok(start_pos) = notation_to_idx(parts[0]) {
                for legal_move in &legal_moves {
                    let (move_start_pos, move_end_pos, _flags) = legal_move.get_idx();
                    if move_start_pos == start_pos {
                        println!("{}", idx_to_notation(move_end_pos));
                    }
                }
            }
        } else if parts.len() == 2 {
            if let Ok(start_pos) = notation_to_idx(parts[0]) {
                if let Ok(end_pos) = notation_to_idx(parts[1]) {
                    for legal_move in &legal_moves {
                        let (move_start_pos, move_end_pos, _flags) = legal_move.get_idx();
                        if start_pos as u16 == move_start_pos && end_pos as u16 == move_end_pos {
                            board.make_move_unchecked(&legal_move);
                            break;
                        }
                    }

                    let timer = Instant::now();
                    legal_moves = board.generate_legal_moves();
                    println!(
                        "Number of legal moves: {}\nGenerating legal Moves took {}μs\n",
                        legal_moves.len(),
                        timer.elapsed().as_micros()
                    );
                    display_board(&board);
                }
            }
        }
    }
}

fn display_board(board: &ChessBoard) {
    println!("---POSITION---");
    println!("{}", board);
    println!("---INFORMATION---");
    println!(
        "Color to move: {}",
        if board.color_to_move == ChessPiece::WHITE {
            "white"
        } else {
            "black"
        }
    );

    println!("Can castle short: {:?}", board.can_catle_short);
    println!("Can castle long: {:?}", board.can_catle_long);
    println!("En passant target square: {:?}", board.en_passant_target);
    println!("Halfmove clock: {}", board.halfmove_clock);
    println!("Fullmove number: {}", board.fullmove_number);
}
