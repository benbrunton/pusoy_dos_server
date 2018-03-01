use data_access::game::Game as GameData;
use data_access::event::Event as EventData;
use data_access::round::Round as RoundData;

use chrono::prelude::*;
use pusoy_dos::game::game::{ Game, GameDefinition };

pub fn execute(game_data: GameData, event_data: EventData, round_data:RoundData) {

    let games = game_data.get_started_games_with_move_limit();

    info!("reviewing latest moves on open games");
    info!("number of open games with move limit: {:?}", games.len());

    let utc: DateTime<UTC> = UTC::now();
    info!("starting sweep at {}", utc);
        
    for &(id, duration) in games.iter() {

        if duration == 0 {
            continue;
        }

        let date = event_data.get_last_game_event(id); 
        info!("got date for {}", id);
        info!(">> {:?}", date);

        match date {
            Some(d) => {

                if utc.signed_duration_since(d).num_minutes() < duration as i64 {
                    continue;
                }
                
                let round_result = round_data.get(id);
                
                match round_result {
                    None => {
                        info!("no round found for game {}", id);
                        continue;
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
                        continue;
                    },
                    _ => ()
                }
                
                let next_player = next_player_result.unwrap().get_id();

                let game_def = round.clone();
                let new_round = game_def.round.skip(next_player).expect("expected skip to work");

                let new_game_def = GameDefinition{ 
                    players: game_def.players.clone(),
                    round: new_round,
                    winners: game_def.winners.clone(),
                    reversed: game_def.reversed
                };

                round_data.update_round(id, new_game_def.clone());

                // todo - could indicate auto pass
                event_data.insert_game_event(next_player, id, "[]".to_string());

            },
            _ => ()
        }
           
    }

    let utc_complete: DateTime<UTC> = UTC::now();
    info!("completed sweep of expired moves at {}", utc);
    let sweep_duration = utc_complete.signed_duration_since(utc).num_milliseconds();
    info!("sweep took {} ms", sweep_duration);
}
