extern crate iron;
extern crate router;
extern crate toml;
extern crate hyper;
extern crate url;
extern crate rustc_serialize;
extern crate logger as iron_logger;
#[macro_use] extern crate mysql;
extern crate cookie;
extern crate uuid;
extern crate tera;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate urlencoded;

#[macro_use] extern crate pusoy_dos;

mod config;
mod util;
mod controller;
mod query;
mod model;
mod data_access;
mod helpers;

use controller::{
        home_page, 
        auth, 
        game, 
        game_list, 
        game_create, 
        new_game, 
        test_auth, 
        logout, 
        game_join,
        begin_game,
        inplay,
        game_move
    };
use config::Config;
use util::session::SessionMiddleware;

use iron::prelude::*;
use router::Router;
use iron_logger::Logger;
use tera::Tera;

lazy_static!{

    static ref TERA: Tera = Tera::new("templates/**/*");
}


fn main() {
    
    env_logger::init().unwrap();

    let pool = mysql::Pool::new("mysql://root@localhost").unwrap();
    let user_data = data_access::user::User::new(pool.clone());
    let session_store = data_access::session::Session::new(pool.clone());
    let game_data = data_access::game::Game::new(pool.clone());
    let round_data = data_access::round::Round::new(pool.clone());

    let mut router = Router::new();
    let config = Config::new();

    let auth_controller = auth::AuthController::new(&config, user_data.clone());
    let home_page_controller = home_page::HomePageController::new(&config, &TERA);
    let game_list_controller = game_list::GameList::new(&config, &TERA, game_data.clone());
    let logout_controller = logout::LogoutController::new(&config);
    let game_create_controller = game_create::GameCreate::new(&config, game_data.clone());
    let new_game_controller = new_game::NewGame::new(&TERA);
    let game_controller = game::Game::new(&config, &TERA, game_data.clone(), user_data.clone());
    let game_join = game_join::GameJoin::new(&config, game_data.clone());
    let begin_game = begin_game::BeginGame::new(&config, game_data.clone(), round_data.clone());
    let inplay_controller = inplay::InPlay::new(&config, &TERA, round_data.clone());
    let move_controller = game_move::GameMove::new(&config, round_data.clone());

    router.get("/", home_page_controller, "index");
    router.get("/auth", auth_controller, "auth_callback");
    router.get("/games", game_list_controller, "game_list");
    router.get("/logout", logout_controller, "log_out");
    router.get("/new-game", new_game_controller, "new_game");
    router.post("/new-game", game_create_controller, "game_create");
    router.get("/game/:id", game_controller, "game");
    router.post("/game/:id/join", game_join, "game_join");
    router.post("/game/:id/begin", begin_game, "begin_game");
    router.get("/play/:id", inplay_controller, "inplay");
    router.post("/play/:id", move_controller, "move");
 
    dev_mode(&config, &mut router, user_data.clone());

    let mut chain = Chain::new(router);

    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);


    let session = SessionMiddleware::new(session_store);
    chain.link_before(session.clone());
	chain.link_after(session.clone());

    chain.link_after(logger_after);
    // todo - a little error checking around this
    // will save a little debugging
    Iron::new(chain).http("0.0.0.0:3000").unwrap();

}

// all bits and pieces to do with dev mode can go in here
fn dev_mode(config: &Config, router: &mut Router, user_data: data_access::user::User){

    warn!("DEV MODE ENABLED");
    let test_auth_controller = test_auth::TestAuthController::new(config, user_data.clone());
    router.get("/test_auth", test_auth_controller, "test_auth");
}

