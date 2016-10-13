use std::fs;
use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use iron::mime::Mime;

use util::session::Session;
use config::Config;
use logger;

pub struct HomePageController {
    hostname: String
}

impl HomePageController {
    
    pub fn new(config: &Config) -> HomePageController {

        let hostname = config.get("hostname").unwrap();
        HomePageController {
            hostname: hostname
        }
    }

    fn logged_in(&self) -> IronResult<Response> {

        logger::info("user logged in - redirecting");

        let full_url = format!("{}/games", self.hostname);
        let url =  Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))
    }

    fn not_logged_in(&self) -> IronResult<Response> {
        Ok(Response::with((get_content_type(), status::Ok, get_homepage())))
    }

}


impl Handler for HomePageController {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session = req.extensions.get::<Session>().unwrap();
        match session.user_id {
            Some(_) => self.logged_in(),
            _ => self.not_logged_in()        
        }
    }
}

fn get_homepage() -> fs::File {
    fs::File::open("templates/index.html").ok().unwrap()
}

fn get_content_type() -> Mime {
    "text/html".parse::<Mime>().unwrap()
}
