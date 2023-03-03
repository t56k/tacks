use std::{error::Error, io, process};

mod shot;

fn parse() -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_reader(io::stdin());
    let mut shots = Vec::new();

    for result in csv_reader.deserialize() {
        let shot: shot::Shot = result?;
        shots.push(shot);
    }

    Ok(())
}

fn main() {
    if let Err(err) = parse() {
        println!("error parsing: {}", err);
        process::exit(1);
    }
}
