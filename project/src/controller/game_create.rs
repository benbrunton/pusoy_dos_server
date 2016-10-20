use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;

use logger;
use config::Config;

pub struct GameCreate {
    hostname: String
}

impl GameCreate {
    pub fn new(config:&Config) -> GameCreate {
        let hostname = config.get("hostname").unwrap();
        GameCreate{ hostname: hostname }
    }
}

impl Handler for GameCreate {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let full_url = format!("{}/games?success=true", self.hostname);
        let url =  Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))

    }

}
