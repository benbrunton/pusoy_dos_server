use std::panic::RefUnwindSafe;

use controller::{Controller, ResponseType};
use serde_json;
use helpers::{PathExtractor, QueryStringExtractor};
use model::Session;

use helpers;

use data_access::round::Round as RoundData;

#[derive(Clone)]
pub struct LastMoveController{
    round_data: RoundData
}

impl LastMoveController {
    pub fn new(round_data: RoundData) -> LastMoveController {
        LastMoveController {
            round_data: round_data
        }
    }

    fn get_json(&self, _:u64, game_id: u64) -> String {

        // TODO - only access players endpoint if user is in game

        let round_result = self.round_data.get(game_id);
        /*
        match round_result {
            None => {
                info!("returning error as no round found for game id: {}", game_id);
                return self.output_error();
            },
            _ => ()
        }
        */

        let round = round_result.expect("failed to load round");
        let last_move = round.round.get_last_move();
        let displayed_last_move = helpers::convert_move_to_display_cards(last_move);

        serde_json::to_string(&displayed_last_move).unwrap()
    }

/*
    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))

    }
*/
}

impl Controller for LastMoveController {
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

impl RefUnwindSafe for LastMoveController {}
