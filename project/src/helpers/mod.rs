use std::fmt::Display;

use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::mime::Mime;
use tera::TeraResult;

use util::session::Session;

pub fn get_user_id(req: &Request) -> Option<u64> {

    match req.extensions.get::<Session>() {
        Some(session) => session.user_id,
        _             => None
    }

}

pub fn redirect<S: Display>(hostname:&str, path:S) -> Response{

    let full_url = format!("{}/{}", hostname, path);
    let url =  Url::parse(&full_url).unwrap();

    Response::with((status::Found, modifiers::Redirect(url)))

}

pub fn render(result: TeraResult<String>) -> Response{

    let content_type = "text/html".parse::<Mime>().unwrap();
    Response::with((content_type, status::Ok, result.unwrap()))
}
