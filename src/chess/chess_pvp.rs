use macroquad::prelude::*;
use crate::chess::chess_utils::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut tup = (true, true, true, true);
    let mut counter = 0;
    let mut save: Vec<Vec<i32>> = Vec::new();
    let mut rep = 0;
    let mut is_white_turn: bool = true;
    let mut is_pressed: bool = false;
    let mut found: bool = false;
    let mut moves: Vec<usize> = Vec::new();
    let mut chosen: usize = 64;
    let (mut board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn) = restart();
    let mut last: Vec<i32> = board_arr.clone();
    while !(is_checkmate(board_arr.clone(), is_white_turn, &last, tup) || is_stalemate(board_arr.clone(), is_white_turn, &last, tup) || rep == 3 || counter == 50 || is_insufficient(&board_arr)) {
        draw_board(&board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn);
        if is_pressed {
            draw_move(moves.clone(), &board_arr);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            (board_arr, last, is_white_turn, is_pressed, found, moves, chosen, rep, save, tup, counter) = player_turn2(board_arr, last, is_white_turn, is_pressed, found, moves, chosen, rep, save, tup, counter);
        }
        next_frame().await;
    }
    let mut text = "";
    if counter == 50 {
        text = "draw by the 50 move rule!";
    }
    if is_insufficient(&board_arr) {
        text = "draw by insufficient material!";
    }
    if rep == 3 {
        text = "draw by repetition!";
    }
    if is_stalemate(board_arr.clone(), is_white_turn, &last, tup) {
        text = "draw by stalemate!";
    }
    if is_checkmate(board_arr.clone(), is_white_turn, &last, tup) {
        if !is_white_turn {
            text = "white won by checkmate!";
        }
        else {
            text = "black won by checkmate!";
        }
    }
    loop {
        draw_board(&board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn);
        draw_text(text, 100.0, 400.0, 60.0, RED);
        next_frame().await;
    }
}

pub fn chess_pvp() {
    main();
}