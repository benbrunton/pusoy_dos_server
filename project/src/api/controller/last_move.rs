use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use rustc_serialize::json;
use std::collections::BTreeMap;
use serde_json;

use helpers;

use data_access::round::Round as RoundData;

#[derive(Clone)]
pub struct LastMove{
    round_data: RoundData
}

impl LastMove {
    pub fn new(round_data: RoundData) -> LastMove {
        LastMove {
            round_data: round_data
        }
    }

    fn output(&self, _:u64, game_id: u64) -> Response {

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
        let last_move = round.round.get_last_move();
        let displayed_last_move = helpers::convert_move_to_display_cards(last_move);

        Response::with((content_type, status::Ok, serde_json::to_string(&displayed_last_move).unwrap()))
   
    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))

    }
}

impl Handler for LastMove {

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
