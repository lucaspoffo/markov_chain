use markov_chain::Chain;
use std::process;

fn main() {
    let mut chain = Chain::new(2);
    
    if let Err(e) = chain.feed_directory("./sample") {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
    println!("Generated text:");
    for i in 0..101 {
        println!("{}) {}", i, chain.generate());
    } 
}
