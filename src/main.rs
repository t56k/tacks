#![recursion_limit = "512"]

use anyhow::Result;
use diesel::prelude::*;
use std::io;
use tacks::establish_connection;

mod schema;
mod shot;

fn import_csv(connection: &mut PgConnection) -> Result<Vec<shot::NewShot>> {
    use crate::schema::shots::dsl::shots;

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
    use crate::schema::shots::dsl::shots;

    let connection = &mut establish_connection();

    let res = shots.limit(1).load::<shot::Shot>(connection)?;
    if res.len() < 1 {
        let _ = import_csv(connection)?;
    }

    let res = shots.limit(5).load::<shot::Shot>(connection)?;
    println!("Displaying {} shots", res.len());

    for s in res {
        println!("{:#?}", s);
    }

    Ok(())
}
