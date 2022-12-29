use std::cmp::min;
use macroquad::prelude::*;
use image::*;

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
        if index / 8 >= 2 {
            if index % 8 > 0 {
                if board_arr[index - 17] <= 0 {
                    moves.push(index - 17);
                }
            }
            if index % 8 < 7 {
                if board_arr[index - 15] <= 0 {
                    moves.push(index - 15);
                }
            }
        }
        if index / 8 < 6 {
            if index % 8 > 0 {
                if board_arr[index + 15] <= 0 {
                    moves.push(index + 15);
                }
            }
            if index % 8 < 7 {
                if board_arr[index + 17] <= 0 {
                    moves.push(index + 17);
                }
            }
        }
        if index % 8 >= 2 {
            if index / 8 > 0 {
                if board_arr[index - 10] <= 0 {
                    moves.push(index - 10);
                }
            }
            if index / 8 < 7 {
                if board_arr[index + 6] <= 0 {
                    moves.push(index + 6);
                }
            }
        }
        if index % 8 < 6 {
            if index / 8 > 0 {
                if board_arr[index - 6] <= 0 {
                    moves.push(index - 6);
                }
            }
            if index / 8 < 7 {
                if board_arr[index + 10] <= 0 {
                    moves.push(index + 10);
                }
            }
        }
    }
    else {
        if index / 8 >= 2 {
            if index % 8 > 0 {
                if board_arr[index - 17] >= 0 {
                    moves.push(index - 17);
                }
            }
            if index % 8 < 7 {
                if board_arr[index - 15] >= 0 {
                    moves.push(index - 15);
                }
            }
        }
        if index / 8 < 6 {
            if index % 8 > 0 {
                if board_arr[index + 15] >= 0 {
                    moves.push(index + 15);
                }
            }
            if index % 8 < 7 {
                if board_arr[index + 17] >= 0 {
                    moves.push(index + 17);
                }
            }
        }
        if index % 8 >= 2 {
            if index / 8 > 0 {
                if board_arr[index - 10] >= 0 {
                    moves.push(index - 10);
                }
            }
            if index / 8 < 7 {
                if board_arr[index + 6] >= 0 {
                    moves.push(index + 6);
                }
            }
        }
        if index % 8 < 6 {
            if index / 8 > 0 {
                if board_arr[index - 6] >= 0 {
                    moves.push(index - 6);
                }
            }
            if index / 8 < 7 {
                if board_arr[index + 10] >= 0 {
                    moves.push(index + 10);
                }
            }
        }
    }
    return moves;
}

pub fn gen_moves_pawn(index: usize, board_arr: &Vec<i32>, is_white: bool, last: &Vec<i32>, test_check: bool) -> Vec<usize> {
    //add en passant
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        if board_arr[index - 8] == 0 && !test_check {
            moves.push(index - 8);
        }
        if index % 8 != 7 && board_arr[index - 7] < 0 {
            moves.push(index - 7);
        }
        if index % 8 != 0 && board_arr[index - 9] < 0 {
            moves.push(index - 9);
        }
        if index / 8 == 6 {
            if board_arr[index - 16] == 0 && board_arr[index - 8] == 0 && !test_check{
                moves.push(index - 16);
            }
        }
        if index >= 24 && index < 32 && !test_check{
            if index % 8 != 0 && board_arr[index - 17] == 0 && last[index - 17] == -6 && board_arr[index - 1] == -6 && last[index - 1] != -6 {
                moves.push(index - 9);
            }
            if index % 8 != 7 && board_arr[index - 15] == 0 && last[index - 15] == -6 && board_arr[index + 1] == -6 && last[index + 1] != -6 {
                moves.push(index - 7);
            }
        }
    }
    else {
        if board_arr[index + 8] == 0 && !test_check{
            moves.push(index + 8);
        }
        if index % 8 != 0 && board_arr[index + 7] > 0 {
            moves.push(index + 7);
        }
        if index % 8 != 7 && board_arr[index + 9] > 0 {
            moves.push(index + 9);
        }
        if index / 8 == 1 {
            if board_arr[index + 16] == 0 && board_arr[index + 8] == 0 && !test_check{
                moves.push(index + 16);
            }
        }
        if index >= 32 && index < 40 && !test_check{
            if index % 8 != 7 && board_arr[index + 17] == 0 && last[index + 17] == 6 && board_arr[index + 1] == 6 && last[index + 1] != 6 {
                moves.push(index + 9);
            }
            if index % 8 != 0 && board_arr[index + 15] == 0 && last[index + 15] == 6 && board_arr[index - 1] == 6 && last[index - 1] != 6 {
                moves.push(index + 7);
            }
        }
    }
    return moves;
}

