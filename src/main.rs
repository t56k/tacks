#![recursion_limit = "512"]

use anyhow::Result;
use diesel::{insert_into, prelude::*};
use std::io;
use tacks::establish_connection;

mod schema;
mod shot;

use schema::shots::dsl::*;

fn import_csv(conn: &mut PgConnection) -> Result<()> {
    let mut csv_reader = csv::Reader::from_path("data/shots_2022_1.csv")?;
    let mut new_shots: Vec<shot::NewShot> = Vec::new();

    for result in csv_reader.deserialize() {
        let new_shot: shot::NewShot = result?;
        new_shots.push(new_shot);
    }

    insert_into(shots).values(&new_shots).execute(conn)?;

    Ok(())
}

fn main() -> Result<()> {
    let conn = &mut establish_connection();

    let _ = import_csv(conn)?;

    let mut buffer = String::new();
    println!("Enter a shooter: ");

    io::stdin().read_line(&mut buffer)?;

    let name: &str = buffer.trim();
    let res = shots
        .filter(shooter_name.eq(name))
        .load::<shot::Shot>(conn)?;

    for s in res {
        println!("{:?}", s.shooter_name);
        println!("{:?}", s.time);
        println!("{:?}", s.game_id);
    }

    Ok(())
}
