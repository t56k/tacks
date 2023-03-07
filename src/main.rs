#![recursion_limit = "512"]

use anyhow::Result;
use diesel::prelude::*;
use std::io;
use tacks::establish_connection;

mod models;
mod schema;

fn import_csv(connection: &mut PgConnection) -> Result<Vec<models::Shot>> {
    use crate::schema::shots::dsl::shots;

    let mut csv_reader = csv::Reader::from_reader(io::stdin());
    let mut new_shots = vec![];

    for result in csv_reader.deserialize() {
        let new_shot: models::Shot = result?;
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
    // let shots = import_csv(connection)?;

    let res = shots.limit(5).load::<models::Shot>(connection)?;

    println!("Displaying {} shots", res.len());

    for shot in res {
        println!("{}", shot.shooter_name);
        println!("{:#?}", shot.id);
    }

    Ok(())
}
