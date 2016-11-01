use iron::prelude::*;
use iron::middleware::Handler;
use tera::{Tera, Context, TeraResult};

use data_access::game::Game as GameData;
use config::Config;
use helpers;

pub struct GameList {
    tera: &'static Tera,
    game_data: GameData,
    hostname: String
}

impl GameList {
    pub fn new(config: &Config, tera:&'static Tera, game_data: GameData) -> GameList {

        let hostname = config.get("hostname").unwrap();

        GameList{ 
            tera: tera,
            game_data: game_data,
            hostname: hostname
        }
    }

    fn get_page(&self, id:u64) -> TeraResult<String> {
        let mut data = Context::new(); 
        let games = self.game_data.get_valid_games(id);
        let num_games = games.len();
        let open_games = self.game_data.get_open_games(id);
        let num_open_games = open_games.len();

        data.add("games", &games);
        data.add("num_games", &num_games);
        data.add("open_games", &open_games);
        data.add("num_open_games", &num_open_games);

        self.tera.render("game_list.html", data)
    }
}

impl Handler for GameList {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(id) => helpers::render(self.get_page(id)),
            _        => redirect_to_homepage
        };

        Ok(resp)

    }

}
