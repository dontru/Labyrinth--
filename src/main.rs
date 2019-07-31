use std::env;
use std::fs;
use labyrinthmm::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    match run(&contents) {
        Ok(bf) => println!("{}", bf),
        Err(e) => eprintln!("{}", e),
    }
}
