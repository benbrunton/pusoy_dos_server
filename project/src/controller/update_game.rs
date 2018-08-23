use std::panic::RefUnwindSafe;
use model::Session;
use data_access::game::Game as GameData;
use controller::{Controller, ResponseType};
use helpers;
use helpers::{PathExtractor, QueryStringExtractor};
use url::form_urlencoded::parse;


pub struct UpdateGameController {
    game_data: GameData
}

impl UpdateGameController {
    pub fn new(game_data: GameData) -> UpdateGameController {
        UpdateGameController{ game_data }
    }

    fn update_game(&self, id: u64, decks: u64) {
        self.game_data.update_decks(id, decks);
    }
}

impl Controller for UpdateGameController {

    fn get_response(
        &self,
        session:&mut Option<Session>,
        body: Option<String>,
        path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        
        if helpers::is_logged_in(session) {
            let id = path.expect("no_path").id as u64;
            let mut decks = 0;
            info!("{:?}", body);
            match body {
                Some(b) => {
                    let parsed_body = parse(b.as_bytes());
                    let pairs = parsed_body.into_owned();
                    for (key, val) in pairs {
                        match key.as_ref() {
                            "decks" => {
                                decks = val.parse::<u64>().unwrap().to_owned();
                            },
                            _ => ()
                        }
                    }
                    self.update_game(id, decks);
                    return ResponseType::Redirect(format!("/game/{}", id))
                },
                _ => ()
            }
            
        }

        ResponseType::Redirect("/".to_string())
    }
}

impl RefUnwindSafe for UpdateGameController {}

