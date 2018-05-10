use tera::{Tera, Context, Result as TeraResult};
use gotham::router::builder::*;
use std::panic::RefUnwindSafe;

use controller::{Controller, ResponseType};

use config::Config;
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

    fn is_logged_in(&self, session: Option<Session>) -> bool {
        match session {
            Some(sess) => sess.user_id != None,
            None       => false
        }
    }

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
        self.tera.render("index.html", &data)
    }
}

impl Controller for HomePageController {
    fn get_response(
        &self,
        session:&mut Option<Session>
    ) -> ResponseType {
		let sess = session.clone();
        if self.is_logged_in(sess) {
            ResponseType::Redirect("/games".to_string())
        } else {
            let body = self.not_logged_in().unwrap();
            ResponseType::PageResponse(body)
        }
    }
}

impl RefUnwindSafe for HomePageController {}
