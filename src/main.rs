#![recursion_limit = "512"]

use anyhow::Result;
use diesel::prelude::*;
use std::io;
use tacks::establish_connection;

mod schema;
mod shot;

use schema::shots::dsl::*;

fn import_csv(connection: &mut PgConnection) -> Result<Vec<shot::NewShot>> {
    let mut csv_reader = csv::Reader::from_reader(io::stdin());
    let mut new_shots = vec![];

    for result in csv_reader.deserialize() {
        let new_shot: shot::NewShot = result?;
        new_shots.push(new_shot);
    }

    diesel::insert_into(shots)
        .values(&new_shots)
        .execute(connection)?;

    Ok(new_shots)
}

fn main() -> Result<()> {
    let connection = &mut establish_connection();

    let res = shots.limit(1).load::<shot::Shot>(connection)?;
    if res.len() < 1 {
        let _ = import_csv(connection)?;
    }

    let mut buffer = String::new();
    println!("Enter a shooter:");

    io::stdin().read_line(&mut buffer)?;

    let name: &str = buffer.trim();
    let res = shots
        .filter(shooter_name.eq(name))
        .load::<shot::Shot>(connection)?;

    for s in res {
        println!("{:#?}", s);
    }

    Ok(())
}
