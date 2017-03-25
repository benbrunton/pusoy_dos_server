use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;

use urlencoded::UrlEncodedBody;
use std::collections::HashMap;

use config::Config;
use data_access::game::Game as GameData;
use util::session::Session;

pub struct GameCreate {
    hostname: String,
    game_data: GameData
}

impl GameCreate {
    pub fn new(config:&Config, game_data: GameData) -> GameCreate {
        let hostname = config.get("hostname").unwrap();
        GameCreate{ hostname: hostname, game_data: game_data }
    }

    fn insert_new_game(&self, id: u64, 
                hashmap:Option<HashMap<String, Vec<String>>>) -> Result<(), String>{

        let params = hashmap.expect("unable to get params from POST");
        let move_duration_raw = params.get("max-move-duration").expect("expected max_move_duration").get(0).unwrap();
        let move_duration = move_duration_raw.parse::<u64>().expect("expected int");
        self.game_data.create_game(id, move_duration, 4, 0);
        Ok(())
    }


    fn get_hashmap(&self, req: &mut Request) -> Option<HashMap<String, Vec<String>>> {

        match req.get_ref::<UrlEncodedBody>(){
            Ok(hashmap) => Some(hashmap.to_owned().to_owned()),
            _ => None
        }
    }

}

impl Handler for GameCreate {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session_user_id = match req.extensions.get::<Session>() {
            Some(session) => session.user_id,
            _             => None
        };

        let ref hashmap = self.get_hashmap(req);

        let mut success = false;
        match session_user_id {
            Some(id) => { 
                let _ = self.insert_new_game(id, hashmap.to_owned());
                success = true;
            },
            _ => ()        
        }

        let full_url = format!("{}/games?success={:?}", self.hostname, success);
        let url =  Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))

    }

}
