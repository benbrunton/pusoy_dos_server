use iron::prelude::*;
use iron::middleware::Handler;
use tera::{Tera, Context, TeraResult};

use data_access::game::Game as GameData;
use config::Config;
use helpers;

pub struct CompleteGames {
    tera: &'static Tera,
    hostname: String,
    game_data: GameData
}

impl CompleteGames {
    pub fn new(config: &Config, tera:&'static Tera, game_data: GameData) -> CompleteGames {
        let hostname = config.get("pd_host").unwrap();
        CompleteGames{ tera: tera, hostname: hostname, game_data: game_data }
    }

    fn get_page(&self, id: u64) -> TeraResult<String> {

        let mut data = Context::new();
        let games = self.game_data.get_closed_games(id);

        data.add("games", &games);
        data.add("num_games", &games.len());
        data.add("logged_in", &true);

        self.tera.render("complete-games.html", data)
    }

}

impl Handler for CompleteGames {

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
