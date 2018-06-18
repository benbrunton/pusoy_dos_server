extern crate chrono;
extern crate time;
extern crate toml;
extern crate pusoy_dos;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate futures;
extern crate tokio_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate schedule_recv;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate mysql;
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate csrf;
extern crate data_encoding;
extern crate url;

mod server;
mod router;
mod controller;
mod model;
mod config;
mod schedule;
mod data_access;
mod helpers;
mod util;
mod handlers;
mod middleware;

use tera::Tera;
use config::Config;

lazy_static!{
    static ref TERA: Tera = Tera::new("templates/**/*").expect("parsing error with templates");
}

pub fn main() {
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

    let config_stat_endpoint = &config.get("stat_endpoint");
    let stat_endpoint = config_stat_endpoint.to_owned().unwrap();
    let game_data = data_access::game::Game::new(pool.clone(), String::from(stat_endpoint));
    let round_data = data_access::round::Round::new(pool.clone());
    let event_data = data_access::event::Event::new(pool.clone());
    let user_data = data_access::user::User::new(pool.clone());

    let port_option = config.get("port");
    let port = port_option.expect("failed to get port").parse::<u16>()
        .expect("failed to unwrap port");

    schedule::run(game_data.clone(), event_data.clone(), round_data.clone());
    server::run(
        port,
        &config,
        &TERA,
        user_data,
        game_data.clone(),
        round_data.clone(),
        event_data.clone()
    );
}
