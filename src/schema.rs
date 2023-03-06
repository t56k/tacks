// @generated automatically by Diesel CLI.

diesel::table! {
    shots (id) {
        id -> Int4,
        arena_adjusted_shot_distance -> Float4,
        arena_adjusted_x_cord -> Float4,
        arena_adjusted_x_cord_abs -> Float4,
        arena_adjusted_y_cord -> Float4,
        arena_adjusted_y_cord_abs -> Float4,
        average_rest_difference -> Float4,
        away_empty_net -> Int4,
        away_penalty_1_length -> Int4,
        away_penalty_1_time_left -> Int4,
        away_skaters_on_ice -> Int4,
        away_team_code -> Text,
        away_team_goals -> Int4,
        defending_team_average_time_on_ice -> Float4,
        defending_team_average_time_on_ice_of_defencemen -> Float4,
        defending_team_average_time_on_ice_of_defencemen_since_faceoff -> Float4,
        defending_team_average_time_on_ice_of_forwards -> Float4,
        defending_team_average_time_on_ice_of_forwards_since_faceoff -> Float4,
        defending_team_average_time_on_ice_since_faceoff -> Float4,
        defending_team_defencemen_on_ice -> Int4,
        defending_team_forwards_on_ice -> Int4,
        defending_team_max_time_on_ice -> Int4,
        defending_team_max_time_on_ice_of_defencemen -> Int4,
        defending_team_max_time_on_ice_of_defencemen_since_faceoff -> Int4,
        defending_team_max_time_on_ice_of_forwards -> Int4,
        defending_team_max_time_on_ice_of_forwards_since_faceoff -> Int4,
        defending_team_max_time_on_ice_since_faceoff -> Int4,
        defending_team_min_time_on_ice -> Int4,
        defending_team_min_time_on_ice_of_defencemen -> Int4,
        defending_team_min_time_on_ice_of_defencemen_since_faceoff -> Int4,
        defending_team_min_time_on_ice_of_forwards -> Int4,
        defending_team_min_time_on_ice_of_forwards_since_faceoff -> Int4,
        defending_team_min_time_on_ice_since_faceoff -> Int4,
        distance_from_last_event -> Float4,
        event -> Text,
        game_id -> Int4,
        goal -> Int4,
        goalie_id_for_shot -> Int4,
        goalie_name_for_shot -> Text,
        home_empty_net -> Int4,
        home_penalty_1_length -> Int4,
        home_penalty_1_time_left -> Int4,
        home_skaters_on_ice -> Int4,
        home_team_code -> Text,
        home_team_goals -> Int4,
        home_team_won -> Int4,
        external_id -> Int4,
        is_home_team -> Float4,
        is_playoff_game -> Int4,
        last_event_category -> Text,
        last_event_shot_angle -> Float4,
        last_event_shot_distance -> Float4,
        last_event_team -> Text,
        last_eventx_cord -> Float4,
        last_eventx_cord_adjusted -> Float4,
        last_eventy_cord -> Float4,
        last_eventy_cord_adjusted -> Float4,
        location -> Text,
        off_wing -> Int4,
        period -> Int4,
        player_num_that_did_event -> Int4,
        player_num_that_did_last_event -> Int4,
        player_position_that_did_event -> Text,
        season -> Int4,
        shooter_left_right -> Text,
        shooter_name -> Text,
        shooter_player_id -> Nullable<Float4>,
        shooter_time_on_ice -> Float4,
        shooter_time_on_ice_since_faceoff -> Float4,
        shooting_team_average_time_on_ice -> Float4,
        shooting_team_average_time_on_ice_of_defencemen -> Float4,
        shooting_team_average_time_on_ice_of_defencemen_since_faceoff -> Float4,
        shooting_team_average_time_on_ice_of_forwards -> Float4,
        shooting_team_average_time_on_ice_of_forwards_since_faceoff -> Float4,
        shooting_team_average_time_on_ice_since_faceoff -> Float4,
        shooting_team_defencemen_on_ice -> Float4,
        shooting_team_forwards_on_ice -> Float4,
        shooting_team_max_time_on_ice -> Float4,
        shooting_team_max_time_on_ice_of_defencemen -> Float4,
        shooting_team_max_time_on_ice_of_defencemen_since_faceoff -> Float4,
        shooting_team_max_time_on_ice_of_forwards -> Float4,
        shooting_team_max_time_on_ice_of_forwards_since_faceoff -> Float4,
        shooting_team_max_time_on_ice_since_faceoff -> Float4,
        shooting_team_min_time_on_ice -> Float4,
        shooting_team_min_time_on_ice_of_defencemen -> Float4,
        shooting_team_min_time_on_ice_of_defencemen_since_faceoff -> Float4,
        shooting_team_min_time_on_ice_of_forwards -> Float4,
        shooting_team_min_time_on_ice_of_forwards_since_faceoff -> Float4,
        shooting_team_min_time_on_ice_since_faceoff -> Float4,
        shot_angle -> Float4,
        shot_angle_adjusted -> Float4,
        shot_angle_plus_rebound -> Float4,
        shot_angle_plus_rebound_speed -> Float4,
        shot_angle_rebound_royal_road -> Float4,
        shot_distance -> Float4,
        shot_generated_rebound -> Float4,
        shot_goalie_froze -> Float4,
        shot_on_empty_net -> Float4,
        shot_play_continued_in_zone -> Float4,
        shot_play_continued_outside_zone -> Float4,
        shot_play_stopped -> Float4,
        shot_rebound -> Float4,
        shot_rush -> Float4,
        shot_type -> Text,
        shot_was_on_goal -> Float4,
        speed_from_last_event -> Float4,
        team -> Text,
        team_code -> Text,
        time -> Int4,
        time_difference_since_change -> Float4,
        time_since_faceoff -> Float4,
        time_since_last_event -> Float4,
        time_until_next_event -> Float4,
        x_cord -> Int4,
        x_cord_adjusted -> Int4,
        x_froze -> Float4,
        x_goal -> Float4,
        x_play_continued_in_zone -> Float4,
        x_play_continued_outside_zone -> Float4,
        x_play_stopped -> Float4,
        x_rebound -> Float4,
        x_shot_was_on_goal -> Float4,
        y_cord -> Int4,
        y_cord_adjusted -> Int4,
    }
}
