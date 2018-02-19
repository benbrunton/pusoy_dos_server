use iron::prelude::*;
use iron::middleware::Handler;
use iron::status;
use iron::mime::Mime;
use serde_json;

use data_access::game::Game as GameData;

#[derive(Clone)]
pub struct GameHistory {
    game_data: GameData
}

impl GameHistory {
    pub fn new(game_data: GameData) -> GameHistory {
        GameHistory {
            game_data
        }
    }

    fn output(&self) -> Response {

        let content_type = "application/json".parse::<Mime>().unwrap();
        let game_history = self.game_data.get_all_closed_games();

        Response::with((content_type, status::Ok, serde_json::to_string(&game_history).unwrap()))
   
    }

}

impl Handler for GameHistory {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let resp = self.output();
        Ok(resp)
    }
}
