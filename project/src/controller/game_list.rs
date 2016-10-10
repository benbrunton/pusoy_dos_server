use iron::prelude::*;
use iron::{status};
use iron::middleware::Handler;

use logger;
use util::session::Session;

pub struct GameList;

impl Handler for GameList {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        logger::info(format!("{:?}", req.extensions.get::<Session>())); 
        Ok(Response::with((status::Ok, "game list")))
    }

}
