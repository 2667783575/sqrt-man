use sqrt_man::calc;
use sqrt_man::Config;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);
    let answer = calc(config);
    println!("{answer}");
}
