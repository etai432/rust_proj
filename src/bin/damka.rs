#![recursion_limit = "10000"]
use std::{collections::HashMap, any::{type_name, Any}};
use std::thread;

struct Damka {
    board: Vec<i8>,
    moves: Vec<i8>,
    kills: Vec<i8>,
    players2: Vec<i8>,
    players0: Vec<i8>,
    turn: Vec<i8>,
    doubles: Vec<(i8, i8)>,
    map: HashMap<String, f32>
}
impl Damka {
    fn new() -> Damka {
        Damka {
            board: Vec::new(),
            moves: Vec::new(),
            kills: Vec::new(),
            players2: Vec::new(),
            players0: Vec::new(),
            turn: Vec::new(),
            doubles: Vec::new(),
            map: HashMap::new()
        }
    }
    fn restart_board(&mut self) {
        let arr = [3, 0, 3, 0, 3, 0, 3, 0, 0, 3, 0, 3, 0, 3, 0, 3, 3, 0, 3, 0, 3, 0, 3, 0, 1, 3, 1, 3, 1, 3, 1, 3, 3, 1, 3, 1, 3, 1, 3, 1, 2, 3, 2, 3, 2, 3, 2, 3, 3, 2, 3, 2, 3, 2, 3, 2, 2, 3, 2, 3, 2, 3, 2, 3];
        self.board = Vec::new();
        for i in arr.iter() {
            self.board.push(i8::from(*i));
        }
    }

    fn check_win(&self, board:&Vec<i8>) -> i32 {
        for i in 0..8 {
            if board[i as usize] == 2 {
                return 2;
            }
            if board[(63 - i) as usize] == 0 {
                return 0;
            }
        }
        return 1;
    }

    fn is_tie(&mut self, turn:i32, board:&mut Vec<i8>) -> bool {
        self.players(turn, board);
        if turn == 0 {
            for i in self.players0.clone().iter() {
                self.gen_all_moves(*i as i32, board);
                if self.turn.len() == 0 {
                    return true;
                }
            }
        }
        else {
            for i in self.players2.clone().iter() {
                self.gen_all_moves(*i as i32, board);
                if self.turn.len() == 0 {
                    return true;
                }
            }
        }
        return false;
    }

    fn print_board(&self, board:&Vec<i8>) {
        println!("the board:");
        for i in 0..64 {
            if board[i] == 2 {
                print!("|2|");
            } else if board[i] == 0 {
                print!("|0|");
            } else {
                print!("| |");
            }
            if i % 8 == 7 {
                println!("");
            }
        }
    }

    fn count_troops(&self, board:&Vec<i8>) -> (i32, i32) {
        let mut troop0 = 0;
        let mut troop2 = 0;
        for i in board.iter() {
            if i == &0 {
                troop0 += 1;    
            }
            if i == &2 {
                troop2 += 1;
            }
        }
        (troop0, troop2)
    }

    fn players(&mut self, turn:i32, board:&Vec<i8>) {
        self.players0 = Vec::new();
        self.players2 = Vec::new();
        if turn == 0 {
            for i in 0..64 {
                if board[i] == 0 {
                    self.players0.push(i as i8);
                }
            }
        }
        else {
            for i in 0..64 {
                if board[i] == 2 {
                    self.players2.push(i as i8);
                }
            }
        }
    }

    fn gen_moves(&mut self, place:i32, board:&Vec<i8>) {
        self.moves = Vec::new();
        if board[place as usize] == 0 {
            if place < 57 && place % 8 != 0 && board[(place + 7) as usize] == 1 {
                self.moves.push((place + 7) as i8);
            }
            if place < 55 && place % 8 != 7 && board[(place + 9) as usize] == 1 {
                self.moves.push((place + 9) as i8);
            }
        }
        else {
            if place > 8 && place % 8 != 0 && board[(place - 9) as usize] == 1 {
                self.moves.push((place - 9) as i8);
            }
            if place > 6 && place % 8 != 7 && board[(place - 7) as usize] == 1 {
                self.moves.push((place - 7) as i8);
            }
        }
    }

    fn gen_kills(&mut self, place:i32, board:&Vec<i8>) {
        self.kills = Vec::new();
        if board[place as usize] == 0 {
            if place < 50 && place % 8 != 0 && place % 8 != 1 && board[(place + 7) as usize] == 2 && board[(place + 14) as usize] == 1 {
                self.kills.push((place + 14) as i8);
            }
            if place < 46 && place % 8 != 7 && place % 8 != 6 && board[(place + 9) as usize] == 2 && board[(place + 18) as usize] == 1 {
                self.kills.push((place + 18) as i8);
            }
        } else {
            if place > 17 && place % 8 != 0 && place % 8 != 1 && board[(place - 9) as usize] == 0 && board[(place - 18) as usize] == 1 {
                self.kills.push((place - 18) as i8);
            }
            if place > 13 && place % 8 != 7 && place % 8 != 6 && board[(place - 7) as usize] == 0 && board[(place - 14) as usize] == 1 {
                self.kills.push((place - 14) as i8);
            }
        }
    }

