use std::env;
use std::fs;
use std::process;

/// Solves
/// https://www.thinkfun.com/products/invasion-of-the-cow-snatchers/[Invasion of
/// the Cow Snatchers] puzzles.

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = match &args[..] {
        [_, file] => file,
        _ => {
            println!("Invalid arguments");
            process::exit(1);
        }
    };
    let src: String = match fs::read_to_string(file) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("Failed to read {} [{}]", file, err);
            process::exit(1);
        }
    };
    println!("{}", src);
    let farm = match src.parse() {
        Ok(farm) => farm,
        Err(_) => {
            println!("Failed to parse IotCS");
            process::exit(1);
        }
    };
    let iotcs = iotcs::IotCS::new(&farm);
    match puzzle::solve(iotcs) {
        None => println!("no solution"),
        Some((mvs, p)) => {
            println!("solution:");
            for m in &mvs {
                print!("{}", m);
            }
            println!();
            println!("{}", p.ufo_with_cattle_to_string());
        }
    }
}
