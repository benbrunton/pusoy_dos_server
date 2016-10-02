use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;

use logger;


pub struct GameList;

impl Handler for GameList {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        Ok(Response::with((status::Ok, "game list")))
    }

}
