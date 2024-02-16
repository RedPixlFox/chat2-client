#![allow(unused)]

use std::{
    io::{stdin, BufRead, BufReader, Read, Write},
    net::TcpStream,
    thread::sleep,
    time::Duration,
};

mod ascii_codes;
use crate::ascii_codes::style::*;

fn main() {
    println!("{}--- Client ---{}", BOLD, BOLD_END);

    // asking user for address
    let mut addr = String::new();
    println!("{DIM}Connect to:{DIM_END}");
    match stdin().read_line(&mut addr) {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
            println!("{BOLD}{BLINK}--- Shut Down ---{BLINK_END}{BOLD_END}");
            let _ = stdin().read_line(&mut String::new());
            return;
        }
    };
    let addr = addr.trim();

    let mut stream = match TcpStream::connect(addr) {
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

    // logic
    stream.write(b"Message 1, chunk 1\r");
    sleep(Duration::from_secs(1));
    stream.write(b"Message 1, chunk 2\r\n");
    sleep(Duration::from_secs(1));
    stream.write(b"Message 2, chunk 1\r");
    sleep(Duration::from_secs(1));
    stream.write(b"Message 2, chunk 2\r");
    sleep(Duration::from_secs(1));
    stream.write(b"Message 2, chunk 3\r\n");
    sleep(Duration::from_secs(1));
    //

    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    };
    println!("{BOLD}{BLINK}--- Shut Down ---{BLINK_END}{BOLD_END}");
    let _ = stdin().read_line(&mut String::new());
}
