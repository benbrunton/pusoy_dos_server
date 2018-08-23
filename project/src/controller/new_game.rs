use tera::{Tera, Context, Result as TeraResult};
use model::Session;
use std::panic::RefUnwindSafe;
use csrf::{AesGcmCsrfProtection, CsrfProtection};

use helpers;
use helpers::{PathExtractor, QueryStringExtractor};
use controller::{Controller, ResponseType};

pub struct NewGameController {
    tera: &'static Tera
}

impl NewGameController {
    pub fn new(tera:&'static Tera) -> NewGameController {
        NewGameController{ tera }
    }

    fn get_page(&self, csrf_token: String) -> TeraResult<String> {
        let mut data = Context::new(); 
        data.add("logged_in", &true);
        data.add("csrf", &csrf_token);
        self.tera.render("game_create.html", &data)
    }
    
    fn update_session(&self, session: &mut Option<Session>, csrf_token: String) {
        let user_id = {
            let sess = session.clone().unwrap();
            sess.user_id
        };

        *session = Some(Session{
            user_id,
            csrf_token: Some(csrf_token)
        });
    }

}

impl Controller for NewGameController {

    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        _path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        
        if helpers::is_logged_in(session) {
            let protect = AesGcmCsrfProtection::from_key(*b"01234567012345670123456701234567");
            let (token, cookie) = protect.generate_token_pair(None, 300)
                .expect("couldn't generate token/cookie pair");
            let token_str = token.b64_string();
            let cookie_str = cookie.b64_string();
            self.update_session(session, cookie_str);
            ResponseType::PageResponse(self.get_page(token_str).expect("unable to unwrap new game page"))
        } else {
            ResponseType::Redirect("/".to_string())
        }
    }
}

impl RefUnwindSafe for NewGameController {}
