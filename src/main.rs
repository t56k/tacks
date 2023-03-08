#![recursion_limit = "512"]

use anyhow::Result;
use diesel::{insert_into, prelude::*};
use std::io;
use tacks::establish_connection;

mod schema;
mod shot;

fn import_csv(conn: &mut PgConnection) -> Result<usize> {
    use schema::shots::dsl::*;

    let mut csv_reader = csv::Reader::from_path("data/shots_2022_1.csv")?;
    let mut new_shots: Vec<shot::NewShot> = Vec::new();

    for result in csv_reader.deserialize() {
        let new_shot: shot::NewShot = result?;
        new_shots.push(new_shot);
    }

    Ok(insert_into(shots).values(&new_shots).execute(conn)?)
}

fn main() -> Result<()> {
    use schema::shots::dsl::*;

    let conn = &mut establish_connection();
    // let recs = import_csv(conn)?;

    let mut shooters = shots.select(shooter_name).load::<String>(conn)?;
    shooters.sort();
    shooters.dedup();

    for shooter in shooters {
        println!("{}", shooter);
    }

    loop {
        let mut input = String::new();
        println!("who > ");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut name = String::new();
                name = input.trim().to_string();

                let results = shots
                    .filter(shooter_name.eq(&name))
                    .load::<shot::Shot>(conn)?;

                for shot_result in results {
                    println!("{}", shot_result.time);
                    println!("{}", shot_result.game_id);
                }

                name.clear();
            }
            Err(error) => println!("Error: {}", error),
        }

        input.clear();
    }
}
