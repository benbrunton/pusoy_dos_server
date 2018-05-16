use tera::{Tera, Context, Result as TeraResult};
use std::cmp::Ordering;
use std::panic::RefUnwindSafe;

use helpers::PathExtractor;
use gotham::state::State;
use config::Config;
use data_access::game::Game as GameData;
use model::game::Game as GameModel;
use data_access::user::User as UserData;
use model::user::User as UserModel;
use model::Session;
use helpers;
use controller::{Controller, ResponseType};

#[derive (PartialEq)]
enum GameState {
    PregameOwner,
    PregameNotJoined,
    PregameJoined,
    InGame,
    NoGame
}

pub struct GameController {
    tera: &'static Tera,
    hostname: String,
    game_data: GameData,
    user_data: UserData
}

impl GameController {
    pub fn new(
        config: &Config,
        tera:&'static Tera,
        game_data: GameData,
        user_data:UserData
    ) -> GameController {
        let hostname = config.get("pd_host").unwrap();
        GameController{ tera: tera, hostname: hostname, game_data: game_data, user_data: user_data }
    }

    fn get_page(&self, user: u64, id:u64) -> ResponseType {

        // game states
        // 1. pre game - game owner  - ( awaiting users / ready to play ) + start / delete
        // 2. pre game - not joined  - game info + join
        // 3. pre game - joined      - game info + leave game
        // 4. in game  - redirect to /play/:id

        let game = self.game_data.get_game(id);
        let users = self.user_data.get_users_by_game(id);

        let game_state = self.determine_state(user, &game, &users);

        if game_state == GameState::InGame {
            return ResponseType::Redirect(format!("play/{}", id));
        }

        let body = self.render_page(game_state, &game, user, users)
            .expect("unable to unwrap game page");

        ResponseType::PageResponse(body)
    }

    fn render_page(
        &self,
        state: GameState,
        game: &Option<GameModel>,
        user_id: u64,
        users: Vec<UserModel>
    ) -> TeraResult<String> {
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

        self.tera.render(template, &data)
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

impl Controller for GameController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        path: Option<PathExtractor>
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            let path_id = path.expect("no_path").id as u64;
            self.get_page(id, path_id)
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }
}

impl RefUnwindSafe for GameController {}
