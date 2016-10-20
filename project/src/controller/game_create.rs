use iron::prelude::*;
use iron::{status};
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context, TeraResult};

use logger;
use util::session::Session;

pub struct GameCreate {
    tera: &'static Tera
}

impl <'a> GameCreate {
    pub fn new(tera:&'static Tera) -> GameCreate {
        GameCreate{ tera: tera }
    }

    fn get_page(&self) -> TeraResult<String> {
        let mut data = Context::new(); 
        self.tera.render("create_success.html", data)
    }
}

impl Handler for GameCreate {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        logger::info(format!("{:?}", req.extensions.get::<Session>())); 
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, self.get_page().unwrap())))
    }

}
