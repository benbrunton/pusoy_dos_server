use hyper::{Response, StatusCode};
use tera::{Tera, Context, TeraResult};
use mime;
use gotham::http::response::create_response;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham::middleware::session::{NewSessionMiddleware, SessionData};
use gotham::handler::{Handler, HandlerFuture};
use futures::{future, Future};

use config::Config;
use helpers;

#[derive(Copy, Clone)]
pub struct HomePageController <'a> {
    hostname: &'a str,
    tera: &'static Tera,
    fb_app_id: &'a str,
    google_app_id: &'a str,
    dev_mode: bool,
}

impl <'a> HomePageController <'a> {
    pub fn new(config: &Config, tera: &'static Tera) -> HomePageController <'a> {

        let hostname = &config.get("pd_host").unwrap();
        let fb_app_id = &config.get("fb_app_id").unwrap();
        let google_app_id = &config.get("google_app_id").unwrap();
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
/*
    fn logged_in(&self) -> IronResult<Response> {

        info!("user logged in - redirecting");
        let redirect = helpers::redirect(&self.hostname, "games");
        Ok(redirect)
    }

    fn not_logged_in(&self) -> IronResult<Response> {
        info!("user not logged in");
        let content_type = "text/html".parse::<Mime>().unwrap();
        let homepage = self.get_homepage().unwrap();
        Ok(Response::with((content_type, status::Ok, homepage)))
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
*/
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

impl <'a> Handler for HomePageController <'a> {

    fn handle(self, mut state: State) -> Box<HandlerFuture> {
/*		let maybe_session = {
			let session_data: &Option<Session> = SessionData::<Option<Session>>::borrow_from(&state);
			session_data.clone()
		};
*/

/*
		let body = match &maybe_session {
			&Some(ref session_data) => "Logged in".to_owned(),
			&None => "Not Logged in".to_owned(),
		};
*/
let body = "you".to_owned();

        let res = {
            create_response(
                &state,
                StatusCode::Ok,
                Some((
					body.as_bytes()
                        .to_vec(),
                    mime::TEXT_PLAIN,
                )),
            )
        };

        Box::new(future::ok((state, res)))
    }
}
