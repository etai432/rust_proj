use neuroflow::FeedForward;
use neuroflow::data::DataSet;
use neuroflow::activators::Type::Tanh;
use neuroflow::io;

pub fn run() {
    // let mut nn = FeedForward::new(&[2, 2, 1, 1]);
    // let mut data: DataSet = DataSet::from_csv("src/NN_rust/addition.csv").unwrap();
    // nn.activation(Tanh)
    //  .learning_rate(0.01)
    //  .train(&data, 1000);
    // io::save(&nn, "src/NN_rust/addition.flow").unwrap();
    let mut nn: FeedForward = io::load("src/NN_rust/addition.flow").unwrap();
    println!("result: {}", nn.calc(&[0.2, 0.5])[0] * 20.0);


}