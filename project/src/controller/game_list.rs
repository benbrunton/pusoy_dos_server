use iron::prelude::*;
use iron::middleware::Handler;
use tera::{Tera, Context, TeraResult};
use std::cmp::Ordering;

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

        let hostname = config.get("pd_host").unwrap();

        GameList{ 
            tera: tera,
            game_data: game_data,
            hostname: hostname
        }
    }

    fn get_page(&self, id:u64) -> TeraResult<String> {

        // todo - base context that can be configured with
        // common attributes - e.g. logged_in
        let mut data = Context::new(); 
        let mut games = self.game_data.get_valid_games(id);
        let num_games = games.len();
        let open_games = self.game_data.get_open_games(id);
        let num_open_games = open_games.len();

        games.sort_by(|a, b| {
            if a.next_player_id.unwrap() == id && b.next_player_id.unwrap() != id {
                Ordering::Less
            } else if b.next_player_id.unwrap() == id && a.next_player_id.unwrap() != id {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        info!("{:?}", games);

        data.add("games", &games);
        data.add("num_games", &num_games);
        data.add("open_games", &open_games);
        data.add("num_open_games", &num_open_games);
		data.add("id", &id);
        data.add("logged_in", &true);

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
