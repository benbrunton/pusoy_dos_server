use data_access::game::Game as GameData;
use data_access::round::Round as RoundData;
use config::Config;
use helpers;
use controller::{Controller, ResponseType};
use std::panic::RefUnwindSafe;
use model::Session;
use helpers::{PathExtractor, QueryStringExtractor};


use pusoy_dos::game::game::Game as CardGame;

pub struct BeginGameController {
    game_data: GameData,
    round_data: RoundData,
    hostname: String
}

impl BeginGameController {
    pub fn new(config: &Config, game_data: GameData, round_data: RoundData) -> BeginGameController {
        let hostname = config.get("pd_host").unwrap();
        BeginGameController {
            game_data: game_data,
            round_data: round_data,
            hostname: hostname
        }
    }

    fn begin_game(&self, user:u64, game_id:u64) {
        let game_data = self.game_data.get_game(game_id);

        match game_data {
            None => return,
            Some(game) => {
                if game.creator_id != user {
                    return;
                }

                if game.started {
                    return;
                }
            }
        }

        let _ = self.game_data.start_game(game_id);        
        let users = self.game_data.get_players(game_id);
        let game = self.game_data.get_game(game_id).unwrap();
        // create a new game
        let new_game = CardGame::setup(users, game.decks as usize).unwrap();
        self.round_data.create_round(game_id, new_game);
    }
}

impl Controller for BeginGameController {
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
            self.begin_game(id, path_id);
            let game_page = format!("/play/{}", path_id);
            ResponseType::Redirect(game_page)
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }
}

impl RefUnwindSafe for BeginGameController {}
