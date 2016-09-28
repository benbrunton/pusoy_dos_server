extern crate iron;
extern crate router;
extern crate toml;
extern crate hyper;
extern crate url;

mod config;
mod controller;

use controller::{home_page, auth};

use iron::prelude::*;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", home_page::handler, "index");
    router.get("/test", home_page::test_handler, "test");

    router.get("/auth", auth::callback, "auth_callback");

    Iron::new(router).http("0.0.0.0:3000").unwrap();

}

