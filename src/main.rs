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
    let mut username = String::new();
    println!("{DIM}Enter your username:{DIM_END}");
    match stdin().read_line(&mut username) {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
            println!("{BOLD}{BLINK}--- Shut Down ---{BLINK_END}{BOLD_END}");
            let _ = stdin().read_line(&mut String::new());
            return;
        }
    };
    let username = username.trim();
    if username != "" {
        stream.write(format!("set_username\r{}\r\n", username).as_bytes());
    }

    loop {
        let mut message = String::new();
        match stdin().read_line(&mut message) {
            Ok(_) => {}
            Err(e) => {
                println!("{e}");
                println!("{BOLD}{BLINK}--- Shut Down ---{BLINK_END}{BOLD_END}");
                let _ = stdin().read_line(&mut String::new());
                return;
            }
        };
        let message = message.trim();
        stream.write(format!("send_msg\r{}\r\n", message).as_bytes());
    }

    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    };
    println!("{BOLD}{BLINK}--- Shut Down ---{BLINK_END}{BOLD_END}");
    let _ = stdin().read_line(&mut String::new());
}
