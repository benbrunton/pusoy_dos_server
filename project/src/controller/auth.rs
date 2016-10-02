use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use hyper::client::Client;
use config::Config;
use std::io::Read;
use rustc_serialize::json::Json;

use query;
use logger;

pub struct AuthController{
    fb_secret: String,
    fb_app_id: String,
    hostname: String
}

impl AuthController {
    
    pub fn new(config: Config) -> AuthController {
        let fb_secret = config.get("fb_secret").unwrap();
        let fb_app_id = config.get("fb_app_id").unwrap();
        let hostname = config.get("hostname").unwrap();

        AuthController{
            fb_secret: fb_secret,
            fb_app_id: fb_app_id,
            hostname: hostname
        }
    }

    fn success(&self) -> IronResult<Response> {

        let full_url = format!("{}/games", self.hostname);
        let url =  Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))
    }

    fn facebook_error(&self) -> IronResult<Response>{

        Ok(Response::with((status::Ok, "there was an error with facebook")))
    }

    fn invalid_query_param(&self) -> IronResult<Response>{
        Ok(Response::with((status::Ok, "not ok")))
    }

}

impl Handler for AuthController {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        
        logger::info("AuthController handler");

        let fb_secret = self.fb_secret.clone();
        let client_id = self.fb_app_id.clone();
        let hostname = self.hostname.clone();
        let redirect = format!("{}/auth", hostname);

        let code = query::get(req.url.to_string(), "code");

        if code == None {
            logger::warn("invalid_query_param - no code");
            return self.invalid_query_param();
        }

        let code = code.unwrap();

        let fb_token_url = format!("https://graph.facebook.com/v2.7/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}", client_id, redirect, fb_secret, code);


        logger::info("requesting token from Facebook");
        let client = Client::new();
        let res = client.get(&fb_token_url).send();

        match res {
            Err(e) => {
                logger::warn(
                    format!("{:?}", e)
                );
                return self.facebook_error();
            },
            _ => ()
        }

        let mut r = res.unwrap();

        let mut buffer = String::new();
        let _ = r.read_to_string(&mut buffer);

        let data = Json::from_str(&buffer).unwrap();
        let obj = data.as_object().unwrap();

        let access_token = obj.get("access_token").unwrap().as_string().unwrap();

        logger::info("got access token");
        logger::info("loading profile");
                

        let profile_url = format!("https://graph.facebook.com/v2.7/me?access_token={}&fields=id,name", access_token);

        let res = client.get(&profile_url).send();


        match res {
            Err(e) => {
                logger::warn(
                    format!("{:?}", e)
                );
                return self.facebook_error();
            },
            _ => ()
        }


        let mut r = res.unwrap();

        let mut buffer = String::new();
        let _ = r.read_to_string(&mut buffer);

        logger::info(buffer);


        self.success()

    }

}

