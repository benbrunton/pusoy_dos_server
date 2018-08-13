use tera::{Tera, Context, Result as TeraResult};
use std::panic::RefUnwindSafe;
use controller::{Controller, ResponseType};
use config::Config;
use model::Session;
use helpers::{PathExtractor, QueryStringExtractor};

#[derive(Clone)]
pub struct AboutController {
    tera: &'static Tera
}


impl AboutController {
    pub fn new(tera: &'static Tera) -> AboutController {

        AboutController {
            tera,
        }
    }

    fn is_logged_in(&self, session: Option<Session>) -> bool {
        match session {
            Some(sess) => sess.user_id != None,
            None       => false
        }
    }

    fn get_page(&self, logged_in: bool) -> TeraResult<String> {
        let mut data = Context::new(); 
        data.add("logged_in", &logged_in);
        self.tera.render("about.html", &data)
    }

}

impl Controller for AboutController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        _path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
		let sess = session.clone();
        let body = self.get_page(self.is_logged_in(sess)).unwrap();
        ResponseType::PageResponse(body)
    }
}

impl RefUnwindSafe for AboutController {}
