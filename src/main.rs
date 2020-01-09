use markov_chain::Chain;
use std::process;
use std::fs;

fn main() {
    let mut chain = Chain::new();
    
    let paths = fs::read_dir("./sample").unwrap();
    for path in paths {
        let filename = path.unwrap().path().display().to_string();
        if let Err(e) = chain.feed_file(filename) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
    //println!("{}", chain);
    println!("Generated text:\n {}", chain.generate());
}
