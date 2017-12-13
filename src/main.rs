mod lib;

use lib::reversor;
use std::env;

fn main() {
    let args = env::args().skip(1);

    match args.len() {
        0 => {
            eprintln!("requires 1 arguments, received 0");
            std::process::exit(1)
        },
        _ => {
            args.for_each(|w| {
                print!("{} ", reversor(&w));
            })
        }
    }
}
