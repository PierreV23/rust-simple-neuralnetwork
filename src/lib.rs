#![allow(unused_variables, unused_imports)]

use core::panic;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::seq::index;
use rand::Rng;
use std::f64::consts::E;
use std::fmt;
use std::iter;
use std::ops::IndexMut;
use std::ops::Range;

pub type ActivationFunction = fn(f64) -> f64;

// Is of type ActivationFunction
pub fn sigmoid(n: f64) -> f64 {
    1. - 1. / (1. + E.powf(n))
}

#[derive(Clone)]
pub struct Neuron {
    weights: Vec<f64>,
    bias: f64,
    activation_function: ActivationFunction,
}

impl Neuron {
    pub fn new(number_of_inputs: usize, activation_function: ActivationFunction) -> Self {
        if number_of_inputs < 1 {
            panic!("A neuron is required to have atleast one input.")
        }
        let mut rng = rand::thread_rng();
        Self {
            weights: (0..number_of_inputs)
                .map(|_| rng.gen_range(-1.0f64..1.0))
                .collect::<Vec<_>>(),
            bias: 1.,
            activation_function,
        }
    }
    pub fn process(&self, input: &Vec<f64>) -> f64 {
        if self.weights.len() != input.len() {
            panic!("The amount of input values has to be the same as the amount of weights this neuron has.")
        }
        (self.activation_function)(
            iter::zip(self.weights.iter(), input.iter())
                .map(|(&a, &b)| a * b)
                .chain(std::iter::once(self.bias))
                .sum(),
        )
    }
    pub fn randomize_weights(&mut self, amount_of_weights: usize, range: Range<f64>) {
        if amount_of_weights < 1 || amount_of_weights > self.weights.len() {
            panic!(
                "Argument `amount_of_weights` has to be 0<n<{}, yours was {}",
                self.weights.len(),
                amount_of_weights,
            )
        }
        let mut rng = rand::thread_rng();
        let chosen = index::sample(&mut rng, self.weights.len(), amount_of_weights);
        for idx in chosen.iter() {
            let weight = self.weights.get_mut(idx).expect("How did you even get this to happen, smh. Anyway rng somehow generated an out of range index. This will probably never happen, why are you reading the code, going to commit plagerism????");
            *weight += rng.gen_range(range.clone());
        }
    }
    pub fn randomize_bias(&mut self, range: Range<f64>) {
        let mut rng = rand::thread_rng();
        self.bias += rng.gen_range(range);
    }
}

#[derive(Clone)]
pub struct NeuralLayer {
    neurons: Vec<Neuron>,
}

impl NeuralLayer {
    pub fn new(
        number_of_neurons: usize,
        number_of_inputs: usize,
        activation_function: ActivationFunction,
    ) -> Self {
        Self {
            neurons: (0..number_of_neurons)
                .map(|_| Neuron::new(number_of_inputs, activation_function))
                .collect::<Vec<_>>(),
        }
    }
    pub fn process(&self, input: &Vec<f64>) -> Vec<f64> {
        self.neurons
            .iter()
            .map(|neuron| neuron.process(input))
            .collect::<Vec<f64>>()
    }
    pub fn get_mut_neurons(&mut self) -> &mut Vec<Neuron> {
        &mut self.neurons
    }
}

#[derive(Clone)]
pub struct NeuralNetwork {
    total_inputs: usize,
    layers: Vec<NeuralLayer>,
}

impl NeuralNetwork {
    pub fn new(
        layer_blueprint: Vec<usize>,
        total_inputs: usize,
        activation_function: Option<ActivationFunction>,
    ) -> Self {
        if layer_blueprint.len() < 1 {
            panic!("Atleast one layer is required.")
        }
        if layer_blueprint.iter().any(|&x| x < 1) {
            panic!("Every layer is required to have atleast one neuron.")
        }
        if total_inputs < 1 {
            panic!("The neural network is required to have atleast one input.")
        }

        let activation_function = activation_function.unwrap_or(sigmoid);

        let mut layers = Vec::<NeuralLayer>::new();

        let mut number_of_inputs = total_inputs;
        for &number_of_neurons in layer_blueprint.iter() {
            layers.push(NeuralLayer::new(
                number_of_neurons,
                number_of_inputs,
                activation_function,
            ));
            number_of_inputs = number_of_neurons;
        }

        Self {
            total_inputs,
            layers,
        }
    }
    pub fn process(&self, input: &Vec<f64>) -> Vec<f64> {
        if self.total_inputs != input.len() {
            panic!("The amount of inputs is incorrect, this network was made to take {} inputs, yet you gave {}", self.total_inputs, input.len())
        }
        let mut output;
        let mut input = input.clone();
        for layer in self.layers.iter() {
            output = layer.process(&input);
            input = output;
        }
        input
    }
    pub fn get_mut_neurons(&mut self) -> Vec<&mut Neuron> {
        self.layers
            .iter_mut()
            .flat_map(|layer| layer.get_mut_neurons())
            .collect::<Vec<&mut Neuron>>()
    }
    pub fn evolve(self, input: Vec<f64>, output: Vec<f64>, amount_of_children: usize) -> Self {
        let change_percentage = 25. / 100.;
        let total_clones = 10;
        fn offset(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
            iter::zip(a, b).map(|(&a, &b)| (a - b).abs()).sum::<f64>()
        }

        let mut rng = rand::thread_rng();

        let mut prev_network = self.clone();
        let mut prev_offset = offset(&prev_network.process(&input), &output);

        for _ in 0..amount_of_children {
            let mut curr_network = self.clone();

            let mut neurons = curr_network.get_mut_neurons();
            let total_neurons = neurons.len();

            let sample = index::sample(
                &mut rng,
                total_neurons,
                (total_neurons as f64 * change_percentage).ceil() as usize,
            );

            for idx in sample {
                let neuron = neurons.get_mut(idx).unwrap();
                neuron.randomize_weights(
                    (neuron.weights.len() as f64 / 4.).ceil() as usize,
                    -0.75..0.75,
                );
                neuron.randomize_bias(-0.75..0.75);
            }

            let curr_offset = offset(&curr_network.process(&input), &output);

            //println!("PREV {}, CURR {}", &prev_offset, &curr_offset);
            if curr_offset < prev_offset {
                prev_network = curr_network;
                prev_offset = curr_offset;
            }
        }

        prev_network
    }
}

impl fmt::Display for NeuralNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for layer in self.layers.iter() {
            //string.push_str(&layer.neurons.len().to_string());
            string.push_str(&layer.neurons.first().unwrap().weights.len().to_string());
            string.push(' ')
        }
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use crate::NeuralNetwork;

    #[test]
    fn it_works() {
        use crate::sigmoid;
        let mut network = NeuralNetwork::new(vec![2, 4, 2], 5, Some(sigmoid));
        println!("TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-TEST-");
        assert_eq!(format!("{}", network), "5 2 4 ")
    }
}
