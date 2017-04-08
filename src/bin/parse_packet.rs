extern crate nom;
extern crate semtech_lorawan_parser;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use nom::IResult;
use semtech_lorawan_parser::parse_packet;
use semtech_lorawan_parser::{Packet};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }
    let filename = args[1].clone();
    println!("Parsing packet from file \"{}\"...", &filename);

    let mut file = File::open(filename).unwrap_or_else(|e| {
        println!("Could not open file: {}", e);
        process::exit(2);
    });
    let mut packet = Vec::new();
    match file.read_to_end(&mut packet) {
        Ok(count) => println!("Read {} bytes...", count),
        Err(e) => {
            println!("Could not read file: {}", e);
            process::exit(2);
        },
    };

    let parsed = match parse_packet(&packet) {
        IResult::Done(i, o) => {
            println!("\nDone: {:?}, Remaining: {:?}\n", o, i);
            o
        }
        IResult::Error(e) => {
            println!("\nError: {:?}", e);
            process::exit(3);
        },
        IResult::Incomplete(n) => {
            println!("\nNeeded more input: {:?}", n);
            process::exit(3);
        },
    };

    println!("Packet info:\n------------");
    match parsed {
        Packet::PushData(p) => {
            println!("Protocol version: {}", p.version);
            println!("Packet type: PUSH_DATA");
            println!("Random token: {:?}", p.random_token);
            print!("Gateway UID: ");
            let mut first = true;
            for &byte in p.gateway_uid {
                if first {
                    first = false;
                } else {
                    print!(":");
                }
                print!("{:X}", byte);
            };
            println!();
        },
        Packet::PushAck(p) => {
            println!("Protocol version: {}", p.version);
            println!("Packet type: PUSH_ACK");
            println!("Random token: {:?}", p.random_token);
        },
    }

}