    fn gen_more_kills(&mut self, place:i32, kill_list:&Vec<i8>, board:&mut Vec<i8>) -> Vec<(i8, i8)> {
        let mut doubles:Vec<(i8, i8)> = Vec::new();
        let mut extra: Vec<(i8,i8)>;
        let mut player: i8;
        for i in kill_list.iter() {
            player = i8::from(board[place as usize]);
            board[*i as usize] = player;
            board[place as usize] = 1;
            board[((i + place as i8)/2) as usize] = 1;
            if i < &50 && i % 8 != 0 && i % 8 != 1 && board[(*i + 7) as usize] == (player-2).abs() && board[(*i + 14) as usize] == 1 {
                self.doubles.push((*i, i + 14));
                extra = self.gen_more_kills(*i as i32, &vec![i + 14], board);
                for j in extra.iter(){
                    doubles.push(*j);
                }
            }
            if i < &46 && i % 8 != 7 && i % 8 != 6 && board[(*i + 9) as usize] == (player-2).abs() && board[(*i + 18) as usize] == 1 {
                self.doubles.push((*i, i + 18));
                extra = self.gen_more_kills(*i as i32, &vec![i + 18], board);
                for j in extra.iter(){
                    doubles.push(*j);
                }
            }
            if i > &17 && i % 8 != 0 && i % 8 != 1 && board[(*i - 9) as usize] == (player-2).abs() && board[(*i - 18) as usize] == 1 {
                self.doubles.push((*i, i - 18));
                extra = self.gen_more_kills(*i as i32, &vec![i - 18], board);
                for j in extra.iter(){
                    doubles.push(*j);
                }
            }
            if i > &13 && i % 8 != 7 && i % 8 != 6 && board[(*i - 7) as usize] == (player-2).abs() && board[(*i - 14) as usize] == 1 {
                self.doubles.push((*i, i - 14));
                extra = self.gen_more_kills(*i as i32, &vec![i - 14], board);
                for j in extra.iter(){
                    doubles.push(*j);
                }
            }
            board[*i as usize] = 1;
            board[place as usize] = player;
            board[((i + place as i8)/2) as usize] = (2-player).abs();
        }
        return doubles;
    }

    fn gen_all_moves(&mut self, place:i32, board:&mut Vec<i8>) {
        self.turn = Vec::new();
        self.gen_moves(place, board);
        for i in self.moves.iter() {
            self.turn.push(*i);
        }
        self.gen_kills(place, board);
        for i in self.kills.iter() {
            self.turn.push(*i);
        }
        self.gen_more_kills(place, &self.kills.clone(), board);
    }

    fn move1(&self, place:i32, goal:i32, board:&mut Vec<i8>) {
        let mut bool1 = false;
        for i in self.turn.iter() {
            if goal == *i as i32 {
                bool1 = true;
                let player = i8::from(board[place as usize]);
                if (goal-place).abs() > 10 {
                    board[goal as usize] = player;
                    board[place as usize] = 1;
                    board[((goal + place)/2) as usize] = 1;
                } else {
                    board[place as usize] = 1;
                    board[goal as usize] = player;
                }
            }
        }
        if !bool1 {
            self.move2(place, goal, -1, board);
        }
    }

    fn move2(&self, place:i32, goal:i32, last:i32, board:&mut Vec<i8>) {
        let mut bool1 = false;
        for i in self.doubles.iter() {
            if last != i.1 as i32 && i.1 as i32 == goal {
                bool1 = true;
                self.move2(place, i.0 as i32, i.0 as i32, board);
                let player = board[i.0 as usize];
                board[i.0 as usize] = 1;
                board[((i.1 + i.0)/2) as usize] = 1;
                board[i.1 as usize] = player;
            }
        }
        if !bool1 {
            self.move1(place, goal, board)
        }
    }

