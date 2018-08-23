use data_access::game::Game as GameData;
use model::Session;
use config::Config;
use helpers;
use helpers::{PathExtractor, QueryStringExtractor};

use controller::{Controller, ResponseType};
use std::panic::RefUnwindSafe;

pub struct GameJoinController{
    game_data: GameData,
    hostname: String
}

impl GameJoinController {
    pub fn new(config: &Config, game_data: GameData) -> GameJoinController {
        let hostname = config.get("pd_host").unwrap();

        GameJoinController{
            game_data: game_data,
            hostname: hostname
        }
    }

    fn get_page(&self, user: u64, game: u64) -> ResponseType {
        // join game - if successful redirect to game page
        // else error?
        let game_model_option = self.game_data.get_game(game);
        let error_message = format!("unable to unwrap game {}", game);
        let game_model = game_model_option.expect(&error_message);

        if game_model.started {
            return ResponseType::ServerError;
        }
        
        let _ = self.game_data.join_game(user, game);
        
        let game_url = format!("/game/{}", game);
        ResponseType::Redirect(game_url)
    }
}

impl Controller for GameJoinController {
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
            self.get_page(id, path_id)
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }
}

impl RefUnwindSafe for GameJoinController {}
