use std::collections::HashMap;

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

    fn print_board(&self) {
        println!("the board:");
        for i in 0..64 {
            if self.board[i] == 2 {
                print!("|2|");
            } else if self.board[i] == 0 {
                print!("|0|");
            } else {
                print!("| |");
            }
            if i % 8 == 7 {
                println!("");
            }
        }
    }

    fn count_troops(&self) -> (i32, i32) {
        let mut troop0 = 0;
        let mut troop2 = 0;
        for i in self.board.iter() {
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
        if turn == 0 {
            for i in 0..64 {
                if board[i] == 0 {
                    self.players0.push(board[i]);
                }
            }
        }
        else {
            for i in 0..64 {
                if board[i] == 0 {
                    self.players2.push(board[i]);
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
            if place < 17 && place % 8 != 0 && place % 8 != 1 && board[(place - 9) as usize] == 2 && board[(place - 18) as usize] == 1 {
                self.kills.push((place - 18) as i8);
            }
            if place < 13 && place % 8 != 7 && place % 8 != 6 && board[(place - 7) as usize] == 2 && board[(place - 14) as usize] == 1 {
                self.kills.push((place - 14) as i8);
            }
        }
    }

    fn gen_more_kills(&mut self, place:i32, kill_list:&Vec<i8>, board:&mut Vec<i8>) -> Vec<i8> {
        let mut doubles:Vec<i8> = Vec::new();
        let mut extra: Vec<i8> = Vec::new();
        let mut player: i8;
        for i in kill_list.iter() {
            player = i8::from(board[place as usize]);
            board[*i as usize] = player;
            board[place as usize] = 1;
            board[((i + place as i8)/2) as usize] = 1;
            if i < &50 && i % 8 != 0 && i % 8 != 1 && board[(*i + 7) as usize] == 2 && board[(*i + 14) as usize] == 1 {
                self.doubles.push((*i, i + 14));
                extra = self.gen_more_kills(*i as i32, &vec![i + 14], board);
                for j in extra.iter(){
                    doubles.push(*j);
                }
            }
            if i < &46 && i % 8 != 7 && i % 8 != 6 && board[(*i + 9) as usize] == 2 && board[(*i + 18) as usize] == 1 {
                self.doubles.push((*i, i + 18));
                extra = self.gen_more_kills(*i as i32, &vec![i + 18], board);
                for j in extra.iter(){
                    doubles.push(*j);
                }
            }
            if i < &17 && i % 8 != 0 && i % 8 != 1 && board[(*i - 9) as usize] == 2 && board[(*i - 18) as usize] == 1 {
                self.doubles.push((*i, i - 18));
                extra = self.gen_more_kills(*i as i32, &vec![i - 18], board);
                for j in extra.iter(){
                    doubles.push(*j);
                }
            }
            if i < &13 && i % 8 != 7 && i % 8 != 6 && board[(*i - 7) as usize] == 2 && board[(*i - 14) as usize] == 1 {
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
} 
fn main() {
    let mut damka = Damka::new();
    damka.restart_board();
    damka.print_board();
}
