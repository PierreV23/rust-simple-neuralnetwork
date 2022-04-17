use core::panic;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::seq::index::sample;
use rand::Rng;
use std::f64::consts::E;
use std::iter;
use std::ops::Range;

pub type ActivationFunction = fn(f64) -> f64;

// Is of type ActivationFunction
pub fn sigmoid(n: f64) -> f64 {
    1. - 1. / (1. + E.powf(n))
}

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
                "Argument `amount_of_weights` has to be 0<n<{}",
                self.weights.len()
            )
        }
        let mut rng = rand::thread_rng();
        let chosen = sample(&mut rng, self.weights.len(), amount_of_weights);
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
    pub fn process(&self, input: Vec<f64>) -> Vec<f64> {
        self.neurons
            .iter()
            // TODO?:
            // undo the need to reference input,
            // this would mean input would have to get copied multiple times,
            // so problaby bad idea.
            .map(|neuron| neuron.process(&input))
            .collect::<Vec<f64>>()
    }
    pub fn get_mut_neurons(&mut self) -> &mut Vec<Neuron> {
        &mut self.neurons
    }
}

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
    pub fn process(&self, inputs: Vec<f64>) -> Vec<f64> {
        if self.total_inputs != inputs.len() {
            panic!("The amount of inputs is incorrect, this network was made to take {} inputs, yet you gave {}", self.total_inputs, inputs.len())
        }
        Vec::new()
    }
    pub fn get_mut_all_neurons(&mut self) -> Vec<&mut Neuron> {
        self.layers
            .iter_mut()
            .flat_map(|layer| layer.get_mut_neurons())
            .collect::<Vec<&mut Neuron>>()
    }
    pub fn evolve(
        &mut self,
        input: Vec<f64>,
        output: Vec<f64>,
        children_per_generation: usize,
        generations: usize,
    ) {
        ()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
