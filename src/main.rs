mod lib;
use lib::sigmoid;
use lib::NeuralNetwork;
use std::iter;
use std;

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

fn difference(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    iter::zip(
        a.iter(),
        b.iter(),
    ).map(|(&a, &b)| (a-b).abs()).sum()

}

fn main() {
    let u8_to_byte = |n: u8| [7, 6, 5, 4, 3, 2, 1, 0].map(|i| ((n >> i) & 1) as f64);
    let mut network = NeuralNetwork::new(vec![40*5, 40*5, 40*5, 40*5], 2, Some(step_function));
    network.layers.push(lib::NeuralLayer::new(1, 40*5, linear));
    let v: [u32; 9] = [6, 3, 1, 8, 5, 0, 7, 2, 4];
    for m in 0..5000 {
        println!("Evolving... {}", m);
        for &n in v.iter() {
            network = network.evolve(vec![n as f64, 2.], vec![n.pow(2).into()], 20); //  2. * n as f64
            //let thing: f64 = network.process(&vec![n as f64, 2.]).iter().sum();
            //println!("{}**2 = {}", n, thing);
        }
        /*
        println!("Tests...");
        for n in 0..v.len() {
            println!("{}**2 = {}", n, network.process(&vec![n as f64, 2.]).iter().sum::<f64>());
        }
        */

        println!("Tests...");
        let mut outputs = Vec::new();
        for n in 0..v.len().try_into().unwrap() {
            //let input = u8_to_byte(n).to_vec();
            //let output = network.process(&input);
            let output = network.process(&vec![n as f64, 2.]);
            println!("{}**2 = {}; len {}", n, output.last().unwrap(), output.len());
            outputs.push(output);
        }
        let diff = difference(&outputs.iter().map(|v| *v.last().unwrap()).collect::<Vec<f64>>(), &v.map(|i| i as f64).to_vec());
        println!("Diff {}", diff);
        if  diff < 40. {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).expect("error: unable to read user input");
        }
    }
    
    for n in 0..5 {
        println!("{}**3 = {}", n, network.process(&vec![n as f64, 3.]).iter().sum::<f64>());
    }
}
