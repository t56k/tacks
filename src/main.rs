#![recursion_limit = "512"]

use diesel::prelude::*;
use std::{error::Error, io};
use tacks::establish_connection;

mod models;
mod schema;

fn parse_csv() -> Result<Vec<models::NewShot>, Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_reader(io::stdin());
    let mut shots = vec![];

    for result in csv_reader.deserialize() {
        let shot: models::NewShot = result?;
        shots.push(shot);
    }

    Ok(shots)
}

pub fn import_shots(conn: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let shots = parse_csv().expect("Can't get shots");
    diesel::insert_into(schema::shots::table).values(shots);

    Ok(())
}

fn main() {
    let connection = &mut establish_connection();
    let shots = import_shots(connection);

    println!("Imported");
}
