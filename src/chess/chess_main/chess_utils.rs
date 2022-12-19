use std::cmp::{min, max};

pub fn gen_moves_queen(index: usize, board_arr: &Vec<i32>, is_white: bool) -> Vec<usize> {
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        //right
        for i in 1..8 - index % 8 {
            if board_arr[index + i] == 0 {
                moves.push(index + i);
            }
            else if board_arr[index + i] < 0 {
                moves.push(index + i);
                break;
            }
            else{
                break;
            }
        }
        //left
        for i in 1..index % 8 + 1 {
            if board_arr[index - i] == 0 {
                moves.push(index - i);
            }
            else if board_arr[index - i] < 0 {
                moves.push(index - i);
                break;
            }
            else{
                break;
            }
        }
        //down
        for i in 1..8 - index / 8 {
            if board_arr[index + i * 8] == 0 {
                moves.push(index + i * 8);
            }
            else if board_arr[index + i * 8] < 0 {
                moves.push(index + i * 8);
                break;
            }
            else{
                break;
            }
        }
        //up
        for i in 1..index / 8 + 1 {
            if board_arr[index - i * 8] == 0 {
                moves.push(index - i * 8);
            }
            else if board_arr[index - i * 8] < 0 {
                moves.push(index - i * 8);
                break;
            }
            else{
                break;
            }
        }
        //up-right
        for i in 1..min(index / 8 + 1, 8 - index % 8) {
            if board_arr[index - i * 7] == 0 {
                moves.push(index - i * 7);
            }
            else if board_arr[index - i * 7] < 0 {
                moves.push(index - i * 7);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(index / 8 + 1, index % 8 + 1) {
            if board_arr[index - i * 9] == 0 {
                moves.push(index - i * 9);
            }
            else if board_arr[index - i * 9] < 0 {
                moves.push(index - i * 9);
                break;
            }
            else{
                break;
            }
        }
        //down-right
        for i in 1..min(8 - index / 8, 8 - index % 8) {
            if board_arr[index + i * 9] == 0 {
                moves.push(index + i * 9);
            }
            else if board_arr[index + i * 9] < 0 {
                moves.push(index + i * 9);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(8 - index / 8, index % 8 + 1) {
            if board_arr[index + i * 7] == 0 {
                moves.push(index + i * 7);
            }
            else if board_arr[index + i * 7] < 0 {
                moves.push(index + i * 7);
                break;
            }
            else{
                break;
            }
        }
    }
    else {
        //right
        for i in 1..8 - index % 8 {
            if board_arr[index + i] == 0 {
                moves.push(index + i);
            }
            else if board_arr[index + i] > 0 {
                moves.push(index + i);
                break;
            }
            else{
                break;
            }
        }
        //left
        for i in 1..index % 8 + 1 {
            if board_arr[index - i] == 0 {
                moves.push(index - i);
            }
            else if board_arr[index - i] > 0 {
                moves.push(index - i);
                break;
            }
            else{
                break;
            }
        }
        //down
        for i in 1..8 - index / 8 {
            if board_arr[index + i * 8] == 0 {
                moves.push(index + i * 8);
            }
            else if board_arr[index + i * 8] > 0 {
                moves.push(index + i * 8);
                break;
            }
            else{
                break;
            }
        }
        //up
        for i in 1..index / 8 + 1 {
            if board_arr[index - i * 8] == 0 {
                moves.push(index - i * 8);
            }
            else if board_arr[index - i * 8] > 0 {
                moves.push(index - i * 8);
                break;
            }
            else{
                break;
            }
        }
        //up-right
        for i in 1..min(index / 8 + 1, 8 - index % 8) {
            if board_arr[index - i * 7] == 0 {
                moves.push(index - i * 7);
            }
            else if board_arr[index - i * 7] > 0 {
                moves.push(index - i * 7);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(index / 8 + 1, index % 8 + 1) {
            if board_arr[index - i * 9] == 0 {
                moves.push(index - i * 9);
            }
            else if board_arr[index - i * 9] > 0 {
                moves.push(index - i * 9);
                break;
            }
            else{
                break;
            }
        }
        //down-right
        for i in 1..min(8 - index / 8, 8 - index % 8) {
            if board_arr[index + i * 9] == 0 {
                moves.push(index + i * 9);
            }
            else if board_arr[index + i * 9] > 0 {
                moves.push(index + i * 9);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(8 - index / 8, index % 8 + 1) {
            if board_arr[index + i * 7] == 0 {
                moves.push(index + i * 7);
            }
            else if board_arr[index + i * 7] > 0 {
                moves.push(index + i * 7);
                break;
            }
            else{
                break;
            }
        }
    }
    return moves;
}

pub fn gen_moves_bishop(index: usize, board_arr: &Vec<i32>, is_white: bool) -> Vec<usize> {
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        //up-right
        for i in 1..min(index / 8 + 1, 8 - index % 8) {
            if board_arr[index - i * 7] == 0 {
                moves.push(index - i * 7);
            }
            else if board_arr[index - i * 7] < 0 {
                moves.push(index - i * 7);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(index / 8 + 1, index % 8 + 1) {
            if board_arr[index - i * 9] == 0 {
                moves.push(index - i * 9);
            }
            else if board_arr[index - i * 9] < 0 {
                moves.push(index - i * 9);
                break;
            }
            else{
                break;
            }
        }
        //down-right
        for i in 1..min(8 - index / 8, 8 - index % 8) {
            if board_arr[index + i * 9] == 0 {
                moves.push(index + i * 9);
            }
            else if board_arr[index + i * 9] < 0 {
                moves.push(index + i * 9);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(8 - index / 8, index % 8 + 1) {
            if board_arr[index + i * 7] == 0 {
                moves.push(index + i * 7);
            }
            else if board_arr[index + i * 7] < 0 {
                moves.push(index + i * 7);
                break;
            }
            else{
                break;
            }
        }
    }
    else {
        //up-right
        for i in 1..min(index / 8 + 1, 8 - index % 8) {
            if board_arr[index - i * 7] == 0 {
                moves.push(index - i * 7);
            }
            else if board_arr[index - i * 7] > 0 {
                moves.push(index - i * 7);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(index / 8 + 1, index % 8 + 1) {
            if board_arr[index - i * 9] == 0 {
                moves.push(index - i * 9);
            }
            else if board_arr[index - i * 9] > 0 {
                moves.push(index - i * 9);
                break;
            }
            else{
                break;
            }
        }
        //down-right
        for i in 1..min(8 - index / 8, 8 - index % 8) {
            if board_arr[index + i * 9] == 0 {
                moves.push(index + i * 9);
            }
            else if board_arr[index + i * 9] > 0 {
                moves.push(index + i * 9);
                break;
            }
            else{
                break;
            }
        }
        //up-left
        for i in 1..min(8 - index / 8, index % 8 + 1) {
            if board_arr[index + i * 7] == 0 {
                moves.push(index + i * 7);
            }
            else if board_arr[index + i * 7] > 0 {
                moves.push(index + i * 7);
                break;
            }
            else{
                break;
            }
        }
    }
    return moves;
}

pub fn gen_moves_rook(index: usize, board_arr: &Vec<i32>, is_white: bool) -> Vec<usize> {
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        //right
        for i in 1..8 - index % 8 {
            if board_arr[index + i] == 0 {
                moves.push(index + i);
            }
            else if board_arr[index + i] < 0 {
                moves.push(index + i);
                break;
            }
            else{
                break;
            }
        }
        //left
        for i in 1..index % 8 + 1 {
            if board_arr[index - i] == 0 {
                moves.push(index - i);
            }
            else if board_arr[index - i] < 0 {
                moves.push(index - i);
                break;
            }
            else{
                break;
            }
        }
        //down
        for i in 1..8 - index / 8 {
            if board_arr[index + i * 8] == 0 {
                moves.push(index + i * 8);
            }
            else if board_arr[index + i * 8] < 0 {
                moves.push(index + i * 8);
                break;
            }
            else{
                break;
            }
        }
        //up
        for i in 1..index / 8 + 1 {
            if board_arr[index - i * 8] == 0 {
                moves.push(index - i * 8);
            }
            else if board_arr[index - i * 8] < 0 {
                moves.push(index - i * 8);
                break;
            }
            else{
                break;
            }
        }
    }
    else {
        //right
        for i in 1..8 - index % 8 {
            if board_arr[index + i] == 0 {
                moves.push(index + i);
            }
            else if board_arr[index + i] > 0 {
                moves.push(index + i);
                break;
            }
            else{
                break;
            }
        }
        //left
        for i in 1..index % 8 + 1 {
            if board_arr[index - i] == 0 {
                moves.push(index - i);
            }
            else if board_arr[index - i] > 0 {
                moves.push(index - i);
                break;
            }
            else{
                break;
            }
        }
        //down
        for i in 1..8 - index / 8 {
            if board_arr[index + i * 8] == 0 {
                moves.push(index + i * 8);
            }
            else if board_arr[index + i * 8] > 0 {
                moves.push(index + i * 8);
                break;
            }
            else{
                break;
            }
        }
        //up
        for i in 1..index / 8 + 1 {
            if board_arr[index - i * 8] == 0 {
                moves.push(index - i * 8);
            }
            else if board_arr[index - i * 8] > 0 {
                moves.push(index - i * 8);
                break;
            }
            else{
                break;
            }
        }
    }
    return moves;
}

pub fn gen_moves_knight(index: usize, board_arr: &Vec<i32>, is_white: bool) -> Vec<usize> {
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        todo!();
    }
    else {
        todo!();
    }
    return moves;
}

pub fn gen_moves_pawn(index: usize, board_arr: &Vec<i32>, is_white: bool) -> Vec<usize> {
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        todo!();
    }
    else {
        todo!();
    }
    return moves;
}

pub fn gen_moves_king(index: usize, board_arr: &Vec<i32>, is_white: bool) -> Vec<usize> {
    todo!();
}

pub fn is_check(board_arr: &Vec<i32>, is_white: bool) -> bool {
    todo!();
}

pub fn is_legal(move1: usize, board_arr: &Vec<i32>, is_white: bool) -> bool {
    todo!();
}

pub fn move_piece(from: usize, to: usize, board_arr: Vec<i32>) -> Vec<i32> {
    todo!();
}

pub fn is_checkmate(board_arr: &Vec<i32>, is_white: bool) -> bool {
    let mut index = 64;
    if is_white {
        for i in 0..64 {
            if board_arr[i] == 1 {
                index = i;
                break;
            }
        }
    }
    else {
        for i in 0..64 {
            if board_arr[i] == -1 {
                index = i;
                break;
            }
        }
    }
    gen_moves_king(index, board_arr, is_white).is_empty()
}

pub fn gen_moves(index: usize, board_arr: &Vec<i32>) -> Vec<usize> {
    todo!("gen move for any piece");
}