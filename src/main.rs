// use rand::prelude::*;
// use rand::seq::SliceRandom;
// use std::fs;
// mod game_of_life;
// use game_of_life::game;
mod chess;
use chess::chess_pvp::chess_pvp;
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
// mod neural_network;
// use neural_network::neurons::Activation;
// use neural_network::neurons::Loss;
// use std::f32::consts::PI;
// mod etai;

fn main() {
    chess_pvp()
    // let mut n = neural_network::neurons::Network::load(r"src/neural_network/sine.et");
    // n.summary();
    // println!("{:?}", n);

    // let mut n = neural_network::neurons::Network::new(
    //     vec![1, 16, 8, 1],
    //     0.00001,
    //     vec![
    //         Activation::Linear,
    //         Activation::Relu,
    //         Activation::Relu,
    //         Activation::Linear,
    //     ],
    //     Loss::MSE,
    // );

    // let data_num = 10000.0;
    // let jump = 2.0 * PI / data_num;
    // let mut num = 0.0;
    // let mut data: Vec<(Vec<f32>, Vec<f32>)> = Vec::new();
    // for i in 0..data_num as usize {
    //     data.push((vec![num], vec![num.sin()]));
    //     num += jump;
    // }
    // let mut rng = rand::thread_rng();
    // data.shuffle(&mut rng);
    // let x: Vec<Vec<f32>> = data.iter().map(|(x, _)| x.clone()).collect();
    // let y: Vec<Vec<f32>> = data.iter().map(|(_, y)| y.clone()).collect();

    // (0..10)
    //     .map(|_| n.fit(x.clone(), y.clone()))
    //     .collect::<Vec<f32>>();

    // println!(
    //     "prediction: {}, result: {}",
    //     n.predict(vec![3.56]).unwrap()[0],
    //     3.56_f32.sin()
    // );
    // println!(
    //     "prediction: {}, result: {}",
    //     n.predict(vec![0.56]).unwrap()[0],
    //     0.56_f32.sin()
    // );

    // n.save(r"src\neural_network\sine.et");
    // let data = fs::read_to_string(r"src\etai\test.et").unwrap();
    // println!("{:?}", etai::lexer::tokenize(data));

    // n.save(r"src\neural_network\mnist.et");
    // println!("{:?}", n);
    // chess_pvp();
}
