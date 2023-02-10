#![allow(dead_code)]
use rand::prelude::*;
use std::f32::{MAX, MIN};
use std::fmt::format;
use std::fs;
use std::io::prelude::*;
#[derive(Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
    pub value: f32,
    pub place: (usize, usize), //layer, index in layer
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
            Self::Relu => |x| if x < 0.0 { 0.0 } else { 1.0 },
            Self::Sigmoid => todo!(),
            Self::Tanh => todo!(),
            Self::Softmax => todo!(),
            Self::LeakyRelu => todo!(),
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "Linear" => Self::Linear,
            "Relu" => Self::Relu,
            "Sigmoid" => Self::Sigmoid,
            "Tanh" => Self::Tanh,
            "Softmax" => Self::Softmax,
            "LeakyRelu" => Self::LeakyRelu,
            &_ => Self::Linear,
        }
    }
    pub fn to_string(self) -> String {
        match self {
            Self::Linear => "Linear".to_string(),
            Self::Relu => "Relu".to_string(),
            Self::Sigmoid => "Sigmoid".to_string(),
            Self::Tanh => "Tanh".to_string(),
            Self::Softmax => "Softmax".to_string(),
            Self::LeakyRelu => "LeakyRelu".to_string(),
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
    pub fn from_str(s: &str) -> Self {
        match s {
            "MSE" => Self::MSE,
            "BCE" => Self::BCE,
            "CCE" => Self::CCE,
            &_ => Self::MSE,
        }
    }
    pub fn to_string(self) -> String {
        match self {
            Self::MSE => "MSE".to_string(),
            Self::BCE => "BCE".to_string(),
            Self::CCE => "CCE".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Network {
    pub neurons: Vec<Vec<Neuron>>, //vector of layers
    pub learning_rate: f32,
    pub loss: Loss,
    //ouput activation!
}

impl Network {
    pub fn new(size: Vec<usize>, lr: f32, activations: Vec<Activation>, loss: Loss) -> Network {
        let mut neurons: Vec<Vec<Neuron>> = Vec::new();
        let mut layer_index = size.len();
        let mut next = 0;
        for layer in size.into_iter().rev() {
            //for each layer
            layer_index -= 1;
            let mut layer1: Vec<Neuron> = Vec::new();
            for index in 0..layer {
                //for each neuron,
                layer1.push(Neuron::new(
                    next,
                    (layer_index, index),
                    activations[layer_index],
                ))
            }
            next = layer;
            neurons.push(layer1);
        }
        neurons.reverse();
        return Network {
            neurons: neurons,
            learning_rate: lr,
            loss: loss,
        };
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

    fn activate(&mut self, place: (usize, usize)) {
        //used to calculate the value of a neuron!
        let mut sum: f32 = 0.0;
        for i in 0..self.neurons[place.0 - 1].len() {
            //index of each neuron in the layer before
            sum +=
                self.neurons[place.0 - 1][i].value * self.neurons[place.0 - 1][i].weights[place.1]
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
                        self.neurons[layer - 1][bf_neuron].weights[neuron] += self.learning_rate
                            * self.neurons[layer - 1][bf_neuron].value
                            * changes[neuron]
                            * self.neurons[layer - 1][bf_neuron].derivative();
                        self.neurons[layer - 1][bf_neuron].bias -= self.learning_rate
                            * changes[neuron]
                            * self.neurons[layer - 1][bf_neuron].derivative();
                        new_changes[bf_neuron] +=
                            self.neurons[layer - 1][bf_neuron].weights[neuron] * changes[neuron];
                    }
                }
                changes = new_changes
            }
        }
        avg_loss / inputs.len() as f32
    }

    pub fn eval(inputs: Vec<Vec<f32>>, labels: Vec<Vec<f32>>) -> (f32, f32) {
        //returns accuracy and loss
        todo!()
    }

    pub fn loss(&self, label: Vec<f32>, prediction: Vec<f32>) -> f32 {
        //mse
        let mut diff: Vec<f32> = (0..label.len()).map(|i| label[i] - prediction[i]).collect();
        diff = (0..diff.len()).map(|i| diff[i].powf(2.0)).collect();
        return diff.iter().copied().sum::<f32>() / diff.len() as f32;
    }

    fn error(&self, label: Vec<f32>, prediction: Vec<f32>) -> Vec<f32> {
        return (0..label.len()).map(|i| label[i] - prediction[i]).collect();
    }

    fn cost(&self, label: Vec<f32>, prediction: Vec<f32>) -> f32 {
        //mse
        let mut diff: Vec<f32> = (0..label.len()).map(|i| label[i] - prediction[i]).collect();
        diff = (0..diff.len()).map(|i| diff[i].powf(2.0)).collect();
        return diff.iter().copied().sum::<f32>();
    }

    pub fn summery() -> String {
        todo!()
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
            output.push((i - min) / (max - min));
        }
        return output;
    }

    pub fn save(&self, path: &str) {
        let weights = self
            .neurons
            .iter()
            .map(|vec| {
                vec.iter()
                    .map(|neuron| neuron.weights.clone())
                    .collect::<Vec<Vec<f32>>>()
            })
            .collect::<Vec<Vec<Vec<f32>>>>();
        let biases = self
            .neurons
            .iter()
            .map(|vec| vec.iter().map(|neuron| neuron.bias).collect::<Vec<f32>>())
            .collect::<Vec<Vec<f32>>>();
        let structure = (0..self.neurons.len())
            .map(|i| self.neurons[i].len())
            .collect::<Vec<usize>>();
        let activations = (0..self.neurons.len())
            .map(|i| self.neurons[i][0].activation)
            .collect::<Vec<Activation>>();
        let loss = self.loss;
        let lr = self.learning_rate;
        let mut file = fs::File::create(path).unwrap();
        file.write_all(format!("{:?}\n", structure).as_bytes())
            .unwrap();
        file.write_all(
            format!(
                "{:?}\n",
                activations
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
            )
            .as_bytes(),
        )
        .unwrap();
        file.write_all(format!("{:?}\n", loss.to_string()).as_bytes())
            .unwrap();
        file.write_all(format!("{}\n", lr).as_bytes()).unwrap();
        file.write_all(format!("{:?}\n", weights).as_bytes())
            .unwrap();
        file.write_all(format!("{:?}", biases).as_bytes()).unwrap();
    }

    pub fn load(path: &str) -> Network {
        let mut data = fs::read_to_string(path).unwrap();
        let mut data = data.split("\n").collect::<Vec<&str>>();
        let structure = serde_json::from_str::<Vec<usize>>(data.remove(0)).unwrap();
        let activations = serde_json::from_str::<Vec<&str>>(data.remove(0))
            .unwrap()
            .into_iter()
            .map(|s| Activation::from_str(s))
            .collect::<Vec<Activation>>();
        let loss = Loss::from_str(data.remove(0));
        let lr = serde_json::from_str::<f32>(data.remove(0)).unwrap();
        let mut n = Network::new(structure, lr, activations, loss);
        let weights = serde_json::from_str::<Vec<Vec<Vec<f32>>>>(data.remove(0)).unwrap();
        let biases = serde_json::from_str::<Vec<Vec<f32>>>(data.remove(0)).unwrap();
        for layer in 0..n.neurons.len() {
            for neuron in 0..n.neurons[layer].len() {
                n.neurons[layer][neuron].weights = weights[layer][neuron].clone();
            }
        }
        for layer in 0..n.neurons.len() {
            for neuron in 0..n.neurons[layer].len() {
                n.neurons[layer][neuron].bias = biases[layer][neuron];
            }
        }
        n
    }
}
