use core::panic;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::f64::consts::E;
use std::iter;

type ActivationFunction = fn(f64) -> f64;

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
    pub fn new(number_of_inputs: u32, activation_function: ActivationFunction) -> Self {
        if number_of_inputs < 1 {
            panic!("A neuron is required to have atleast one input.")
        }
        let mut rng = rand::thread_rng();
        Self {
            weights: (0..number_of_inputs)
                .map(|_| rng.gen_range(-1.0f64..1.0))
                .collect::<Vec<_>>(),
            bias: 1.,
            activation_function: activation_function,
        }
    }
    pub fn process(&self, input: &Vec<f64>) -> f64 {
        if self.weights.len() != input.len() {
            panic!("The amount of input values has to be the same as the amount of weights this neuron has.")
        }
        iter::zip(self.weights.iter(), input.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }
}

pub struct NeuralLayer {
    neurons: Vec<Neuron>,
}

impl NeuralLayer {
    pub fn new(
        number_of_neurons: u32,
        number_of_inputs: u32,
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
            // this would mean input would have to get multiple times,
            // so probraly bad idea.
            .map(|neuron| neuron.process(&input))
            .collect::<Vec<f64>>()
    }
}

struct NeuralNetwork {
    total_inputs: u32,
    layers: Vec<NeuralLayer>,
}

impl NeuralNetwork {
    pub fn new(
        layer_blueprint: Vec<u32>,
        total_inputs: u32,
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
