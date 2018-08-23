use std::panic::RefUnwindSafe;
use controller::{Controller, ResponseType};
use serde_json;
use serde_json::{Value, Map};
use helpers::{PathExtractor, QueryStringExtractor};
use model::Session;

use helpers;

use data_access::round::Round as RoundData;
use data_access::user::User as UserData;
use data_access::event::Event as EventData;

use pusoy_dos::game::game::Game;

pub struct PlayersController{
    round_data: RoundData,
    user_data: UserData,
    event_data: EventData
}

impl PlayersController {
    pub fn new(round_data: RoundData, user_data: UserData, event_data: EventData) -> PlayersController {
        PlayersController {
            round_data: round_data,
            user_data: user_data,
            event_data: event_data
        }
    }

    fn get_json(&self, user_id:u64, game_id: u64) -> String {

        // TODO - only access players endpoint if user is in game
        let round_result = self.round_data.get(game_id);
/*        match round_result {
            None => {
                info!("returning error as no round found for game id: {}", game_id);
                return self.output_error();
            },
            _ => ()
        }
*/

        let round = round_result.expect("failed to load round");
        let game = Game::load(round.clone()).expect("game failed to load");
        let next_player = game.get_next_player().expect("unable to get next player");
        let next_player_id = next_player.get_id();
        let reversed = round.reversed;

        info!("getting game events");
        let events = self.event_data.get_game_events(game_id);
        let players = self.user_data.get_users_by_game(game_id);

        // TODO - winning player condition
        let output_players = players.iter().map(|ref player|{
            let mut p = Map::new();

            for event in &events {
                if event.match_user_id(player.id) {
                    p.insert("move".to_string(), json!(event.get_message()));
                    p.insert("move_time".to_string(), json!(event.get_time()));
                    break;
                }
            }

            p.insert("id".to_string(), json!(player.id));
            p.insert("name".to_string(), json!(player.name.clone()));
            p.insert("next".to_string(), json!(player.id == next_player_id));
            p.insert("loggedIn".to_string(), json!(player.id == user_id));
            p.insert("reversed".to_string(), json!(reversed));
            p.insert("winner".to_string(), json!(round.winners.len() > 0 && round.winners[0] == player.id));
            let card_count = game.get_player(player.id).unwrap().get_hand().len() as u64;
            p.insert("cardCount".to_string(), json!(card_count));
            let players_still_playing =  round.round.export().players;
            p.insert("stillIn".to_string(), json!(players_still_playing.iter().any(|&e| e == player.id)));
            p
        }).collect::<Vec<Map<String, Value>>>();

        serde_json::to_string(&output_players).unwrap()
    }

}

impl Controller for PlayersController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            let path_id = path.expect("no_path").id as u64;
            let json = self.get_json(id, path_id);
            ResponseType::Json(json)
        } else {
           ResponseType::Json("{}".to_string())
        }
    }
}

impl RefUnwindSafe for PlayersController {}
