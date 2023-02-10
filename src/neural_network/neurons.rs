#![allow(dead_code)]
use rand::prelude::*;
use std::f32::{MIN, MAX};
#[derive(Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
    pub value: f32,
    pub place: (usize, usize),//layer, index in layer
    pub activation: Activation,
}

#[derive(Debug, Clone, Copy)]
pub enum Activation {
    Linear,
    Relu,
    Sigmoid,
    Tanh,
    Softmax,
    LeakyRelu,
}

impl Activation {
    pub fn as_fn(self) -> fn(f32) -> f32 {
        match self {
            Self::Linear => |x| x,
            Self::Relu => |x| x.max(0.0),
            Self::Sigmoid => todo!(),
            Self::Tanh => todo!(),
            Self::Softmax => todo!(),
            Self::LeakyRelu => todo!(),
        }
    }
    pub fn as_derivative(self) -> fn(f32) -> f32 {
        match self {
            Self::Linear => |_| 1.0,
            Self::Relu => |x| if x < 0.0 {0.0} else {1.0},
            Self::Sigmoid => todo!(),
            Self::Tanh => todo!(),
            Self::Softmax => todo!(),
            Self::LeakyRelu => todo!(),
        }
    }
}

impl Neuron {
    pub fn new(len: usize, place: (usize, usize), activation: Activation) -> Neuron {
        Neuron {
            bias: rand::thread_rng().gen(),
            weights: (0..len).map(|_| rand::thread_rng().gen()).collect(),
            value: 0.0,
            place: place,
            activation: activation,
        }
    }
    pub fn activate(&mut self) {
        self.value = self.activation.as_fn()(self.value)
    }
    pub fn derivative(&self) -> f32 {
        self.activation.as_derivative()(self.value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Loss {
    MSE,
    BCE,
    CCE,
}

impl Loss {
    pub fn as_fn(self) -> fn(Vec<f32>, Vec<f32>) -> f32 {
        match self {
            Self::MSE => todo!(),
            Self::BCE => todo!(),
            Self::CCE => todo!(),
        }
    }
    pub fn as_derivative(self) -> fn(f32, f32) -> f32 {
        match self {
            Self::MSE => todo!(),
            Self::BCE => todo!(),
            Self::CCE => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Network {
    pub neurons: Vec<Vec<Neuron>>,//vector of layers
    pub learning_rate: f32,
    pub loss: Loss,
    //ouput activation!
}

impl Network {
    pub fn new(size: Vec<usize>, lr: f32, activations: Vec<Activation>, loss: Loss) -> Network {
        let mut neurons: Vec<Vec<Neuron>> = Vec::new();
        let mut layer_index = size.len();
        let mut next = 0;
        for layer in size.into_iter().rev() { //for each layer
            layer_index -= 1;
            let mut layer1: Vec<Neuron> = Vec::new();
            for index in 0..layer {//for each neuron, 
                layer1.push(Neuron::new(next, (layer_index, index), activations[layer_index]))
            }
            next = layer;
            neurons.push(layer1);
        }
        neurons.reverse();
        return Network {neurons: neurons, learning_rate: lr, loss: loss};
    }

    pub fn predict(&mut self, inputs: Vec<f32>) -> Option<Vec<f32>> {
        if inputs.len() != self.neurons[0].len() {
            return None;
        }
        for i in 0..inputs.len() {
            self.neurons[0][i].value = inputs[i];
            self.neurons[0][i].activate()
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
        return Some(output);
    }

    fn activate(&mut self, place: (usize, usize)) {//used to calculate the value of a neuron!
        let mut sum: f32 = 0.0;
        for i in 0..self.neurons[place.0 - 1].len() { //index of each neuron in the layer before
            sum += self.neurons[place.0 - 1][i].value * self.neurons[place.0 - 1][i].weights[place.1]
        }
        self.neurons[place.0][place.1].value = sum + self.neurons[place.0][place.1].bias;
        self.neurons[place.0][place.1].activate();
    }

    pub fn fit(&mut self, inputs: Vec<Vec<f32>>, labels: Vec<Vec<f32>>) -> f32 { 
        if inputs.len() != labels.len() {
            return 0.0;
        }
    
        let mut avg_loss = 0.0;
        for i in 0..inputs.len() {
            let prediction = self.predict(inputs[i].clone()).unwrap();
            let mut changes = vec![0.0; prediction.len()];
            for j in 0..prediction.len() {
                changes[j] = labels[i][j] - prediction[j];
            }
            avg_loss += self.loss(labels[i].clone(), prediction);
    
            for layer in (1..self.neurons.len()).rev() {
                let mut new_changes = vec![0.0; self.neurons[layer - 1].len()];
                for neuron in 0..self.neurons[layer].len() {
                    for bf_neuron in 0..self.neurons[layer - 1].len() {
                        self.neurons[layer - 1][bf_neuron].weights[neuron] += self.learning_rate * self.neurons[layer - 1][bf_neuron].value * changes[neuron] * self.neurons[layer - 1][bf_neuron].derivative();
                        self.neurons[layer - 1][bf_neuron].bias -= self.learning_rate * changes[neuron] * self.neurons[layer - 1][bf_neuron].derivative();
                        new_changes[bf_neuron] += self.neurons[layer - 1][bf_neuron].weights[neuron] * changes[neuron];
                    }
                }
                changes = new_changes
            }
        }
        avg_loss / inputs.len() as f32
    }

    pub fn eval(inputs: Vec<Vec<f32>>, labels: Vec<Vec<f32>>) -> (f32, f32) { //returns accuracy and loss
        todo!()
    }

    pub fn loss(&self, label: Vec<f32>, prediction: Vec<f32>) -> f32 { //mse
        let mut diff: Vec<f32> = (0..label.len()).map(|i| label[i] - prediction[i]).collect();
        diff = (0..diff.len()).map(|i| diff[i].powf(2.0)).collect();
        return diff.iter().copied().sum::<f32>() / diff.len() as f32;
    }

    fn error(&self, label: Vec<f32>, prediction: Vec<f32>) -> Vec<f32> {
        return (0..label.len()).map(|i| label[i] - prediction[i]).collect();
    }

    fn cost(&self, label: Vec<f32>, prediction: Vec<f32>) -> f32 { //mse
        let mut diff: Vec<f32> = (0..label.len()).map(|i| label[i] - prediction[i]).collect();
        diff = (0..diff.len()).map(|i| diff[i].powf(2.0)).collect();
        return diff.iter().copied().sum::<f32>();
    }

    pub fn normalize(input: Vec<f32>) -> Vec<f32> {
        let mut min = MAX;
        let mut max = MIN;
        for i in input.iter().copied() {
            if i > max {
                max = i;
            }
            if i < min {
                min = i;
            }
        }
        let mut output: Vec<f32> = Vec::new();
        for i in input.iter().copied() {
            output.push((i - min)/(max - min));
        }
        return output;
    }

    pub fn save(path: &str) {
        todo!()
    }
    pub fn load(path: &str) -> Network {
        todo!()
    }
}