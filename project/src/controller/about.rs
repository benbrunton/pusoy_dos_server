use iron::prelude::*;
use iron::{status};
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context, TeraResult};

use util::session::Session;
use helpers;

pub struct About {
    tera: &'static Tera
}

impl About {
    pub fn new(tera:&'static Tera) -> About {
        About{ tera: tera }
    }

    fn get_page(&self, logged_in: bool) -> TeraResult<String> {
        let mut data = Context::new(); 
        data.add("logged_in", &logged_in);
        self.tera.render("about.html", data)
    }
}

impl Handler for About {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let session_user_id = helpers::get_user_id(req);

        let logged_in = match session_user_id {
            Some(_) => true,
            _ => false
        };

        info!("{:?}", req.extensions.get::<Session>());
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, self.get_page(logged_in).unwrap())))
    }

}
