use std::{env, process};

use s1eepeng_crates_oidc_replay_lab::{Operation, calculate};

fn usage() -> ! {
    eprintln!("usage: oidc-calc <number> <add|sub|mul|div> <number>");
    process::exit(2);
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.len() != 3 {
        usage();
    }

    let left = arguments[0].parse::<f64>().unwrap_or_else(|_| usage());
    let right = arguments[2].parse::<f64>().unwrap_or_else(|_| usage());
    let operation = match arguments[1].as_str() {
        "add" => Operation::Add,
        "sub" => Operation::Subtract,
        "mul" => Operation::Multiply,
        "div" => Operation::Divide,
        _ => usage(),
    };

    match calculate(left, operation, right) {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("error: {error}");
            process::exit(1);
        }
    }
}
