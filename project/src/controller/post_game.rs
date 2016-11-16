use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context};

pub struct PostGame{
    tera: &'static Tera,
}

impl PostGame{
    pub fn new(tera: &'static Tera) -> PostGame{
        
        PostGame{
            tera: tera
        }
    }

    pub fn display(&self) -> Response {
        let content_type = "text/html".parse::<Mime>().unwrap();

        let mut data = Context::new();
        let template = "post_game.html";
        let page = self.tera.render(template, data).expect("error rendering template");
        Response::with((content_type, status::Ok, page))

    }
}

impl Handler for PostGame {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Ok(self.display())
    }
}
 
