use rand::prelude::*;
// use std::fs;
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
use neural_network::neurons::Activation;
use neural_network::neurons::Loss;
// mod etai;

fn main() {
    // let data = fs::read_to_string(r"src\etai\test.et").unwrap();
    // println!("{:?}", etai::lexer::tokenize(data));

    let mut n = neural_network::neurons::Network::load(r"src\neural_network\addition.et");
    n.summary();
    // println!("{:?}", n);

    // let mut n = neural_network::neurons::Network::new(
    //     vec![2, 8, 1],
    //     0.0000001,
    //     vec![Activation::Linear, Activation::Linear, Activation::Linear],
    //     Loss::MSE,
    // );
    let mut rng = rand::thread_rng();
    // for i in 0..100 {
    let x: Vec<Vec<f32>> = (0..100)
        .map(|_| vec![rng.gen_range(0..1000) as f32, rng.gen_range(0..1000) as f32])
        .collect();
    let y: Vec<Vec<f32>> = (0..100).map(|i| vec![x[i][0] + x[i][1]]).collect();
    // println!("{:?}", n.eval(x, y));
    n.best_lr(x, y);
    println!("{:?}", n);
    // println!("avg loss: {}", n.fit(x, y));
    // }
    println!(
        "prediction: {}, result: {}",
        n.predict(vec![503.9, 407.33]).unwrap()[0],
        503.9 + 407.33
    );

    // n.save(r"src\neural_network\addition.et");
    // println!("{:?}", n);
    // chess_pvp();
}
