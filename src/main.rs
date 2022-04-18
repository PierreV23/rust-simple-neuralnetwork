mod lib;
use lib::sigmoid;
use lib::NeuralNetwork;

fn main() {
    let mut network = NeuralNetwork::new(vec![20, 20, 20], 1, Some(sigmoid));
    for n in 2..15u8 {
        for _ in 0..(n*10) {
            network = network.evolve(vec![n as f64], vec![2.*n as f64], 500); // n.pow(2).into()
            let thing: f64 = network.process(&vec![n as f64]).iter().sum();
            println!("{}**2 = {}", n, thing);
        }
    }
}