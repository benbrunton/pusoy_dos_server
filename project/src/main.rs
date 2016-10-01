extern crate iron;
extern crate router;
extern crate toml;
extern crate hyper;
extern crate url;
extern crate rustc_serialize;

mod config;
mod controller;
mod query;
mod logger;

use controller::{home_page, auth};
use config::Config;

use iron::prelude::*;
use router::Router;


fn main() {
    let mut router = Router::new();
    let config = Config::new();
    let auth_controller = auth::AuthController::new(config);

    router.get("/", home_page::handler, "index");
    router.get("/test", home_page::test_handler, "test");

    router.get("/auth", auth_controller, "auth_callback");

    Iron::new(router).http("0.0.0.0:3000").unwrap();

}


