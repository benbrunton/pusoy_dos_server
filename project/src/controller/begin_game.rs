use iron::prelude::*;
use iron::middleware::Handler;

use data_access::game::Game as GameData;
use config::Config;
use helpers;

pub struct BeginGame{
    game_data: GameData,
    hostname: String
}

impl BeginGame{
    pub fn new(config: &Config, game_data: GameData) -> BeginGame {
        let hostname = config.get("hostname").unwrap();
        BeginGame{
            game_data: game_data,
            hostname: hostname
        }
    }

    fn begin_game(&self) {

        // create a new game
        // set move to first move
    }
}

impl Handler for BeginGame {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(id) => helpers::redirect(&self.hostname, format!("game/{}", id)),
            _        => redirect_to_homepage
        };

        Ok(resp)


    }

}
