use std::panic::RefUnwindSafe;
use tera::{Tera, Context, Result as TeraResult};
use data_access::game::Game as GameData;
use helpers;
use helpers::{PathExtractor, QueryStringExtractor};
use controller::{Controller, ResponseType};
use model::Session;

pub struct CompleteGamesController {
    tera: &'static Tera,
    game_data: GameData
}

impl CompleteGamesController {
    pub fn new(tera:&'static Tera, game_data: GameData) -> CompleteGamesController {
        CompleteGamesController{ tera, game_data }
    }

    fn get_page(&self, id: u64) -> TeraResult<String> {

        let mut data = Context::new();
        let games = self.game_data.get_closed_games(id);

        data.add("games", &games);
        data.add("num_games", &games.len());
        data.add("logged_in", &true);
		data.add("id", &id);

        self.tera.render("complete-games.html", &data)
    }

}

impl Controller for CompleteGamesController {

    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        _path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            ResponseType::PageResponse(self.get_page(id).expect("unable to unwrap complete games page"))
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }

}

impl RefUnwindSafe for CompleteGamesController {}
