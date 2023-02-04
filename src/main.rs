// mod game_of_life;
// use game_of_life::game;
// mod chess;
// use chess::chess_pvp::chess_pvp;
// use chess::chess_pvai::pvai;
// mod file_renamer;
// mod learning_rust;
// use learning_rust::macros;
// mod thread_pool
// mod time_utils;
// mod random_utils;
// use crate::timeit;
// mod ray_tracer;
// use ray_tracer::screen::run;
mod neural_network;

fn main() {
    let mut n = neural_network::neurons::Network::new(vec![2, 1]);
    println!("{:?}", n.predict(vec![0.5, 0.1]));
    println!("{:?}", n);
    // run();
    // time_utils::test();
    // timeit!();
    // pvai();
    // chess_pvp();
    // file_renamer::run()
    // macros::run();
    // thread_pool::run();
}