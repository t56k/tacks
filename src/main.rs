use diesel::prelude::*;
use std::{error::Error, io, process};

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

pub fn import_shots(conn: &mut PgConnection) -> Vec<models::Shot> {
    let shots = parse_csv();

    diesel::insert_into(shots::table)
        .values(&shots)
        .get_results(conn)
        .expect("Error importing shots")
}

fn main() {
    let connection = &mut establish_connection();
    let shots = import_shots(connection);

    println!("Imported (first): {} with id {}", shots[0], shots[0].id);
}
