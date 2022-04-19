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

fn linear(n: f64) -> f64 {
    n * 2.
}

fn main() {
    let mut network = NeuralNetwork::new(vec![20, 20, 20, 40], 2, Some(sigmoid));
    //network.layers.push(lib::NeuralLayer::new(1, 20, linear));
    for _ in 0..300 {
        for n in 0..13u8 {
            network = network.evolve(vec![n as f64, 2.], vec![n.pow(2).into()], 50); //  2. * n as f64
            let thing: f64 = network.process(&vec![n as f64, 2.]).iter().sum();
            println!("{}**2 = {}", n, thing);
        }
    }
}
