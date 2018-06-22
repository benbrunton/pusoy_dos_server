use std::panic::RefUnwindSafe;

use controller::{Controller, ResponseType};
use serde_json;
use helpers::PathExtractor;
use model::Session;
use helpers::DCard;
use helpers;

use data_access::round::Round as RoundData;

use pusoy_dos::game::game::Game;

#[derive(Clone)]
pub struct MyCardsController{
    round_data: RoundData
}

impl MyCardsController {
    pub fn new(round_data: RoundData) -> MyCardsController {
        MyCardsController {
            round_data: round_data
        }
    }

    fn get_json(&self, user_id:u64, game_id: u64) -> String {

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
        let game = Game::load(round.clone()).expect("game failed to load");
        let current_user = game.get_player(user_id).unwrap();

        let mut cards:Vec<DCard> = current_user
                                    .get_hand()
                                    .iter()
                                    .map(|&c|{ DCard::new(c.clone()) }).collect();
        cards.sort();
        cards.reverse();


        serde_json::to_string(&cards).unwrap()
   
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

impl Controller for MyCardsController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        path: Option<PathExtractor>
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

impl RefUnwindSafe for MyCardsController {}
