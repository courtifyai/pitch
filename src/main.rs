use pitch::{Options, Pitch};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let config_path = args[1].clone();
    let pitch = Pitch::new(Options {
        config: config_path,
    });

    pitch.shift();
    pitch.patch();
}
