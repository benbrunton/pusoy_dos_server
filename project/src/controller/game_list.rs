use iron::prelude::*;
use iron::{status};
use iron::middleware::Handler;


pub struct GameList;

impl Handler for GameList {

    fn handle(&self, _: &mut Request) -> IronResult<Response> {

        Ok(Response::with((status::Ok, "game list")))
    }

}
