use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;

use tera::{Tera, Context};
use data_access::round::Round as RoundData;

pub struct InPlay{
    tera: &'static Tera,
    round_data: RoundData,
}

impl InPlay {
    
    pub fn new(tera:&'static Tera, round_data: RoundData) -> InPlay {
        InPlay{
            tera: tera,
            round_data: round_data
        }
    }

    pub fn display(&self) -> Response {

        let template = "inplay.html";
        let data = Context::new();
        let content_type = "text/html".parse::<Mime>().unwrap();
        let page = self.tera.render(template, data).unwrap();
        Response::with((content_type, status::Ok, page))
    }
}

impl Handler for InPlay {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Ok(self.display())
    }

}

