use data_access::game::Game as GameData;
use data_access::event::Event as EventData;
use data_access::round::Round as RoundData;

use chrono::prelude::*;
use pusoy_dos::game::game::Game;

pub fn execute(game_data: GameData, event_data: EventData, round_data:RoundData) {

    let games = game_data.get_started_games_with_move_limit();

    info!("reviewing latest moves on open games");
    info!("number of open games with move limit: {:?}", games.len());

    let utc: DateTime<UTC> = UTC::now();
        
    for &(id, duration) in games.iter() {

        if duration == 0 {
            break;
        }

        let date = event_data.get_last_game_event(id); 

        match date {
            Some(d) => {

                if utc.signed_duration_since(d).num_minutes() < duration as i64 {
                    break;
                }
                
                let round_result = round_data.get(id);
                
                match round_result {
                    None => {
                        info!("no round found for game {}", id);
                        break;
                    },
                    _ => ()
                }

                info!("loading game: {}", id);

                let round = round_result.expect("error with round result");

                let game = Game::load(round.clone()).expect("error loading game");
                let next_player_result = game.get_next_player();

                match next_player_result {
                    None => {
                        info!("aint no next player {}", id);
                        break;
                    },
                    _ => ()
                }
                
                let next_player = next_player_result.unwrap().get_id();

                let valid_move = game.player_move(next_player, vec!());

                match valid_move {
                    Ok(updated_game) => {
                        round_data.update_round(id, updated_game.clone());
                        event_data.insert_game_event(next_player, id, "[]".to_string());

                        info!("auto passed game {}", id);

                    },
                    _ => {
                        info!("invalid_move!");
                        break;
                    }
                }

            },
            _ => ()
        }
           
    }
}
