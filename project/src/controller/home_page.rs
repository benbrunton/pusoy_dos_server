use std::fs;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

pub fn handler(_: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, get_homepage())))
}

pub fn test_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "this is a test")))
}

fn get_homepage() -> fs::File {
    fs::File::open("templates/index.html").ok().unwrap()
}
