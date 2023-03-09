use anyhow::Result;
use diesel::prelude::*;
use serde::*;

use crate::schema::shots;

impl Shot {
    pub fn all_shooters(conn: &mut PgConnection) -> Result<Vec<String>> {
        use crate::schema::shots::dsl::*;

        Ok(shots.select(shooter_name).load::<String>(conn)?)
    }

    pub fn by_shooter(conn: &mut PgConnection, name: &str) -> Result<Vec<Shot>> {
        use crate::schema::shots::dsl::*;

        Ok(shots.filter(shooter_name.eq(&name)).load::<Self>(conn)?)
    }
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Shot {
    pub id: i32,
    pub shot_id: i32,
    pub arena_adjusted_shot_distance: f32,
    pub arena_adjusted_x_cord: f32,
    pub arena_adjusted_x_cord_abs: f32,
    pub arena_adjusted_y_cord: f32,
    pub arena_adjusted_y_cord_abs: f32,
    pub average_rest_difference: f32,
    pub away_empty_net: i32,
    pub away_penalty_1_length: i32,
    pub away_penalty_1_time_left: i32,
    pub away_skaters_on_ice: i32,
    pub away_team_code: String,
    pub away_team_goals: i32,
    pub defending_team_average_time_on_ice: f32,
    pub defending_team_average_time_on_ice_of_defencemen: f32,
    pub defending_team_average_time_on_ice_of_defencemen_since_faceoff: f32,
    pub defending_team_average_time_on_ice_of_forwards: f32,
    pub defending_team_average_time_on_ice_of_forwards_since_faceoff: f32,
    pub defending_team_average_time_on_ice_since_faceoff: f32,
    pub defending_team_defencemen_on_ice: i32,
    pub defending_team_forwards_on_ice: i32,
    pub defending_team_max_time_on_ice: i32,
    pub defending_team_max_time_on_ice_of_defencemen: i32,
    pub defending_team_max_time_on_ice_of_defencemen_since_faceoff: i32,
    pub defending_team_max_time_on_ice_of_forwards: i32,
    pub defending_team_max_time_on_ice_of_forwards_since_faceoff: i32,
    pub defending_team_max_time_on_ice_since_faceoff: i32,
    pub defending_team_min_time_on_ice: i32,
    pub defending_team_min_time_on_ice_of_defencemen: i32,
    pub defending_team_min_time_on_ice_of_defencemen_since_faceoff: i32,
    pub defending_team_min_time_on_ice_of_forwards: i32,
    pub defending_team_min_time_on_ice_of_forwards_since_faceoff: i32,
    pub defending_team_min_time_on_ice_since_faceoff: i32,
    pub distance_from_last_event: f32,
    pub event: String,
    pub game_id: i32,
    pub goal: i32,
    pub goalie_id_for_shot: i32,
    pub goalie_name_for_shot: String,
    pub home_empty_net: i32,
    pub home_penalty_1_length: i32,
    pub home_penalty_1_time_left: i32,
    pub home_skaters_on_ice: i32,
    pub home_team_code: String,
    pub home_team_goals: i32,
    pub home_team_won: i32,
    pub external_id: i32,
    pub is_home_team: f32,
    pub is_playoff_game: i32,
    pub last_event_category: String,
    pub last_event_shot_angle: f32,
    pub last_event_shot_distance: f32,
    pub last_event_team: String,
    pub last_eventx_cord: f32,
    pub last_eventx_cord_adjusted: f32,
    pub last_eventy_cord: f32,
    pub last_eventy_cord_adjusted: f32,
    pub location: String,
    pub off_wing: i32,
    pub period: i32,
    pub player_num_that_did_event: i32,
    pub player_num_that_did_last_event: i32,
    pub player_position_that_did_event: String,
    pub season: i32,
    pub shooter_left_right: String,
    pub shooter_name: String,
    pub shooter_player_id: Option<f32>,
    pub shooter_time_on_ice: f32,
    pub shooter_time_on_ice_since_faceoff: f32,
    pub shooting_team_average_time_on_ice: f32,
    pub shooting_team_average_time_on_ice_of_defencemen: f32,
    pub shooting_team_average_time_on_ice_of_defencemen_since_faceoff: f32,
    pub shooting_team_average_time_on_ice_of_forwards: f32,
    pub shooting_team_average_time_on_ice_of_forwards_since_faceoff: f32,
    pub shooting_team_average_time_on_ice_since_faceoff: f32,
    pub shooting_team_defencemen_on_ice: f32,
    pub shooting_team_forwards_on_ice: f32,
    pub shooting_team_max_time_on_ice: f32,
    pub shooting_team_max_time_on_ice_of_defencemen: f32,
    pub shooting_team_max_time_on_ice_of_defencemen_since_faceoff: f32,
    pub shooting_team_max_time_on_ice_of_forwards: f32,
    pub shooting_team_max_time_on_ice_of_forwards_since_faceoff: f32,
    pub shooting_team_max_time_on_ice_since_faceoff: f32,
    pub shooting_team_min_time_on_ice: f32,
    pub shooting_team_min_time_on_ice_of_defencemen: f32,
    pub shooting_team_min_time_on_ice_of_defencemen_since_faceoff: f32,
    pub shooting_team_min_time_on_ice_of_forwards: f32,
    pub shooting_team_min_time_on_ice_of_forwards_since_faceoff: f32,
    pub shooting_team_min_time_on_ice_since_faceoff: f32,
    pub shot_angle: f32,
    pub shot_angle_adjusted: f32,
    pub shot_angle_plus_rebound: f32,
    pub shot_angle_plus_rebound_speed: f32,
    pub shot_angle_rebound_royal_road: f32,
    pub shot_distance: f32,
    pub shot_generated_rebound: f32,
    pub shot_goalie_froze: f32,
    pub shot_on_empty_net: f32,
    pub shot_play_continued_in_zone: f32,
    pub shot_play_continued_outside_zone: f32,
    pub shot_play_stopped: f32,
    pub shot_rebound: f32,
    pub shot_rush: f32,
    pub shot_type: String,
    pub shot_was_on_goal: f32,
    pub speed_from_last_event: f32,
    pub team: String,
    pub team_code: String,
    pub time: i32,
    pub time_difference_since_change: f32,
    pub time_since_faceoff: f32,
    pub time_since_last_event: f32,
    pub time_until_next_event: f32,
    pub x_cord: i32,
    pub x_cord_adjusted: i32,
    pub x_froze: f32,
    pub x_goal: f32,
    pub x_play_continued_in_zone: f32,
    pub x_play_continued_outside_zone: f32,
    pub x_play_stopped: f32,
    pub x_rebound: f32,
    pub x_shot_was_on_goal: f32,
    pub y_cord: i32,
    pub y_cord_adjusted: i32,
}

#[derive(Deserialize, Insertable, PartialEq, Debug)]
#[diesel(table_name = shots)]
pub struct NewShot {
    pub shot_id: i32,
    pub arena_adjusted_shot_distance: f32,
    pub arena_adjusted_x_cord: f32,
    pub arena_adjusted_x_cord_abs: f32,
    pub arena_adjusted_y_cord: f32,
    pub arena_adjusted_y_cord_abs: f32,
    pub average_rest_difference: f32,
    pub away_empty_net: i32,
    pub away_penalty_1_length: i32,
    pub away_penalty_1_time_left: i32,
    pub away_skaters_on_ice: i32,
    pub away_team_code: String,
    pub away_team_goals: i32,
    pub defending_team_average_time_on_ice: f32,
    pub defending_team_average_time_on_ice_of_defencemen: f32,
    pub defending_team_average_time_on_ice_of_defencemen_since_faceoff: f32,
    pub defending_team_average_time_on_ice_of_forwards: f32,
    pub defending_team_average_time_on_ice_of_forwards_since_faceoff: f32,
    pub defending_team_average_time_on_ice_since_faceoff: f32,
    pub defending_team_defencemen_on_ice: i32,
    pub defending_team_forwards_on_ice: i32,
    pub defending_team_max_time_on_ice: i32,
    pub defending_team_max_time_on_ice_of_defencemen: i32,
    pub defending_team_max_time_on_ice_of_defencemen_since_faceoff: i32,
    pub defending_team_max_time_on_ice_of_forwards: i32,
    pub defending_team_max_time_on_ice_of_forwards_since_faceoff: i32,
    pub defending_team_max_time_on_ice_since_faceoff: i32,
    pub defending_team_min_time_on_ice: i32,
    pub defending_team_min_time_on_ice_of_defencemen: i32,
    pub defending_team_min_time_on_ice_of_defencemen_since_faceoff: i32,
    pub defending_team_min_time_on_ice_of_forwards: i32,
    pub defending_team_min_time_on_ice_of_forwards_since_faceoff: i32,
    pub defending_team_min_time_on_ice_since_faceoff: i32,
    pub distance_from_last_event: f32,
    pub event: String,
    pub game_id: i32,
    pub goal: i32,
    pub goalie_id_for_shot: i32,
    pub goalie_name_for_shot: String,
    pub home_empty_net: i32,
    pub home_penalty_1_length: i32,
    pub home_penalty_1_time_left: i32,
    pub home_skaters_on_ice: i32,
    pub home_team_code: String,
    pub home_team_goals: i32,
    pub home_team_won: i32,
    pub external_id: i32,
    pub is_home_team: f32,
    pub is_playoff_game: i32,
    pub last_event_category: String,
    pub last_event_shot_angle: f32,
    pub last_event_shot_distance: f32,
    pub last_event_team: String,
    pub last_eventx_cord: f32,
    pub last_eventx_cord_adjusted: f32,
    pub last_eventy_cord: f32,
    pub last_eventy_cord_adjusted: f32,
    pub location: String,
    pub off_wing: i32,
    pub period: i32,
    pub player_num_that_did_event: i32,
    pub player_num_that_did_last_event: i32,
    pub player_position_that_did_event: String,
    pub season: i32,
    pub shooter_left_right: String,
    pub shooter_name: String,
    pub shooter_player_id: Option<f32>,
    pub shooter_time_on_ice: f32,
    pub shooter_time_on_ice_since_faceoff: f32,
    pub shooting_team_average_time_on_ice: f32,
    pub shooting_team_average_time_on_ice_of_defencemen: f32,
    pub shooting_team_average_time_on_ice_of_defencemen_since_faceoff: f32,
    pub shooting_team_average_time_on_ice_of_forwards: f32,
    pub shooting_team_average_time_on_ice_of_forwards_since_faceoff: f32,
    pub shooting_team_average_time_on_ice_since_faceoff: f32,
    pub shooting_team_defencemen_on_ice: f32,
    pub shooting_team_forwards_on_ice: f32,
    pub shooting_team_max_time_on_ice: f32,
    pub shooting_team_max_time_on_ice_of_defencemen: f32,
    pub shooting_team_max_time_on_ice_of_defencemen_since_faceoff: f32,
    pub shooting_team_max_time_on_ice_of_forwards: f32,
    pub shooting_team_max_time_on_ice_of_forwards_since_faceoff: f32,
    pub shooting_team_max_time_on_ice_since_faceoff: f32,
    pub shooting_team_min_time_on_ice: f32,
    pub shooting_team_min_time_on_ice_of_defencemen: f32,
    pub shooting_team_min_time_on_ice_of_defencemen_since_faceoff: f32,
    pub shooting_team_min_time_on_ice_of_forwards: f32,
    pub shooting_team_min_time_on_ice_of_forwards_since_faceoff: f32,
    pub shooting_team_min_time_on_ice_since_faceoff: f32,
    pub shot_angle: f32,
    pub shot_angle_adjusted: f32,
    pub shot_angle_plus_rebound: f32,
    pub shot_angle_plus_rebound_speed: f32,
    pub shot_angle_rebound_royal_road: f32,
    pub shot_distance: f32,
    pub shot_generated_rebound: f32,
    pub shot_goalie_froze: f32,
    pub shot_on_empty_net: f32,
    pub shot_play_continued_in_zone: f32,
    pub shot_play_continued_outside_zone: f32,
    pub shot_play_stopped: f32,
    pub shot_rebound: f32,
    pub shot_rush: f32,
    pub shot_type: String,
    pub shot_was_on_goal: f32,
    pub speed_from_last_event: f32,
    pub team: String,
    pub team_code: String,
    pub time: i32,
    pub time_difference_since_change: f32,
    pub time_since_faceoff: f32,
    pub time_since_last_event: f32,
    pub time_until_next_event: f32,
    pub x_cord: i32,
    pub x_cord_adjusted: i32,
    pub x_froze: f32,
    pub x_goal: f32,
    pub x_play_continued_in_zone: f32,
    pub x_play_continued_outside_zone: f32,
    pub x_play_stopped: f32,
    pub x_rebound: f32,
    pub x_shot_was_on_goal: f32,
    pub y_cord: i32,
    pub y_cord_adjusted: i32,
}
