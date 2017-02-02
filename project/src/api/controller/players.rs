use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use rustc_serialize::json;
use std::collections::BTreeMap;
use serde_json;
use serde_json::{Value, Map};

use helpers;

use data_access::round::Round as RoundData;
use data_access::user::User as UserData;

use pusoy_dos::game::game::Game;

pub struct Players{
    round_data: RoundData,
    user_data: UserData
}

impl Players {
    pub fn new(round_data: RoundData, user_data: UserData) -> Players {
        Players {
            round_data: round_data,
            user_data: user_data
        }
    }

    fn output_players(&self, user_id:u64, game_id: u64) -> Response {

        // TODO - only access players endpoint if user is in game

        let round_result = self.round_data.get(game_id);
        match round_result {
            None => {
                info!("returning error as no round found for game id: {}", game_id);
                return self.output_error();
            },
            _ => ()
        }

        let round = round_result.expect("failed to load round");
        let game = Game::load(round.clone()).expect("game failed to load");
        let next_player = game.get_next_player().expect("unable to get next player");
        let next_player_id = next_player.get_id();
        let reversed = round.reversed;

        let players = self.user_data.get_users_by_game(game_id);

        let content_type = "application/json".parse::<Mime>().unwrap();
        // TODO - winning player condition
        let output_players = players.iter().map(|ref player|{
            let mut p = Map::new();
            p.insert("id".to_string(), Value::U64(player.id));
            p.insert("name".to_string(), Value::String(player.name.clone()));
            p.insert("next".to_string(), Value::Bool(player.id == next_player_id));
            p.insert("loggedIn".to_string(), Value::Bool(player.id == user_id));
            p.insert("reversed".to_string(), Value::Bool(reversed));
            p
        }).collect::<Vec<Map<String, Value>>>();

        let output = serde_json::to_string(&output_players).unwrap();
        Response::with((content_type, status::Ok, output))
   
    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))

    }
}

impl Handler for Players {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);

        let resp = match session_user_id {
            Some(user_id) => {
                info!("valid user - checking game id");
                match *query {
                    Some(id) => {
                        self.output_players(user_id, id.parse::<u64>().unwrap())
                    },
                    _ => {
                        info!("invalid id: {:?}", query);
                        self.output_error()
                    }
                }
            },
            _ => self.output_error()
        };

        Ok(resp)
    }
}
