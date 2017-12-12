mod lib;

use lib::reversor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("requires 1 arguments, received {}", args.len());
        std::process::exit(1)
    }

    for w in args.into_iter() {
        println!("{}", reversor(&w));
    }
}
