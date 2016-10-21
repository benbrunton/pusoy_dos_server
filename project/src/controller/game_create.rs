use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;

use logger;
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

    fn insert_new_game(&self, id: u64) -> Result<(), String>{

        self.game_data.create_game(id);
        Ok(())
    }
}

impl Handler for GameCreate {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session_user_id = match req.extensions.get::<Session>() {
            Some(session) => session.user_id,
            _             => None
        };

        let mut success = false;
        match session_user_id {
            Some(id) => { 
                self.insert_new_game(id);
                success = true;
            },
            _ => ()        
        }

        let full_url = format!("{}/games?success={:?}", self.hostname, success);
        let url =  Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))

    }

}
