extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");

    Iron::new(router).http("0.0.0.0:3000").unwrap();

}

fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "pusoy dos")))
}
