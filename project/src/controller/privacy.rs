use tera::{Tera, Context, Result as TeraResult};
use std::panic::RefUnwindSafe;
use controller::{Controller, ResponseType};
use config::Config;
use model::Session;
use helpers::{PathExtractor, QueryStringExtractor};

#[derive(Clone)]
pub struct PrivacyController {
    tera: &'static Tera
}


impl PrivacyController {
    pub fn new(tera: &'static Tera) -> PrivacyController {

        PrivacyController {
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
        self.tera.render("privacy.html", &data)
    }

}

impl Controller for PrivacyController {
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

impl RefUnwindSafe for PrivacyController {}
