use std::{io::stdin, net::TcpStream, thread::sleep, time::Duration};

mod ascii_codes;
use crate::ascii_codes::style::*;

fn main() {
    println!("{}--- Client ---{}", BOLD, BOLD_END);

    // asking user for address
    let mut addr = String::new();
    println!("{DIM}Connect to:{DIM_END}");
    match stdin().read_line(&mut addr) {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    };
    let addr = addr.trim();

    let stream = match TcpStream::connect(addr) {
        Ok(tcp_stream) => tcp_stream,
        Err(e) => {
            println!("{e}");
            println!("{BOLD}{BLINK}--- Shut Down ---{BLINK_END}{BOLD_END}");
            let _ = stdin().read_line(&mut String::new());
            return;
        }
    };
    let addr = stream.local_addr().unwrap();

    println!("{DIM}connected! address: {addr}{DIM_END}");

    //

    //

    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    };
    println!("{BOLD}{BLINK}--- Shut Down ---{BLINK_END}{BOLD_END}");
    let _ = stdin().read_line(&mut String::new());
}
