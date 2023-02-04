use rand::{prelude::*, seq::index};
#[derive(Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
    pub value: f32,
    pub place: (usize, usize),//layer, index in layer
}

impl Neuron {
    pub fn new(len: usize, place: (usize, usize)) -> Neuron {
        Neuron {
            bias: rand::thread_rng().gen(),
            weights: (0..len).map(|_| rand::thread_rng().gen()).collect(),
            value: 0.0,
            place: place,
        }
    }

    pub fn update() {//updates weights and bias
        todo!()
    }
}

#[derive(Debug)]
pub struct Network {
    pub neurons: Vec<Vec<Neuron>>,//vector of layers
    pub learning_rate: f32,
}

impl Network {
    pub fn new(size: Vec<usize>) -> Network {
        let mut neurons: Vec<Vec<Neuron>> = Vec::new();
        let mut layer_index = size.len();
        let mut next = 0;
        for layer in size.into_iter().rev() { //for each layer
            layer_index -= 1;
            let mut layer1: Vec<Neuron> = Vec::new();
            for index in 0..layer {//for each neuron, 
                layer1.push(Neuron::new(next, (layer_index, index)))
            }
            next = layer;
            neurons.push(layer1);
        }
        neurons.reverse();
        return Network {neurons: neurons, learning_rate: 0.001};
    }

    pub fn predict(&mut self, inputs: Vec<f32>) -> Option<Vec<f32>> {
        if inputs.len() != self.neurons[0].len() {
            return None;
        }
        for i in 0..inputs.len() {
            self.neurons[0][i].value = inputs[i];
        }
        for layer in 1..self.neurons.len() {
            for neuron in 0..self.neurons[layer].len() {
                self.activate((layer, neuron));
            }
        }
        let mut output: Vec<f32> = Vec::new();
        for i in 0..self.neurons[self.neurons.len() - 1].len() {
            output.push(self.neurons[self.neurons.len() - 1][i].value);
        }
        //might add activation function here!
        return Some(output);
    }

    fn activate(&mut self, place: (usize, usize)) {//used to calculate the value of a neuron!
        let mut sum: f32 = 0.0;
        for i in 0..self.neurons[place.0 - 1].len() { //index of each neuron in the layer before
            sum += self.neurons[place.0 - 1][i].value * self.neurons[place.0 - 1][i].weights[place.1]
        }
        //might add activation funciton here!
        self.neurons[place.0][place.1].value = sum + self.neurons[place.0][place.1].bias;
    }

    pub fn fit() {
        todo!()
    }

    pub fn eval() {
        todo!()
    }

    fn loss() {
        todo!()
    }

    fn error(input: (Vec<f32>, Vec<f32>)) -> f32 {
        todo!()
    }
}