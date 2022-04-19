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
    n
}

fn main() {
    let mut network = NeuralNetwork::new(vec![40*5, 40*5, 40*5, 40*5], 2, Some(step_function));
    network.layers.push(lib::NeuralLayer::new(1, 40*5, linear));
    let v: [u32; 9] = [6, 3, 1, 8, 5, 0, 7, 2, 4];
    for m in 0..5000 {
        println!("Evolving... {}", m);
        for &n in v.iter() {
            network = network.evolve(vec![n as f64, 2.], vec![n.pow(2).into()], 20); //  2. * n as f64
            let thing: f64 = network.process(&vec![n as f64, 2.]).iter().sum();
            //println!("{}**2 = {}", n, thing);
        }
        println!("Tests...");
        for n in 0..v.len() {
            println!("{}**2 = {}", n, network.process(&vec![n as f64, 2.]).iter().sum::<f64>());
        }
    }
    
    for n in 0..5 {
        println!("{}**3 = {}", n, network.process(&vec![n as f64, 3.]).iter().sum::<f64>());
    }
}
