// mod game_of_life;
// use game_of_life::game;
// mod chess;
// use chess::chess_pvp::chess_pvp;
mod NN_rust;
use NN_rust::NN;
use NN_rust::make_csv;

fn main() {
    // make_csv::main();
    NN::run();
}