#![recursion_limit = "512"]

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::{insert_into, prelude::*};
use dotenvy::dotenv;
use std::env;

mod game;
mod schema;
mod shot;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn import_games(conn: &mut PgConnection, path: &str) -> Result<usize> {
    use schema::games::dsl::*;

    let mut csv_reader = csv::Reader::from_path(path)?;
    let mut new_games: Vec<game::NewGame> = Vec::new();

    for result in csv_reader.deserialize() {
        let new_game: game::NewGame = result?;
        new_games.push(new_game);
    }

    Ok(insert_into(games).values(&new_games).execute(conn)?)
}

pub fn import_shots(conn: &mut PgConnection, path: &str) -> Result<usize> {
    use schema::shots::dsl::*;

    let mut csv_reader = csv::Reader::from_path(path)?;
    let mut new_shots: Vec<shot::NewShot> = Vec::new();

    for result in csv_reader.deserialize() {
        let new_shot: shot::NewShot = result?;
        new_shots.push(new_shot);
    }

    Ok(insert_into(shots).values(&new_shots).execute(conn)?)
}