    fn dfs(&mut self, board:& mut Vec<i8>, turn:i32) -> f32 {
        self.print_board(&board);
        println!("{}", turn);
        let win = self.check_win(&board);
        let mut bool1 = false;
        let mut total_score = 0.0;
        let mut counter = 0.0;
        let mut copy_board: Vec<i8>;
        let mut str1:String;
        let mut tup: (i32, i32);
        let mut score: f32;
        if win != 1 {
            str1 = String::from(&(board.clone().into_iter().map(|i| i.to_string()).collect::<String>().replace("3", "").replace(",", "").replace(" ", "")));
            if !self.map.contains_key(&str1) {
                tup = self.count_troops(&board);
                score = win as f32 / 2.0;
                self.map.insert(str1, score + (tup.1 - tup.0) as f32 / 10.0);
            }
            return (win / 2) as f32 * 0.92;
        }
        if turn == 2 {
            self.players(2, &board);
            for player in self.players2.clone() {
                self.gen_all_moves(player as i32, board);
                for move3 in self.turn.clone().iter() {
                    counter += 1.0;
                    copy_board = board.clone();
                    self.move1(player as i32, *move3 as i32, &mut copy_board);
                    if &copy_board != board {
                        bool1 = true;
                    }
                    str1 = String::from(&(copy_board.clone().into_iter().map(|i| i.to_string()).collect::<String>().replace("3", "").replace(",", "").replace(" ", "")));
                    if self.map.contains_key(&str1) {
                        return self.map.get(&str1).unwrap() * 0.92;
                    } else {
                        tup = self.count_troops(&copy_board);
                        score = self.dfs(&mut copy_board.clone(), 0);
                        self.map.insert(str1, score + (tup.1 - tup.0) as f32 / 10.0);
                    }
                }
                for move3 in self.doubles.clone().iter() {
                    copy_board = board.clone();
                    self.move2(player as i32, move3.1 as i32, -1, &mut copy_board);
                    str1 = String::from(&(copy_board.clone().into_iter().map(|i| i.to_string()).collect::<String>().replace("3", "").replace(",", "").replace(" ", "")));
                    if self.map.contains_key(&str1) {
                        return self.map.get(&str1).unwrap() * 0.92;
                    } else {
                        tup = self.count_troops(&copy_board);
                        score = self.dfs(&mut copy_board.clone(), 0);
                        self.map.insert(str1, score + (tup.1 - tup.0) as f32 / 10.0);
                    }
                    counter += 1.0;
                }
            }
        } else {
            self.players(0, &board);
            for player in self.players0.clone() {
                self.gen_all_moves(player as i32, board);
                for move3 in self.turn.clone().iter() {
                    counter += 1.0;
                    copy_board = board.clone();
                    self.move1(player as i32, *move3 as i32, &mut copy_board);
                    if &copy_board != board {
                        bool1 = true;
                    }
                    str1 = String::from(&(copy_board.clone().into_iter().map(|i| i.to_string()).collect::<String>().replace("3", "").replace(",", "").replace(" ", "")));
                    if self.map.contains_key(&str1) {
                        return self.map.get(&str1).unwrap() * 0.92;
                    } else {
                        tup = self.count_troops(&copy_board);
                        score = self.dfs(&mut copy_board.clone(), 2);
                        self.map.insert(str1, score + (tup.1 - tup.0) as f32 / 10.0);
                    }
                }
                for move3 in self.doubles.clone().iter() {
                    copy_board = board.clone();
                    self.move2(player as i32, move3.1 as i32, -1, &mut copy_board);
                    str1 = String::from(&(copy_board.clone().into_iter().map(|i| i.to_string()).collect::<String>().replace("3", "").replace(",", "").replace(" ", "")));
                    if self.map.contains_key(&str1) {
                        return self.map.get(&str1).unwrap() * 0.92;
                    } else {
                        tup = self.count_troops(&copy_board);
                        score = self.dfs(&mut copy_board.clone(), 2);
                        self.map.insert(str1, score + (tup.1 - tup.0) as f32 / 10.0);
                    }
                    counter += 1.0;
                }
            }
        }
        if !bool1 {
            str1 = String::from(&(board.clone().into_iter().map(|i| i.to_string()).collect::<String>().replace("3", "").replace(",","").replace(" ", "")));
                    if !self.map.contains_key(&str1) {
                        tup = self.count_troops(&board);
                        score = 0.5;
                        self.map.insert(str1, score + (tup.1 - tup.0) as f32 / 10.0);
                    }
            return 0.46;
        }
        return total_score / counter * 0.92;
    }
} 
fn main() {
    let mut damka = Damka::new();
    damka.restart_board();
    damka.dfs(& mut damka.board.clone(), 2);
    println!("done");
    println!("{:?}", damka.map)
    // damka.board[33] = 0;
    // damka.board[10] = 1;
    // damka.print_board();
    // damka.gen_all_moves(42, &mut damka.board.clone());
    // println!("{:?}", damka.doubles);
    // println!("{:?}", damka.turn);
    // let mut board = damka.board.clone();
    // damka.move1(42, 28, &mut board);
    // damka.board = board;
    // damka.print_board();
}