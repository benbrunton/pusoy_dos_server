use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context, TeraResult};

use config::Config;
use helpers;

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
            hostname: hostname,
            tera: tera,
            fb_app_id: fb_app_id,
            google_app_id: google_app_id,
            dev_mode: dev_mode,
        }
    }

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
}


impl Handler for HomePageController {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session_user_id = helpers::get_user_id(req);

        match session_user_id {
            Some(_) => self.logged_in(),
            _ => self.not_logged_in(),        
        }
    }
}