pub fn gen_moves_king(index: usize, board_arr: &Vec<i32>, is_white: bool, tup: (bool, bool, bool, bool)) -> Vec<usize> {
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        if index / 8 != 0 {
            if board_arr[index - 8] <= 0 {
                moves.push(index - 8);
            }
            if index % 8 != 0 {
                if board_arr[index - 9] <= 0 {
                    moves.push(index - 9);
                }
            }
            if index % 8 != 7 {
                if board_arr[index - 7] <= 0 {
                    moves.push(index - 7);
                }
            }
        }
        if index / 8 != 7 {
            if board_arr[index + 8] <= 0 {
                moves.push(index + 8);
            }
            if index % 8 != 7 {
                if board_arr[index + 9] <= 0 {
                    moves.push(index + 9);
                }
            }
            if index % 8 != 0 {
                if board_arr[index + 7] <= 0 {
                    moves.push(index + 7);
                }
            }
        }
        if index % 8 != 7 {
            if board_arr[index + 1] <= 0 {
                moves.push(index + 1);
            }
        }
        if index % 8 != 0 {
            if board_arr[index - 1] <= 0 {
                moves.push(index - 1);
            }
        }
        if tup.0 {
            if board_arr[59] == 0 && board_arr[58] == 0 && board_arr[57] == 0 && !is_check(board_arr, is_white, &Vec::new(), 59) && !is_check(board_arr, is_white, &Vec::new(), 58) && !is_check(board_arr, is_white, &Vec::new(), 59) && !is_check(board_arr, is_white, &Vec::new(), 60) {
                moves.push(58);
            }
        }
        if tup.1 {
            if board_arr[61] == 0 && board_arr[62] == 0 && !is_check(board_arr, is_white, &Vec::new(), 61) && !is_check(board_arr, is_white, &Vec::new(), 60) && !is_check(board_arr, is_white, &Vec::new(), 62) {
                moves.push(62);
            }
        }
    }
    else {
        if index / 8 != 0 {
            if board_arr[index - 8] >= 0 {
                moves.push(index - 8);
            }
            if index % 8 != 0 {
                if board_arr[index - 9] >= 0 {
                    moves.push(index - 9);
                }
            }
            if index % 8 != 7 {
                if board_arr[index - 7] >= 0 {
                    moves.push(index - 7);
                }
            }
        }
        if index / 8 != 7 {
            if board_arr[index + 8] >= 0 {
                moves.push(index + 8);
            }
            if index % 8 != 7 {
                if board_arr[index + 9] >= 0 {
                    moves.push(index + 9);
                }
            }
            if index % 8 != 0 {
                if board_arr[index + 7] >= 0 {
                    moves.push(index + 7);
                }
            }
        }
        if index % 8 != 7 {
            if board_arr[index + 1] >= 0 {
                moves.push(index + 1);
            }
        }
        if index % 8 != 0 {
            if board_arr[index - 1] >= 0 {
                moves.push(index - 1);
            }
        } 
        if tup.2 {
            if board_arr[1] == 0 && board_arr[2] == 0 && board_arr[3] == 0 && !is_check(board_arr, is_white, &Vec::new(), 2) && !is_check(board_arr, is_white, &Vec::new(), 3) && !is_check(board_arr, is_white, &Vec::new(), 4){
                moves.push(2)
            }
        }
        if tup.3 {
            if board_arr[6] == 0 && board_arr[5] == 0 && !is_check(board_arr, is_white, &Vec::new(), 5) && !is_check(board_arr, is_white, &Vec::new(), 4) && !is_check(board_arr, is_white, &Vec::new(), 6){
                moves.push(6)
            }
        }
    }
    
    return moves;
}

