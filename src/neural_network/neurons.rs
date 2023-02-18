#![allow(dead_code)]
use core::f32;
use rand::prelude::*;
use std::f32::{consts::E, MAX, MIN};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
    pub value: f32,
    pub place: (usize, usize), //layer, index in layer
    pub activation: Activation,
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
    pub fn activate(&mut self, sum: Option<f32>) {
        if self.activation.to_string() == "Softmax".to_string() {
            self.value = self.activation.as_fn()(self.value) / sum.unwrap(); // sum is sum(e^every_value)
        } else {
            self.value = self.activation.as_fn()(self.value)
        }
    }
    pub fn derivative(&self) -> f32 {
        self.activation.as_derivative()(self.value)
    }
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
            Self::Sigmoid => |x| 1.0 / (1.0 + E.powf(-x)),
            Self::Tanh => |x| (E.powf(x) - E.powf(-x)) / (E.powf(x) + E.powf(x)),
            Self::Softmax => |x| E.powf(x),
            Self::LeakyRelu => |x| x.max(0.01 * x),
        }
    }
    pub fn as_derivative(self) -> fn(f32) -> f32 {
        match self {
            Self::Linear => |_| 1.0,
            Self::Relu => |x| if x < 0.0 { 0.0 } else { 1.0 },
            Self::Sigmoid => |x| x * (1.0 - x),
            Self::Tanh => |x| 1.0 - x * x,
            Self::Softmax => |x| x * (1.0 - x), //might be wrong
            Self::LeakyRelu => |x| if x < 0.0 { 0.01 } else { 1.0 },
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

#[derive(Debug, Clone, Copy)]
pub enum Loss {
    MSE,
    BCE,
    CCE,
}

impl Loss {
    pub fn as_fn(self) -> fn(Vec<f32>, Vec<f32>) -> f32 {
        match self {
            Self::MSE => |label, prediction| {
                (0..label.len())
                    .map(|i| (label[i] - prediction[i]).powf(2.0))
                    .sum::<f32>()
                    / label.len() as f32
            },
            Self::BCE => todo!(),
            Self::CCE => todo!(),
        }
    }
    pub fn as_derivative(self) -> fn(f32, f32) -> f32 {
        match self {
            Self::MSE => |prediction, label| 2.0 * (prediction - label),
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

    pub fn best_lr(&mut self, inputs: Vec<Vec<f32>>, labels: Vec<Vec<f32>>) {
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
        self.learning_rate = 1.0;
        self.fit(inputs.clone(), labels.clone());
        while self.any_nan() {
            self.learning_rate /= 10.0;
            // println!("testing learning rate: {}", self.learning_rate);
            for layer in 0..self.neurons.len() {
                for neuron in 0..self.neurons[layer].len() {
                    self.neurons[layer][neuron].weights = weights[layer][neuron].clone();
                }
            }
            for layer in 0..self.neurons.len() {
                for neuron in 0..self.neurons[layer].len() {
                    self.neurons[layer][neuron].bias = biases[layer][neuron];
                }
            }
            self.fit(inputs.clone(), labels.clone());
        }
    }

    fn any_nan(&self) -> bool {
        for layer in 0..self.neurons.len() {
            for neuron in 0..self.neurons[layer].len() {
                if self.neurons[layer][neuron].bias.is_nan() {
                    return true;
                }
                for weight in 0..self.neurons[layer][neuron].weights.len() {
                    if self.neurons[layer][neuron].weights[weight].is_nan() {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn predict(&mut self, inputs: Vec<f32>) -> Option<Vec<f32>> {
        if inputs.len() != self.neurons[0].len() {
            return None;
        }
        for i in 0..inputs.len() {
            self.neurons[0][i].value = inputs[i];
            if self.neurons[0][i].activation.to_string() == "Softmax".to_string() {
                let sum1 = self.softmax_sum(0);
                self.neurons[0][i].activate(Some(sum1))
            } else {
                self.neurons[0][i].activate(None)
            }
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
        if self.neurons[place.0][place.1].activation.to_string() == "Softmax".to_string() {
            let sum1 = self.softmax_sum(place.0);
            self.neurons[place.0][place.1].activate(Some(sum1))
        } else {
            self.neurons[place.0][place.1].activate(None)
        }
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
                changes[j] = self.loss.as_derivative()(prediction[j], labels[i][j]);
            }
            avg_loss += self.loss(labels[i].clone(), prediction);

            for layer in (1..self.neurons.len()).rev() {
                let mut new_changes = vec![0.0; self.neurons[layer - 1].len()];
                for neuron in 0..self.neurons[layer].len() {
                    for bf_neuron in 0..self.neurons[layer - 1].len() {
                        self.neurons[layer - 1][bf_neuron].weights[neuron] -= self.learning_rate
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

    pub fn eval(&mut self, inputs: Vec<Vec<f32>>, labels: Vec<Vec<f32>>) -> (f32, Option<f32>) {
        let predictions = (0..inputs.len())
            .map(|i| self.predict(inputs[i].clone()).unwrap())
            .collect::<Vec<Vec<f32>>>();
        let avg_loss = (0..predictions.len())
            .map(|i| self.loss(labels[i].clone(), predictions[i].clone()))
            .sum::<f32>()
            / predictions.len() as f32;
        if self.loss.to_string() == "MSE".to_string() {
            return (avg_loss, None);
        }
        todo!("implement finding accuracy for classifications");
    }

    pub fn loss(&self, label: Vec<f32>, prediction: Vec<f32>) -> f32 {
        self.loss.as_fn()(label, prediction)
    }

    fn error(&self, label: Vec<f32>, prediction: Vec<f32>) -> Vec<f32> {
        return (0..label.len()).map(|i| label[i] - prediction[i]).collect();
    }

    pub fn summary(&self) {
        let mut params = (0..self.neurons.len() - 1)
            .map(|layer| self.neurons[layer].len() * (self.neurons[layer + 1].len() + 1))
            .collect::<Vec<usize>>();
        params.push(self.neurons[self.neurons.len() - 1].len());
        let line = 60;
        println!("model summary:");
        (0..line).for_each(|_| print!("-"));
        let placings = vec![18, 36, 54];
        println!();
        self.print_layer("layer", "neurons#", "activation", "param#", &placings);
        println!();
        (0..line).for_each(|_| print!("="));
        println!();
        self.print_layer(
            "input",
            self.neurons[0].len().to_string().as_str(),
            self.neurons[0][0].activation.to_string().as_str(),
            params[0].to_string().as_str(),
            &placings,
        );
        for layer in 1..self.neurons.len() - 1 {
            println!("\n");
            self.print_layer(
                &format!("hidden {}", layer),
                self.neurons[layer].len().to_string().as_str(),
                self.neurons[layer][0].activation.to_string().as_str(),
                params[layer].to_string().as_str(),
                &placings,
            );
        }
        println!("\n");
        self.print_layer(
            "output",
            self.neurons[self.neurons.len() - 1]
                .len()
                .to_string()
                .as_str(),
            self.neurons[self.neurons.len() - 1][0]
                .activation
                .to_string()
                .as_str(),
            params[self.neurons.len() - 1].to_string().as_str(),
            &placings,
        );
        println!();
        (0..line).for_each(|_| print!("="));
        println!();
        println!("total params: {}", params.into_iter().sum::<usize>());
        (0..line).for_each(|_| print!("-"));
        println!();
    }

    fn print_layer(&self, layer: &str, neurons: &str, act: &str, param: &str, pl: &Vec<usize>) {
        let mut place = 0;
        print!("{}", layer);
        place += layer.len();
        (0..pl[0] - place).for_each(|_| print!(" "));
        print!("{}", neurons);
        place = pl[0] + neurons.len();
        (0..pl[1] - place).for_each(|_| print!(" "));
        print!("{}", act);
        place = pl[1] + act.len();
        (0..pl[2] - place).for_each(|_| print!(" "));
        print!("{}", param);
    }

    fn softmax_sum(&self, layer: usize) -> f32 {
        let mut sum = 0.0;
        for neuron in &self.neurons[layer] {
            sum += E.powf(neuron.value);
        }
        sum
    }

    pub fn test_train_split(
        per: f32,
        inputs: Vec<Vec<f32>>,
        labels: Vec<Vec<f32>>,
    ) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        todo!()
    }

    pub fn read_csv(path: &str, label_index: usize) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        if !Path::new(path).exists() {
            panic!("path doesnt exist");
        }
        let mut reader = csv::Reader::from_path(path).unwrap();
        let mut inputs: Vec<Vec<f32>> = Vec::new();
        let mut labels: Vec<Vec<f32>> = Vec::new();
        for result in reader.records() {
            let mut vec: Vec<f32> = result
                .unwrap()
                .into_iter()
                .map(|i| i.parse::<f32>().unwrap())
                .collect();
            labels.push(vec![vec.remove(label_index)]);
            inputs.push(vec);
        }
        (inputs, labels)
    }

    pub fn class_to_label(classes: Vec<Vec<f32>>, classes_num: usize) -> Vec<Vec<f32>> {
        let mut labels: Vec<Vec<f32>> = Vec::new();
        for label in 0..classes.len() {
            let mut vec: Vec<f32> = vec![0.0; classes_num];
            vec[classes[label][0] as usize] = 1.0;
            labels.push(vec);
        }
        labels
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
        let data = fs::read_to_string(path).unwrap();
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
