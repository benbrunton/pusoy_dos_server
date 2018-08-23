use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use std::collections::BTreeMap;
use serde_json;

use helpers;
use helpers::DCard;

use data_access::round::Round as RoundData;

use pusoy_dos::game::game::Game;

#[derive(Clone)]
pub struct MyCards{
    round_data: RoundData
}

impl MyCards {
    pub fn new(round_data: RoundData) -> MyCards {
        MyCards {
            round_data: round_data
        }
    }

    fn output(&self, user_id:u64, game_id: u64) -> Response {

        // TODO - only access players endpoint if user is in game

        let content_type = "application/json".parse::<Mime>().unwrap();
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
        let current_user = game.get_player(user_id).unwrap();

        let mut cards:Vec<DCard> = current_user
                                    .get_hand()
                                    .iter()
                                    .map(|&c|{ DCard::new(c.clone()) }).collect();
        cards.sort();
        cards.reverse();


        Response::with((content_type, status::Ok, serde_json::to_string(&cards).unwrap()))
   
    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))

    }
}

impl Handler for MyCards {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);

        let resp = match session_user_id {
            Some(user_id) => {
                info!("valid user - checking game id");
                match *query {
                    Some(id) => {
                        self.output(user_id, id.parse::<u64>().unwrap())
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
