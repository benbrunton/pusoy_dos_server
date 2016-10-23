use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context, TeraResult};
use router::Router;

use util::session::Session;
use config::Config;

pub struct Game {
    tera: &'static Tera,
    hostname: String
}

impl <'a> Game {
    pub fn new(config: &Config, tera:&'static Tera) -> Game {
        let hostname = config.get("hostname").unwrap();
        Game{ tera: tera, hostname: hostname }
    }

    fn get_page_response(&self) -> Response {
        let content_type = "text/html".parse::<Mime>().unwrap();
        let mut data = Context::new(); 
        let page = self.tera.render("game.html", data);
        Response::with((content_type, status::Ok, page.unwrap()))
    }
}

impl Handler for Game {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let resp = match *query {
            Some(id) => self.get_page_response(),
            _ => {
                let full_url = format!("{}/games", self.hostname);
                let url =  Url::parse(&full_url).unwrap();

                Response::with((status::Found, modifiers::Redirect(url)))
            }
        };

        Ok(resp)

    }

}
