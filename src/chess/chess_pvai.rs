use macroquad::prelude::*;
use crate::chess::chess_utils::*;

#[macroquad::main(window_conf)]
async fn main() {
    let (mut board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn) = restart();
    let mut last: Vec<i32> = board_arr.clone();
    while !is_checkmate(board_arr.clone(), is_white_turn, &last) {
        draw_board(&board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn);
        next_frame().await;
    }
    todo!("PVAI");
}

pub fn pvai() {
    main();
    todo!("minimax");
    todo!("alpha-beta pruning");
    todo!("possibly optimize the check function");
    todo!("player vs computer function");
    todo!("computer vs computer function");
}