use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context};
use router::Router;

use config::Config;
use data_access::game::Game as GameData;
use model::game::Game as GameModel;
use data_access::user::User as UserData;
use model::user::User as UserModel;
use helpers;

#[derive (PartialEq)]
enum GameState {
    PregameOwner,
    PregameNotJoined,
    PregameJoined,
    InGame,
    NoGame
}

pub struct Game {
    tera: &'static Tera,
    hostname: String,
    game_data: GameData,
    user_data: UserData
}

impl Game {
    pub fn new(config: &Config, tera:&'static Tera, game_data: GameData, user_data:UserData) -> Game {
        let hostname = config.get("pd_host").unwrap();
        Game{ tera: tera, hostname: hostname, game_data: game_data, user_data: user_data }
    }

    fn get_page_response(&self, user: u64, id:u64) -> Response {

        // game states
        // 1. pre game - game owner  - ( awaiting users / ready to play ) + start / delete
        // 2. pre game - not joined  - game info + join
        // 3. pre game - joined      - game info + leave game
        // 4. in game  - redirect to /play/:id

        let game = self.game_data.get_game(id);
        let users = self.user_data.get_users_by_game(id);

        let game_state = self.determine_state(user, &game, &users);

        if game_state == GameState::InGame {
            return helpers::redirect(&self.hostname, format!("play/{}", id));
        }

        let content_type = "text/html".parse::<Mime>().unwrap();
        let page = self.render_page(game_state, &game, user, users);
        Response::with((content_type, status::Ok, page))
    }

    fn render_page(&self, state: GameState, game: &Option<GameModel>, user_id: u64, users: Vec<UserModel>) -> String {
        let mut data = Context::new();

        data.add("logged_in", &true);
        data.add("current_user", &user_id);
        
        info!("rendering page for user {}", user_id);

        match *game {
            Some(ref game_model) => {
                info!("genuine game page being rendered");
                data.add("id", &game_model.id);
                data.add("decks", &game_model.decks);
                data.add("num_users", &users.len());
                data.add("users", &users);
                data.add("move_limit", &game_model.max_move_duration);
                data.add("owner_id", &game_model.creator_id);
            },
            None => ()
        };

        let template = match state {
            GameState::NoGame           => "no_game.html",
            GameState::PregameOwner     => "pregame_owned.html",
            GameState::PregameNotJoined => "pregame_not_joined.html",
            GameState::PregameJoined    => "pregame_joined.html",
            _                           => "game.html"
        };

        info!("using template {}", template);

        self.tera.render(template, data).unwrap()
    }

    fn determine_state(&self, user:u64, game: &Option<GameModel>, users: &Vec<UserModel>) -> GameState {
        
        match *game {
            Some(ref game) => {
                info!("game creator: {}", game.creator_id);
                info!("current user {}", user);
                if game.started {
                    GameState::InGame
                } else if game.creator_id == user {
                    GameState::PregameOwner
                } else if users.iter().any(|user_m| { user == user_m.id } ) {
                    GameState::PregameJoined
                } else {
                    GameState::PregameNotJoined
                }
            },
            None => GameState::NoGame
        }
        
    }
}

impl Handler for Game {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");
        info!("rendering game page for id: {:?}", query);

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "games");
        
        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => self.get_page_response(user_id, id.parse::<u64>().unwrap_or(0)),
                    _ => redirect_to_homepage
                }

            },
            _ => redirect_to_homepage
        };

        Ok(resp)

    }

}
