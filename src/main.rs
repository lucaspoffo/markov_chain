use markov_chain::Chain;
use std::process;
use std::fs;

fn main() {
    let mut chain = Chain::new(2);
    
    let paths = fs::read_dir("./sample").unwrap();
    for path in paths {
        let filename = path.unwrap().path().display().to_string();
        if let Err(e) = chain.feed_file(filename) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
    //println!("{}", chain);
    //chain.print_frase_start();
    println!("Generated text:\n");
    for i in 0..101 {
        println!("{}) {}", i, chain.generate());
    } 
}
