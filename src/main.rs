mod lib;
use lib::sigmoid;
use lib::NeuralNetwork;

fn step_function(n: f64) -> f64 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}

fn main() {
    let mut network = NeuralNetwork::new(vec![20, 50, 10], 1, Some(step_function));
    for _ in 0..50 {
        for n in 2..15u8 {
            network = network.evolve(vec![n as f64], vec![2. * n as f64], 500); // n.pow(2).into()
            let thing: f64 = network.process(&vec![n as f64]).iter().sum();
            println!("{}**2 = {}", n, thing);
        }
    }
}
