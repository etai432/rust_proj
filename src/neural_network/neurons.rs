use rand::{prelude::*};
use std::f32::{MIN, MAX};
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
}

#[derive(Debug)]
pub struct Network {
    pub neurons: Vec<Vec<Neuron>>,//vector of layers
    pub learning_rate: f32,
    //ouput activation!
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
        return Network {neurons: neurons, learning_rate: 0.0001};
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

    pub fn fit(&mut self, inputs: Vec<Vec<f32>>, labels: Vec<Vec<f32>>) -> f32 { //returns loss, uses back propagation
        if inputs.len() != labels.len() {
            return 0.0;
        }
        let mut changes: Vec<f32> = (0..labels[0].len()).map(|_| 0.0).collect(); //the changes we want the make for the last neurons
        let predicts = inputs.into_iter().map(|input| self.predict(input).unwrap()).collect::<Vec<Vec<f32>>>();
        for i in 0..predicts.len() {//per prediction
            for j in 0..predicts[0].len() {//per value in prediction
                changes[j] += labels[i][j] - predicts[i][j];
            }
        }
        for layer in (1..self.neurons.len()).rev() {
            for neuron in 0..self.neurons[layer].len() { //updating the weights
                for bf_neuron in 0..self.neurons[layer - 1].len() {
                    self.neurons[layer-1][bf_neuron].weights[neuron] -= self.learning_rate * changes[neuron] * self.neurons[layer][neuron].value;
                    self.neurons[layer-1][bf_neuron].bias -= self.learning_rate * changes[neuron];
                }
            }
            let mut new_changes: Vec<f32> = (0..self.neurons[layer - 1].len()).map(|_| 0.0).collect();
            println!("{:?}", changes);
            for bf_neuron in 0..self.neurons[layer-1].len() {//for each wanted change neuron
                for neuron in 0..self.neurons[layer].len() {//for each neuron in current layer
                    new_changes[bf_neuron] += self.neurons[layer-1][bf_neuron].weights[neuron] * changes[neuron];
                }
            }
            changes = new_changes;
        }
        (0..predicts.len()).map(|i| self.loss(labels[i].clone(), predicts[i].clone())).sum::<f32>() / (predicts.len() as f32)
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
}