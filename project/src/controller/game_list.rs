use iron::prelude::*;
use iron::{status};
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context, TeraResult};

use util::session::Session;
use data_access::game::Game as GameData;

pub struct GameList {
    tera: &'static Tera,
    game_data: GameData
}

impl <'a> GameList {
    pub fn new(tera:&'static Tera, game_data: GameData) -> GameList {
        GameList{ 
            tera: tera,
            game_data: game_data
        }
    }

    fn get_page(&self, id:Option<u64>) -> TeraResult<String> {
        let mut data = Context::new(); 
        let games = match id {
            Some(x) => self.game_data.get_valid_games(x),
            _       => vec!()
        };
        let num_games = games.len();
        let open_games = match id {
            Some(x) => self.game_data.get_open_games(x),
            _       => vec!()
        };
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


        let session_user_id = match req.extensions.get::<Session>() {
            Some(session) => session.user_id,
            _             => None
        };

        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, self.get_page(session_user_id).unwrap())))

    }

}
