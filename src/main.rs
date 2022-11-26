use serialport::*;
use std::env;

fn get_parameter() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        return args[1].clone();
    }
    "TEST".to_string()
}

fn list_ports() -> Option<String> {
    println!("Available ports:");
    if let Ok(sis) = available_ports() {
        for si in &sis {
            println!(" {}", si.port_name)
        }
        if sis.len() == 1 {
            return Some(sis[0].port_name.clone());
        }
    }
    None
}

fn main() {
    let string_to_send = get_parameter();
    println!("String to send: {}", string_to_send);
    if let Some(port) = list_ports() {
        if let Ok(mut open_port) = serialport::new(&port, 57600).open() {
            println!("Opened port {}", open_port.name().unwrap());
            open_port
                .set_parity(Parity::None)
                .expect("Set parity failed");
            open_port
                .set_data_bits(DataBits::Eight)
                .expect("Set data bits failed");
            open_port
                .write(string_to_send.as_bytes())
                .expect("Write failed");
        } else {
            println!("Can't open port {}", port)
        }
    }
}
