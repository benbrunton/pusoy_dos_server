use iron::prelude::*;
use iron::middleware::Handler;
use router::Router;

use data_access::game::Game as GameData;
use data_access::round::Round as RoundData;
use config::Config;
use helpers;

use pusoy_dos::game::game::Game as CardGame;

pub struct BeginGame{
    game_data: GameData,
    round_data: RoundData,
    hostname: String
}

impl BeginGame{
    pub fn new(config: &Config, game_data: GameData, round_data: RoundData) -> BeginGame {
        let hostname = config.get("hostname").unwrap();
        BeginGame{
            game_data: game_data,
            round_data: round_data,
            hostname: hostname
        }
    }

    fn begin_game(&self, /*user*/_:u64, game_id:u64) {

        // todo - validate that this user can begin the game
        let _ = self.game_data.start_game(game_id);        
        let users = self.game_data.get_players(game_id);
        // create a new game
        let new_game = CardGame::setup(users).unwrap();
        self.round_data.create_round(game_id, new_game);

    }
}

impl Handler for BeginGame {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.begin_game(user_id, id.parse::<u64>().unwrap());
                        helpers::redirect(&self.hostname, format!("game/{}", id))
                    },
                    _ => redirect_to_homepage
                }

            },
            _ => redirect_to_homepage
        };



        Ok(resp)


    }

}
