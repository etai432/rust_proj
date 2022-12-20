use std::cmp::min;

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

pub fn gen_moves_pawn(index: usize, board_arr: &Vec<i32>, is_white: bool, last: &Vec<i32>) -> Vec<usize> {
    //add en passant
    let mut moves: Vec<usize> = Vec::new();
    if is_white {
        if board_arr[index - 8] == 0 {
            moves.push(index - 8);
        }
        if board_arr[index - 7] < 0 {
            moves.push(index - 7);
        }
        if board_arr[index - 9] < 0 {
            moves.push(index - 9);
        }
        if index / 8 == 6 {
            if board_arr[index - 16] == 0 {
                moves.push(index - 16);
            }
        }
    }
    else {
        if board_arr[index + 8] == 0 {
            moves.push(index + 8);
        }
        if board_arr[index + 7] > 0 {
            moves.push(index + 7);
        }
        if board_arr[index + 9] > 0 {
            moves.push(index + 9);
        }
        if index / 8 == 1 {
            if board_arr[index + 16] == 0 {
                moves.push(index + 16);
            }
        }
    }
    return moves;
}

pub fn gen_moves_king(index: usize, board_arr: &Vec<i32>, is_white: bool) -> Vec<usize> {
    todo!();
}

pub fn is_check(board_arr: Vec<i32>, is_white: bool, last: &Vec<i32>) -> bool {
    let mut index = 64;
    let mut moves = Vec::new();
    if is_white {
        for i in 0..64 {
            if board_arr[i] == 1 {
                index = i;
                break;
            }
        }
        for i in 0..64 {
            if board_arr[i] < -1 {
                moves = gen_moves(i, board_arr.clone(), last);
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
            if board_arr[i] == -1 {
                index = i;
                break;
            }
        }
        for i in 0..64 {
            if board_arr[i] > 1 {
                moves = gen_moves(i, board_arr.clone(), last);
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
    return !is_check(move_piece(from, move1, board_arr.clone()), is_white, &board_arr);
}

pub fn move_piece(from: usize, to: usize, board_arr: Vec<i32>) -> Vec<i32> {
    todo!();
}

pub fn is_checkmate(board_arr: Vec<i32>, is_white: bool, last: &Vec<i32>) -> bool {
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
    if gen_moves_king(index, &board_arr, is_white).is_empty() {
        let mut moves = Vec::new();
        if is_white {
            for i in 0..64 {
                if board_arr[i] > 1 {
                    moves = gen_moves(i, board_arr.clone(), last);
                    for move1 in moves {
                        if is_legal(move1, i, board_arr.clone(), is_white) {
                            return false;
                        }
                    }
                }
            }
            return true;
        }
        else {
            for i in 0..64 {
                if board_arr[i] < -1 {
                    moves = gen_moves(i, board_arr.clone(), last);
                    for move1 in moves {
                        if is_legal(move1, i, board_arr.clone(), is_white) {
                            return false;
                        }
                    }
                }
            }
            return true;
        }
    }
    return false;
}

pub fn gen_moves(index: usize, board_arr: Vec<i32>, last: &Vec<i32>) -> Vec<usize> {
    let mut moves = match board_arr[index] {
        1 => gen_moves_king(index, &board_arr, true),
        2 => gen_moves_queen(index, &board_arr, true),
        3 => gen_moves_rook(index, &board_arr, true),
        4 => gen_moves_bishop(index, &board_arr, true),
        5 => gen_moves_knight(index, &board_arr, true),
        6 => gen_moves_pawn(index, &board_arr, true, last),
        -1 => gen_moves_king(index, &board_arr, false),
        -2 => gen_moves_queen(index, &board_arr, false),
        -3 => gen_moves_rook(index, &board_arr, false),
        -4 => gen_moves_bishop(index, &board_arr, false),
        -5 => gen_moves_knight(index, &board_arr, false),
        -6 => gen_moves_pawn(index, &board_arr, false, last),
        _ => Vec::new(),
    };
    for i in 0..moves.len() {
        if !is_legal(moves[i], index, board_arr.clone(), board_arr[index] > 0) {
            moves.remove(i);
        }
    }
    return moves;
}

// draw_move(gen_moves(8, board_arr.clone(), &board_arr), &board_arr); //compiler crash 1.66.0 stable