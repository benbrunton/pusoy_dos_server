extern crate iron;
extern crate router;
extern crate toml;
extern crate hyper;
extern crate url;
extern crate rustc_serialize;
extern crate logger as iron_logger;
#[macro_use]
extern crate mysql;
extern crate cookie;
extern crate uuid;
extern crate tera;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate urlencoded;
extern crate staticfile;
extern crate mount;
extern crate bodyparser;
extern crate time;
extern crate rand;
extern crate chrono;
extern crate schedule_recv;
extern crate regex;

#[macro_use]
extern crate pusoy_dos;

mod config;
mod util;
mod controller;
mod query;
mod model;
mod data_access;
mod helpers;
mod api;

use controller::{home_page, fb_auth, google_auth, game, game_list, game_create, new_game, test_auth,
                 logout, game_join, begin_game, inplay, game_move, post_game, leaderboard,
                 remove_user, about, update_game};
use config::Config;
use util::session::SessionMiddleware;
use util::nocache::NoCacheMiddleware;
use util::move_limit_task;

use iron::prelude::*;
use router::Router;
use iron_logger::Logger;
use tera::Tera;
use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use std::path::Path;
use std::thread;
use staticfile::Static;
use mount::Mount;
use schedule_recv::periodic_ms;

lazy_static!{

    static ref TERA: Tera = Tera::new("templates/**/*");
}


fn main() {

    env_logger::init().expect("failed to init logger");

    let config = Config::new();

    let mysql_user = config.get("mysql_user").expect("no mysql user set");
    let mysql_pw = config.get("mysql_pw").expect("no mysql pw set");
    let mysql_host = config.get("mysql_host").expect("no mysql host set");
    let mysql_port = config.get("mysql_port").expect("no mysql port set");

    let mut builder = mysql::OptsBuilder::new();
    builder.ip_or_hostname(Some(mysql_host))
            .user(Some(mysql_user))
            .pass(Some(mysql_pw))
            .tcp_port(mysql_port.parse::<u16>().expect("failed to build mysql opts"));

    let pool_result = mysql::Pool::new(builder);
    
    match pool_result {
        Err(err) => {
            error!("{:?}", err);
            error!("exiting!");
            return;
        },
        _ => ()   
    };

    let pool = pool_result.expect("failed to unwrap mysql pool");

    let user_data = data_access::user::User::new(pool.clone());
    let session_store = data_access::session::Session::new(pool.clone());
    let game_data = data_access::game::Game::new(pool.clone());
    let round_data = data_access::round::Round::new(pool.clone());
    let leaderboard_data = data_access::leaderboard::Leaderboard::new(pool.clone());
    let event_data = data_access::event::Event::new(pool.clone());

    let notification_data = data_access::notification::Notification::new(pool.clone());

    let mut router = Router::new();

    let facebook_auth_controller = fb_auth::FacebookAuthController::new(&config, user_data.clone());
    let google_auth_controller = google_auth::GoogleAuthController::new(&config, user_data.clone());
    let home_page_controller = home_page::HomePageController::new(&config, &TERA);
    let game_list_controller = game_list::GameList::new(&config, &TERA, game_data.clone());
    let logout_controller = logout::LogoutController::new(&config);
    let game_create_controller = game_create::GameCreate::new(&config, game_data.clone());
    let new_game_controller = new_game::NewGame::new(&TERA);
    let about = about::About::new(&TERA);
    let game_controller = game::Game::new(&config, &TERA, game_data.clone(), user_data.clone());
    let game_join = game_join::GameJoin::new(&config, game_data.clone());
    let begin_game = begin_game::BeginGame::new(&config, game_data.clone(), round_data.clone());
    let inplay_controller =
        inplay::InPlay::new(&config, &TERA, round_data.clone(), user_data.clone());
    let move_controller = game_move::GameMove::new(&config, round_data.clone(), game_data.clone());
    let post_game_controller = post_game::PostGame::new(&TERA);
    let leaderboard = leaderboard::Leaderboard::new(&config, &TERA, leaderboard_data.clone());
    let remove_user = remove_user::RemoveUser::new(&config, game_data.clone());
    let update_game = update_game::UpdateGame::new(&config, game_data.clone());

    router.get("/", home_page_controller, "index");
    router.get("/fb-auth", facebook_auth_controller, "fb_auth_callback");
    router.get("/google-auth",
               google_auth_controller,
               "google_auth_callback");
    router.get("/games", game_list_controller, "game_list");
    router.get("/logout", logout_controller, "log_out");
    router.get("/new-game", new_game_controller, "new_game");
    router.post("/new-game", game_create_controller, "game_create");
    router.get("/game/:id", game_controller, "game");
    router.post("/game/:id/join", game_join, "game_join");
    router.post("/game/:id/begin", begin_game, "begin_game");
    router.get("/game-complete/:id", post_game_controller, "post_game");
    router.get("/play/:id", inplay_controller, "inplay");
    router.post("/play/:id", move_controller, "move");
    router.get("/leaderboard", leaderboard, "leaderboard");
    router.get("/about", about, "about");
    router.post("/game/:id/remove/:user", remove_user, "remove_user");
    router.post("/game/:id/update", update_game, "update_game");

    match config.get("mode") {
        Some(mode) => {
            if mode == "dev" {
                dev_mode(&config, &mut router, user_data.clone())
            }
        }
        _ => (),
    }

    let (logger_before, logger_after) = Logger::new(None);

    let api_router = api::router::new(round_data.clone(),
                                      user_data.clone(),
                                      game_data.clone(),
                                      event_data.clone(),
                                      notification_data.clone());

    let mut page_chain = Chain::new(router);
    let mut api_chain = Chain::new(api_router);

    let session = SessionMiddleware::new(session_store);
    let no_cache = NoCacheMiddleware;

    page_chain.link_before(session.clone());
    page_chain.link_after(session.clone());

    api_chain.link_before(session.clone());
    api_chain.link_after(no_cache);

    let mut mount = Mount::new();
    mount.mount("/", page_chain)
        .mount("/api/v1/", api_chain)
        .mount("/public/", Static::new(Path::new("public")))
        .mount("/sw.js", Static::new(Path::new("public/js/sw.js")));


    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);


    info!("setting up scheduled jobs..");
    let tick = periodic_ms(60000);

    let handle = thread::spawn(move || loop {
        tick.recv().expect("failed to receive tick period");

        move_limit_task::execute(game_data.clone(), event_data.clone(), round_data.clone());
    });


    info!("setting up server");
    let port = config.get("port");
    // todo - a little error checking around this
    // will save a fair amount of debugging
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let host = SocketAddr::new(
        ip, 
        port.expect("failed to get port").parse::<u16>().expect("failed to unwrap host")
    );
    Iron::new(chain).http(host).expect("failed to create Iron server");

}

// all bits and pieces to do with dev mode can go in here
fn dev_mode(config: &Config, router: &mut Router, user_data: data_access::user::User) {

    warn!("DEV MODE ENABLED");
    let test_auth_controller = test_auth::TestAuthController::new(config, user_data.clone());
    router.get("/test_auth", test_auth_controller, "test_auth");
}
