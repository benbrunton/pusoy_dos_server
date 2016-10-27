use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use router::Router;

use data_access::game::Game as GameData;
use model::game::Game as GameModel;
use util::session::Session;
use config::Config;


pub struct GameJoin{
    game_data: GameData,
    hostname: String
}

impl GameJoin {
    pub fn new(config: &Config, game_data: GameData) -> GameJoin {

        let hostname = config.get("hostname").unwrap();

        GameJoin{
            game_data: game_data,
            hostname: hostname
        }
    }

    fn get_page_response(&self, user: u64, game: u64) -> Response {

        // join game - if successful redirect to game page
        // else error?
        self.game_data.join_game(user, game);
        
        let game_url = format!("game/{}", game);
        self.redirect(&game_url)
    }

    fn redirect(&self, path: &str) -> Response {
        let full_url = format!("{}/{}", self.hostname, path);
        let url =  Url::parse(&full_url).unwrap();

        Response::with((status::Found, modifiers::Redirect(url)))

    }

}

impl Handler for GameJoin {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");
        info!("joining game {:?}", query);

        let session_user_id = match req.extensions.get::<Session>() {
            Some(session) => session.user_id,
            _             => None
        };

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => self.get_page_response(user_id, id.parse::<u64>().unwrap_or(0)),
                    _ => {
                        let full_url = format!("{}/games", self.hostname);
                        let url =  Url::parse(&full_url).unwrap();

                        Response::with((status::Found, modifiers::Redirect(url)))
                    }
                }

            },
            _ => {
                let full_url = format!("{}/games", self.hostname);
                let url =  Url::parse(&full_url).unwrap();

                Response::with((status::Found, modifiers::Redirect(url)))

            }
        };

        Ok(resp)

    }

}
