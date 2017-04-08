extern crate nom;
extern crate semtech_lorawan_parser;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use nom::IResult;
use semtech_lorawan_parser::parse_packet;

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

    match parse_packet(&packet) {
        IResult::Done(i, o) => println!("Done: {:?}, Remaining: {:?}", o, i),
        IResult::Error(e) => println!("Error: {:?}", e),
        IResult::Incomplete(n) => println!("Needed more input: {:?}", n),
    };
}
