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

        let cmd: Vec<&str> = input.trim().split_whitespace().collect();
        match cmd.get(0).unwrap_or(&"") {
            &"games" => {
                let games_file = cmd.get(1).unwrap_or(&"data/all_teams.csv");
                println!("importing games from {}...", games_file);
                import_games(conn, games_file)?;
            }
            &"shots" => {
                let shots_file = cmd.get(1).unwrap_or(&"data/shots_2022_1.csv");
                println!("importing shots from {}...", shots_file);
                import_shots(conn, shots_file)?;
            }
            &"players" => {
                println!("fetching players...");
                let players = shot::Shot::all_shooters(conn)?;

                for player in players {
                    println!("{}", player);
                }
            }
            _ => {
                let name = cmd.join(" ");
                let shots = shot::Shot::by_shooter(conn, &name)?;
                println!("fetching {}...", name);

                for shot in shots {
                    println!("Time: {}", shot.time);
                    println!("Game: {}", shot.game_id);
                    println!("Goal: {}", shot.goal);
                }
            }
        }
    }
}
