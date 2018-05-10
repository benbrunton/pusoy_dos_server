use tera::{Tera, Context, Result as TeraResult};
use std::cmp::Ordering;
use std::panic::RefUnwindSafe;

use data_access::game::Game as GameData;
use config::Config;
use helpers;
use controller::{Controller, ResponseType};
use model::Session;

pub struct GameListController {
    tera: &'static Tera,
    game_data: GameData,
    hostname: String
}

impl GameListController {
    pub fn new(config: &Config, tera:&'static Tera, game_data: GameData) -> GameListController {

        let hostname = config.get("pd_host").unwrap();

        GameListController{ 
            tera: tera,
            game_data: game_data,
            hostname: hostname
        }
    }

    fn get_page(&self, id:u64) -> TeraResult<String> {

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

        self.tera.render("game_list.html", &data)
    }
}

impl Controller for GameListController {

    fn get_response(&self, session:&mut Option<Session>) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            ResponseType::PageResponse(self.get_page(id).expect("unable to unwrap game list page"))
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }

}

impl RefUnwindSafe for GameListController {}
