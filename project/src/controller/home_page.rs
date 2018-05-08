use hyper::{Response, StatusCode};
use hyper::header::Location;
use tera::{Tera, Context, TeraResult};
use mime;
use gotham::http::response::create_response;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham::middleware::session::{NewSessionMiddleware, SessionData};
use gotham::handler::{NewHandler, Handler, HandlerFuture};
use futures::{future, Future};

use config::Config;
use std::io;
use model::Session;

#[derive(Clone)]
pub struct HomePageController {
    hostname: String,
    tera: &'static Tera,
    fb_app_id: String,
    google_app_id: String,
    dev_mode: bool,
}

impl HomePageController {
    pub fn new(config: &Config, tera: &'static Tera) -> HomePageController {

        let hostname = config.get("pd_host").unwrap();
        let fb_app_id = config.get("fb_app_id").unwrap();
        let google_app_id = config.get("google_app_id").unwrap();
        let dev_mode = match config.get("mode") {
            Some(mode) => mode == "dev",
            _ => false,
        };

        HomePageController {
            hostname,
            tera,
            fb_app_id,
            google_app_id,
            dev_mode,
        }
    }

    fn get_response(
        &self,
        session:Option<Session>
    ) -> (StatusCode, Option<(Vec<u8>, mime::Mime)>, Option<String>) {
        if(self.is_logged_in(session)){
            (StatusCode::Found, None, Some("/games".to_string()))
        } else {
            let body = self.not_logged_in().unwrap();
            (StatusCode::Ok,
            Some((
                body.as_bytes()
                    .to_vec(),
                mime::TEXT_HTML,
            )),
            None)

        }
    }

    fn is_logged_in(&self, session: Option<Session>) -> bool {
        match session {
            Some(sess) => sess.user_id != None,
            None       => false
        }
    }
/*
    fn logged_in(&self) -> IronResult<Response> {

        info!("user logged in - redirecting");
        let redirect = helpers::redirect(&self.hostname, "games");
        Ok(redirect)
    }

*/

    fn not_logged_in(&self) -> TeraResult<String> {
        info!("user not logged in");
        self.get_homepage()
    }

    fn get_homepage(&self) -> TeraResult<String> {
        let fb = format!("https://www.facebook.com/v2.\
                          7/dialog/oauth?client_id={}&redirect_uri={}/fb-auth",
                         self.fb_app_id,
                         self.hostname);
        let mut data = Context::new();
        data.add("fb_login", &fb);
        data.add("google_app_id", &self.google_app_id);
        data.add("dev_mode", &self.dev_mode);
        self.tera.render("index.html", data)
    }
}


/*
impl Handler for HomePageController {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session_user_id = helpers::get_user_id(req);

        match session_user_id {
            Some(_) => self.logged_in(),
            _ => self.not_logged_in(),        
        }
    }
}
*/

