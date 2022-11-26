use serialport::*;
use std::env;

fn get_parameter() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        return args[1].clone();
    }
    "TEST".to_string()
}

fn main() {
    println!("{}", get_parameter());

    match available_ports() {
        Ok(ok_var) => println!("Ok {:?}", ok_var),
        _ => println!("Error"),
    }
}
