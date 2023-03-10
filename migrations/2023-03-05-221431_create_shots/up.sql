CREATE TABLE shots (
  id SERIAL PRIMARY KEY,
  shot_id INTEGER NOT NULL,
  arena_adjusted_shot_distance REAL NOT NULL,
  arena_adjusted_x_cord REAL NOT NULL,
  arena_adjusted_x_cord_abs REAL NOT NULL,
  arena_adjusted_y_cord REAL NOT NULL,
  arena_adjusted_y_cord_abs REAL NOT NULL,
  average_rest_difference REAL NOT NULL,
  away_empty_net INTEGER NOT NULL,
  away_penalty_1_length INTEGER NOT NULL,
  away_penalty_1_time_left INTEGER NOT NULL,
  away_skaters_on_ice INTEGER NOT NULL,
  away_team_code TEXT NOT NULL,
  away_team_goals INTEGER NOT NULL,
  defending_team_average_time_on_ice REAL NOT NULL,
  defending_team_average_time_on_ice_of_defencemen REAL NOT NULL,
  defending_team_average_time_on_ice_of_defencemen_since_faceoff REAL NOT NULL,
  defending_team_average_time_on_ice_of_forwards REAL NOT NULL,
  defending_team_average_time_on_ice_of_forwards_since_faceoff REAL NOT NULL,
  defending_team_average_time_on_ice_since_faceoff REAL NOT NULL,
  defending_team_defencemen_on_ice INTEGER NOT NULL,
  defending_team_forwards_on_ice INTEGER NOT NULL,
  defending_team_max_time_on_ice INTEGER NOT NULL,
  defending_team_max_time_on_ice_of_defencemen INTEGER NOT NULL,
  defending_team_max_time_on_ice_of_defencemen_since_faceoff INTEGER NOT NULL,
  defending_team_max_time_on_ice_of_forwards INTEGER NOT NULL,
  defending_team_max_time_on_ice_of_forwards_since_faceoff INTEGER NOT NULL,
  defending_team_max_time_on_ice_since_faceoff INTEGER NOT NULL,
  defending_team_min_time_on_ice INTEGER NOT NULL,
  defending_team_min_time_on_ice_of_defencemen INTEGER NOT NULL,
  defending_team_min_time_on_ice_of_defencemen_since_faceoff INTEGER NOT NULL,
  defending_team_min_time_on_ice_of_forwards INTEGER NOT NULL,
  defending_team_min_time_on_ice_of_forwards_since_faceoff INTEGER NOT NULL,
  defending_team_min_time_on_ice_since_faceoff INTEGER NOT NULL,
  distance_from_last_event REAL NOT NULL,
  event TEXT NOT NULL,
  game_id INTEGER NOT NULL,
  goal INTEGER NOT NULL,
  goalie_id_for_shot INTEGER NOT NULL,
  goalie_name_for_shot TEXT NOT NULL,
  home_empty_net INTEGER NOT NULL,
  home_penalty_1_length INTEGER NOT NULL,
  home_penalty_1_time_left INTEGER NOT NULL,
  home_skaters_on_ice INTEGER NOT NULL,
  home_team_code TEXT NOT NULL,
  home_team_goals INTEGER NOT NULL,
  home_team_won INTEGER NOT NULL,
  external_id INTEGER NOT NULL,
  is_home_team REAL NOT NULL,
  is_playoff_game INTEGER NOT NULL,
  last_event_category TEXT NOT NULL,
  last_event_shot_angle REAL NOT NULL,
  last_event_shot_distance REAL NOT NULL,
  last_event_team TEXT NOT NULL,
  last_eventx_cord REAL NOT NULL,
  last_eventx_cord_adjusted REAL NOT NULL,
  last_eventy_cord REAL NOT NULL,
  last_eventy_cord_adjusted REAL NOT NULL,
  location TEXT NOT NULL,
  off_wing INTEGER NOT NULL,
  period INTEGER NOT NULL,
  player_num_that_did_event INTEGER NOT NULL,
  player_num_that_did_last_event INTEGER NOT NULL,
  player_position_that_did_event TEXT NOT NULL,
  season INTEGER NOT NULL,
  shooter_left_right TEXT NOT NULL,
  shooter_name TEXT NOT NULL,
  shooter_player_id REAL,
  shooter_time_on_ice REAL NOT NULL,
  shooter_time_on_ice_since_faceoff REAL NOT NULL,
  shooting_team_average_time_on_ice REAL NOT NULL,
  shooting_team_average_time_on_ice_of_defencemen REAL NOT NULL,
  shooting_team_average_time_on_ice_of_defencemen_since_faceoff REAL NOT NULL,
  shooting_team_average_time_on_ice_of_forwards REAL NOT NULL,
  shooting_team_average_time_on_ice_of_forwards_since_faceoff REAL NOT NULL,
  shooting_team_average_time_on_ice_since_faceoff REAL NOT NULL,
  shooting_team_defencemen_on_ice REAL NOT NULL,
  shooting_team_forwards_on_ice REAL NOT NULL,
  shooting_team_max_time_on_ice REAL NOT NULL,
  shooting_team_max_time_on_ice_of_defencemen REAL NOT NULL,
  shooting_team_max_time_on_ice_of_defencemen_since_faceoff REAL NOT NULL,
  shooting_team_max_time_on_ice_of_forwards REAL NOT NULL,
  shooting_team_max_time_on_ice_of_forwards_since_faceoff REAL NOT NULL,
  shooting_team_max_time_on_ice_since_faceoff REAL NOT NULL,
  shooting_team_min_time_on_ice REAL NOT NULL,
  shooting_team_min_time_on_ice_of_defencemen REAL NOT NULL,
  shooting_team_min_time_on_ice_of_defencemen_since_faceoff REAL NOT NULL,
  shooting_team_min_time_on_ice_of_forwards REAL NOT NULL,
  shooting_team_min_time_on_ice_of_forwards_since_faceoff REAL NOT NULL,
  shooting_team_min_time_on_ice_since_faceoff REAL NOT NULL,
  shot_angle REAL NOT NULL,
  shot_angle_adjusted REAL NOT NULL,
  shot_angle_plus_rebound REAL NOT NULL,
  shot_angle_plus_rebound_speed REAL NOT NULL,
  shot_angle_rebound_royal_road REAL NOT NULL,
  shot_distance REAL NOT NULL,
  shot_generated_rebound REAL NOT NULL,
  shot_goalie_froze REAL NOT NULL,
  shot_on_empty_net REAL NOT NULL,
  shot_play_continued_in_zone REAL NOT NULL,
  shot_play_continued_outside_zone REAL NOT NULL,
  shot_play_stopped REAL NOT NULL,
  shot_rebound REAL NOT NULL,
  shot_rush REAL NOT NULL,
  shot_type TEXT NOT NULL,
  shot_was_on_goal REAL NOT NULL,
  speed_from_last_event REAL NOT NULL,
  team TEXT NOT NULL,
  team_code TEXT NOT NULL,
  time INTEGER NOT NULL,
  time_difference_since_change REAL NOT NULL,
  time_since_faceoff REAL NOT NULL,
  time_since_last_event REAL NOT NULL,
  time_until_next_event REAL NOT NULL,
  x_cord INTEGER NOT NULL,
  x_cord_adjusted INTEGER NOT NULL,
  x_froze REAL NOT NULL,
  x_goal REAL NOT NULL,
  x_play_continued_in_zone REAL NOT NULL,
  x_play_continued_outside_zone REAL NOT NULL,
  x_play_stopped REAL NOT NULL,
  x_rebound REAL NOT NULL,
  x_shot_was_on_goal REAL NOT NULL,
  y_cord INTEGER NOT NULL,
  y_cord_adjusted INTEGER NOT NULL
)
