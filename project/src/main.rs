extern crate iron;
extern crate router;
extern crate toml;
extern crate hyper;
extern crate url;
extern crate rustc_serialize;
extern crate logger as iron_logger;
#[macro_use]
extern crate mysql;

mod config;
mod controller;
mod query;
mod logger;

use controller::{home_page, auth, game_list};
use config::Config;

use iron::prelude::*;
use router::Router;
use iron_logger::Logger;


fn main() {
    
    let pool = mysql::Pool::new("mysql://root@localhost").unwrap();

    let mut router = Router::new();
    let config = Config::new();
    let auth_controller = auth::AuthController::new(config, pool);
    let game_list_controller = game_list::GameList;

    router.get("/", home_page::handler, "index");
    router.get("/test", home_page::test_handler, "test");

    router.get("/auth", auth_controller, "auth_callback");
    router.get("/games", game_list_controller, "game_list");

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(router);

    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("0.0.0.0:3000").unwrap();

}