pub fn king_index(is_white: bool, board_arr: &Vec<i32>) -> usize {
    if is_white {
        for i in 0..64 {
            if board_arr[i] == 1 {
                return i;
            }
        }
    }
    else {
        for i in 0..64 {
            if board_arr[i] == -1 {
                return i;
            }
        }
    }
    return 64;
}

pub fn is_check(board_arr: &Vec<i32>, is_white: bool, last: &Vec<i32>, index: usize) -> bool {
    let mut moves: Vec<usize>;
    if is_white {
        for i in 0..64 {
            if board_arr[i] <= -1 {
                moves = gen_moves_not_safe(i, board_arr, last);
                for move1 in moves {
                    if move1 == index {
                        return true;
                    }
                }
            }
        }
    }
    else {
        for i in 0..64 {
            if board_arr[i] >= 1 {
                moves = gen_moves_not_safe(i, board_arr, last);
                for move1 in moves {
                    if move1 == index {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

pub fn is_legal(move1: usize, from: usize, board_arr: Vec<i32>, is_white: bool) -> bool {
    let board = move_piece(from, move1, board_arr.clone());
    return !is_check(&board, is_white, &board_arr, king_index(is_white, &board));
}

pub fn move_piece(from: usize, to: usize, mut board_arr: Vec<i32>) -> Vec<i32> {
    let temp = board_arr[from];
    if from == 60 && temp == 1 {
        if to == 58 {
            board_arr[56] = 0;
            board_arr[59] = 3;
        }
        if to == 62 {
            board_arr[63] = 0;
            board_arr[61] = 3;
        }
    }
    if from == 4 && temp == -1 {
        if to == 2 {
            board_arr[0] = 0;
            board_arr[3] = -3;
        }
        if to == 6 {
            board_arr[7] = 0;
            board_arr[5] = -3;
        }
    }
    if board_arr[from] == 6 && from.abs_diff(to) == 7 && board_arr[to] == 0 {
        board_arr[from + 1] = 0;
    }
    if board_arr[from] == 6 && from.abs_diff(to) == 9 && board_arr[to] == 0 {
        board_arr[from - 1] = 0;
    }
    if board_arr[from] == -6 && from.abs_diff(to) == 7 && board_arr[to] == 0 {
        board_arr[from - 1] = 0;
    }
    if board_arr[from] == -6 && from.abs_diff(to) == 9 && board_arr[to] == 0 {
        board_arr[from + 1] = 0;
    }
    board_arr[from] = 0;
    board_arr[to] = temp;
    if to < 8 {
        if temp == 6 {
            board_arr[to] = 2;
        }
    }
    if to > 55 {
        if temp == -6 {
            board_arr[to] = -2;
        }
    }
    return board_arr;
}

pub fn is_stalemate(board_arr: Vec<i32>, is_white: bool, last: &Vec<i32>, tup: (bool, bool, bool, bool)) -> bool {
    let index = king_index(is_white, &board_arr);
    if gen_moves(index, &board_arr, last, tup).is_empty() {
        let mut moves: Vec<usize>;
        if is_white {
            for i in 0..64 {
                if board_arr[i] >= 1 {
                    moves = gen_moves(i, &board_arr, last, tup);
                    if moves.len() > 0 {
                        return false;
                    }
                }
            }
            return true;
        }
        else {
            for i in 0..64 {
                if board_arr[i] <= -1 {
                    moves = gen_moves(i, &board_arr, last, tup);
                    if moves.len() > 0 {
                        return false;
                    }
                }
            }
            return true;
        }
    }
    return false;
}

pub fn is_checkmate(board_arr: Vec<i32>, is_white: bool, last: &Vec<i32>, tup: (bool, bool, bool, bool)) -> bool {
    let index = king_index(is_white, &board_arr);
    return is_check(&board_arr, is_white, last, index) && is_stalemate(board_arr, is_white, last, tup);
}

pub fn gen_moves_not_safe(index: usize, board_arr: &Vec<i32>, last: &Vec<i32>) -> Vec<usize> {
    let moves = match board_arr[index] {
        1 => gen_moves_king(index, board_arr, true, (false, false, true, true)),
        2 => gen_moves_queen(index, board_arr, true),
        3 => gen_moves_rook(index, board_arr, true),
        4 => gen_moves_bishop(index, board_arr, true),
        5 => gen_moves_knight(index, board_arr, true),
        6 => gen_moves_pawn(index, board_arr, true, last, true),
        -1 => gen_moves_king(index, board_arr, false, (false, false, true, true)),
        -2 => gen_moves_queen(index, board_arr, false),
        -3 => gen_moves_rook(index, board_arr, false),
        -4 => gen_moves_bishop(index, board_arr, false),
        -5 => gen_moves_knight(index, board_arr, false),
        -6 => gen_moves_pawn(index, board_arr, false, last, true),
        _ => Vec::new(),
    };
    return moves;
}

pub fn gen_moves(index: usize, board_arr: &Vec<i32>, last: &Vec<i32>, tup: (bool, bool, bool, bool)) -> Vec<usize> {
    let mut moves = match board_arr[index] {
        1 => gen_moves_king(index, board_arr, true, tup),
        2 => gen_moves_queen(index, board_arr, true),
        3 => gen_moves_rook(index, board_arr, true),
        4 => gen_moves_bishop(index, board_arr, true),
        5 => gen_moves_knight(index, board_arr, true),
        6 => gen_moves_pawn(index, board_arr, true, last, false),
        -1 => gen_moves_king(index, board_arr, false, tup),
        -2 => gen_moves_queen(index, board_arr, false),
        -3 => gen_moves_rook(index, board_arr, false),
        -4 => gen_moves_bishop(index, board_arr, false),
        -5 => gen_moves_knight(index, board_arr, false),
        -6 => gen_moves_pawn(index, board_arr, false, last, false),
        _ => Vec::new(),
    };
    let mut remove = Vec::new();
    for i in 0..moves.len() {
        if !is_legal(moves[i], index, board_arr.clone(), board_arr[index] > 0) {
            remove.push(i);
        }
    }
    for i in remove.into_iter().rev() {
        moves.remove(i);
    }
    return moves;
}

pub fn get_mouse_pos() -> usize {
    let mouse_pos = mouse_position();
    return mouse_pos.0 as usize / 100 + mouse_pos.1 as usize / 100 * 8;
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "chess".to_owned(),
        window_height: 800,
        window_width: 800,
        window_resizable: false,
        ..Default::default()
    }
}

pub fn copy_image(image: &DynamicImage) -> Image {
    let dim = image.dimensions();
    let mut image1 = Image::gen_image_color(dim.0 as u16, dim.1 as u16, GREEN);
    for x in 0..dim.0 {
        for y in 0..dim.1 {
            let pixel = image.get_pixel(x, y);
            image1.set_pixel(x, y, Color {r: pixel.0[0] as f32 / 255.0, g: pixel.0[1] as f32 / 255.0, b: pixel.0[2] as f32 / 255.0, a: pixel.0[3] as f32 / 255.0})
        }
    }
    return image1;
}
//board array, board, king-1, queen-2, rook-3, bishop-4, horse-5, pawn-6. negative for black
pub fn restart() -> (Vec<i32>, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D) {
    let board_arr = vec![-3, -5, -4, -2, -1, -4, -5, -3, -6, -6, -6, -6, -6, -6 ,-6 ,-6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 6, 6, 6, 6, 6, 6, 3, 5, 4, 2, 1, 4, 5, 3];
    // let board_arr = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let board = Texture2D::from_file_with_format(include_bytes!("board.png"),None,);
    let wking = Texture2D::from_file_with_format(include_bytes!("white_king.png"),None,);
    let wqueen = Texture2D::from_file_with_format(include_bytes!("white_queen.png"),None,);
    let wrook = Texture2D::from_file_with_format(include_bytes!("white_rook.png"),None,);
    let wbishop = Texture2D::from_file_with_format(include_bytes!("white_bishop.png"),None,);
    let wknight = Texture2D::from_file_with_format(include_bytes!("white_knight.png"),None,);
    let wpawn = Texture2D::from_file_with_format(include_bytes!("white_pawn.png"),None,);
    let bking = Texture2D::from_file_with_format(include_bytes!("black_king.png"),None,);
    let bqueen = Texture2D::from_file_with_format(include_bytes!("black_queen.png"),None,);
    let brook = Texture2D::from_file_with_format(include_bytes!("black_rook.png"),None,);
    let bbishop = Texture2D::from_file_with_format(include_bytes!("black_bishop.png"),None,);
    let bknight = Texture2D::from_file_with_format(include_bytes!("black_knight.png"),None,);
    let bpawn = Texture2D::from_file_with_format(include_bytes!("black_pawn.png"),None,);
    return (board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn);
}

pub fn draw_board(board_arr: &Vec<i32>, board: Texture2D, wking: Texture2D, wqueen: Texture2D, wrook: Texture2D, wbishop: Texture2D, wknight: Texture2D, wpawn: Texture2D, bking: Texture2D, bqueen: Texture2D, brook: Texture2D, bbishop: Texture2D, bknight: Texture2D, bpawn: Texture2D) {
    draw_texture(board, 0.0, 0.0, WHITE);
    for i in 0..board_arr.len() {
        match board_arr[i] {
            1 => draw_texture(wking, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            2 => draw_texture(wqueen, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            3 => draw_texture(wrook, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            4 => draw_texture(wbishop, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            5 => draw_texture(wknight, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            6 => draw_texture(wpawn, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -1 => draw_texture(bking, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -2 => draw_texture(bqueen, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -3 => draw_texture(brook, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -4 => draw_texture(bbishop, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -5 => draw_texture(bknight, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -6 => draw_texture(bpawn, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            _ => (),
        }
    }
}

pub fn draw_move(moves: Vec<usize>, board_arr: &Vec<i32>) {
    for i in moves {
        if board_arr[i] == 0 {
            draw_circle(50.0 + (i % 8 * 100) as f32, 50.0 + (i / 8 * 100) as f32, 20.0, Color{r: 0.4, g: 0.4, b: 0.4, a: 0.5});
        }
        else{
            if board_arr[i] == -1 || board_arr[i] == 1 {
                draw_circle_lines(50.0 + (i % 8 * 100) as f32, 50.0 + (i / 8 * 100) as f32, 49.0, 7.0, RED);
            }
            else {
                draw_circle_lines(50.0 + (i % 8 * 100) as f32, 50.0 + (i / 8 * 100) as f32, 49.0, 7.0, Color{r: 0.4, g: 0.4, b: 0.4, a: 0.5});
            }
        }
    }
}

pub fn player_turn2(mut board_arr: Vec<i32>, mut last: Vec<i32>, mut is_white_turn: bool, mut is_pressed: bool, mut found: bool, mut moves: Vec<usize>, mut chosen: usize, mut rep: i32, mut save: Vec<Vec<i32>>, mut tup: (bool, bool, bool, bool), mut counter: i32) -> (Vec<i32>, Vec<i32>, bool, bool, bool, Vec<usize>, usize, i32, Vec<Vec<i32>>, (bool, bool, bool, bool), i32) {
    if is_pressed {
        let pos = get_mouse_pos();
        for i in moves.clone() {
            if pos == i {
                last = board_arr.clone();
                if board_arr[chosen] == 6 || board_arr[chosen] == -6 || board_arr[pos] != 0 {
                    counter = 0;
                }
                else {
                    if !is_white_turn{
                        counter += 1;
                    }
                }
                board_arr = move_piece(chosen, pos, board_arr);
                if is_white_turn {
                    if tup.0 == true || tup.1 == true {
                        if chosen == 60 {
                            tup.0 = false;
                            tup.1 = false;
                        }
                    }
                    if tup.0 == true {
                        if chosen == 56 {
                            tup.0 = false;
                        }
                    }
                    if tup.1 == true {
                        if chosen == 63 {
                            tup.1 = false;
                        }
                    }
                }
                else {
                    if tup.2 == true || tup.3 == true {
                        if chosen == 4 {
                            tup.2 = false;
                            tup.3 = false;
                        }
                    }
                    if tup.2 == true {
                        if chosen == 0 {
                            tup.2 = false;
                        }
                    }
                    if tup.3 == true {
                        if chosen == 7 {
                            tup.3 = false;
                        }
                    }
                }
                save.push(board_arr.clone());
                if save.len() > 4 && !is_white_turn {
                    if board_arr == save[save.len() - 5] {
                        rep += 1;
                    }
                    else {
                        rep = 0;
                    }
                }
                found = true;
                break;
            }
        }
        if found {
            found = false;
            is_pressed = false;
            is_white_turn = !is_white_turn;
        }
        else {            
            moves = Vec::new();
            is_pressed = false;
            if (board_arr[pos] > 0) == is_white_turn {
                chosen = pos;
                moves = gen_moves(pos, &board_arr, &last, tup);
                draw_move(moves.clone(), &board_arr);
                is_pressed = true;
            }
        }
    }
    else {
        let pos = get_mouse_pos();
        if (board_arr[pos] > 0) == is_white_turn {
            chosen = pos;
            moves = gen_moves(pos, &board_arr, &last, tup);
            is_pressed = true;
        }
    }
    return (board_arr, last, is_white_turn, is_pressed, found, moves, chosen, rep, save, tup, counter);
}

pub fn is_insufficient(board_arr: &Vec<i32>) -> bool {
    let mut white = false;
    let mut black = false;
    let tup_white = count_pieces(board_arr, true);
    let tup_black = count_pieces(board_arr, false);
    if tup_white.2 == 1 || tup_white.3 ==1 {
        white = true;
    }
    if tup_white.2 == 1 && tup_white.3 ==1 {
        white = false;
    }
    if tup_white.0 > 0 || tup_white.1 > 0 || tup_white.4 > 0 {
        white = false;
    }
    if tup_black.2 == 1 || tup_black.3 ==1 {
        black = true;
    }
    if tup_black.2 == 1 && tup_black.3 ==1 {
        black = false;
    }
    if tup_black.0 > 0 || tup_black.1 > 0 || tup_black.4 > 0 {
        black = false;
    }
    black && white
}

pub fn count_pieces(board_arr: &Vec<i32>, is_white: bool) -> (i32, i32, i32, i32, i32) {
    let mut tup = (0, 0, 0, 0, 0);
    if is_white {
        for i in board_arr{
            if i == &2 {
                tup.0 += 1;
            }
            if i == &3 {
                tup.1 += 1;
            }
            if i == &4 {
                tup.2 += 1;
            }
            if i == &5 {
                tup.3 += 1;
            }
            if i == &6 {
                tup.4 += 1;
            }
        }
    }
    else {
        for i in board_arr{
            if i == &-2 {
                tup.0 += 1;
            }
            if i == &-3 {
                tup.1 += 1;
            }
            if i == &-4 {
                tup.2 += 1;
            }
            if i == &-5 {
                tup.3 += 1;
            }
            if i == &-6 {
                tup.4 += 1;
            }
        }
    }
    return tup;
}