use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use iron::mime::Mime;

use std::collections::HashMap;
use util::session::Session;
use urlencoded::UrlEncodedBody;
use config::Config;
use data_access::game::Game as GameData;
use helpers;

pub struct UpdateGame {
    hostname: String,
    game_data: GameData
}

impl UpdateGame {
    pub fn new(config: &Config, game_data: GameData) -> UpdateGame {
        let hostname = config.get("pd_host").unwrap();
        UpdateGame{ hostname: hostname, game_data: game_data }
    }

    fn update_game(&self, id: u64, hashmap:Option<HashMap<String, Vec<String>>>) {
        // todo - check that user is creator of game
        let params = hashmap.expect("unable to get params from POST");
        let decks_raw = params.get("decks").expect("expected decks").get(0).unwrap();
        let decks = decks_raw.parse::<u64>().expect("expected decks int");

        self.game_data.update_decks(id, decks);
    }

    fn get_hashmap(&self, req: &mut Request) -> Option<HashMap<String, Vec<String>>> {

        match req.get_ref::<UrlEncodedBody>(){
            Ok(hashmap) => Some(hashmap.to_owned().to_owned()),
            _ => None
        }
    }

}

impl Handler for UpdateGame {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref hashmap = {
            self.get_hashmap(req)
        };

//        let router = req.extensions.get::<Router>().unwrap();
//        let ref query = router.find("id");

        let session_user_id = match req.extensions.get::<Session>() {
            Some(session) => session.user_id,
            _             => None
        };

        let mut success = false;
        let full_url = match session_user_id {
            /*Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.update_game(
                            id.parse::<u64>().unwrap_or(0),
                            hashmap.to_owned());
                        String::from(format!("{}/game/{}", self.hostname, id))
                    },
                    _ => String::from(format!("{}/games", self.hostname))
                } 
            },
            */
            _ => String::from(format!("{}/games", self.hostname))
        };

        
        let url =  Url::parse(&full_url).unwrap();
        Ok(Response::with((status::Found, modifiers::Redirect(url))))


    }

}


