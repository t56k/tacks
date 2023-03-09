#![recursion_limit = "512"]

use anyhow::Result;
use std::io;
use tacks::*;

mod game;
mod player;
mod schema;
mod shot;

fn main() -> Result<()> {
    let conn = &mut establish_connection();

    loop {
        println!("> ");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut cmd = input.trim();
        match cmd {
            "games" => {
                println!("importing games...");
                import_games(conn, "data/all_games.csv")?;
            }
            "shots" => {
                println!("importing shots...");
                import_shots(conn, "data/shots_2022_1.csv")?;
            }
            "players" => {
                println!("fetching players...");
                let players = shot::Shot::all_shooters(conn)?;

                for player in players {
                    println!("{}", player);
                }
            }
            _ => {
                let shots = shot::Shot::by_shooter(conn, &cmd)?;

                for shot in shots {
                    println!("Time: {}", shot.time);
                    println!("Game: {}", shot.game_id);
                    println!("Goal: {}", shot.goal);
                }
            }
        }

        input.clear();
    }
}
