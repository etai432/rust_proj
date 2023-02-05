use rand::prelude::*;
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
    let mut n = neural_network::neurons::Network::new(vec![2, 32, 1]);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        let x: Vec<Vec<f32>> = (0..100).map(|_| vec![rng.gen_range(0..10) as f32, rng.gen_range(0..10) as f32]).collect();
        let y: Vec<Vec<f32>> = (0..100).map(|i| vec![x[i][0] + x[i][1]]).collect();
        println!("{}", n.fit(x, y));
    }
    println!("{}", n.predict(vec![3.0, 4.0]).unwrap()[0]);
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