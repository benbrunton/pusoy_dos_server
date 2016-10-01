use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use hyper::client::Client;
use config::Config;
use std::io::Read;
use rustc_serialize::json::Json;

use query;
use logger;

pub struct AuthController{
    fb_secret: String,
    fb_app_id: String
}

impl AuthController {
    
    pub fn new(config: Config) -> AuthController {
        let fb_secret = config.get("fb_secret").unwrap();
        let fb_app_id = config.get("fb_app_id").unwrap();

        AuthController{
            fb_secret: fb_secret,
            fb_app_id: fb_app_id
        }
    }

    fn get_access_token(&self) -> Result<String, String> {

        let fb_app_access_token_url = format!("https://graph.facebook.com/v2.7//oauth/access_token?client_id={}&client_secret={}&grant_type=client_credentials", self.fb_app_id, self.fb_secret);
        
        let client = Client::new();
        let res = client.get(&fb_app_access_token_url).send();

        let response = match res {
            Ok(mut r) => {
                let mut buffer = String::new();
                let _ = r.read_to_string(&mut buffer);
                Ok(buffer)
            },
            Err(e) => {
                let err = format!("{:?}", e);
                logger::warn(
                    &err
                );
                Err(err)
            }
        };

        response
    }

    fn success(&self) -> IronResult<Response> {

        Ok(Response::with((status::Ok, "ok")))
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
        let redirect = "http://localhost:3000/auth";

        let code = query::get(req.url.to_string(), "code");

        if code == None {
            logger::warn("invalid_query_param - no code");
            return self.invalid_query_param();
        }

        let code = code.unwrap();

        let app_access_token = self.get_access_token();

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
        logger::info(
            format!("response from fb access token request: {}", buffer)
        );

        let data = Json::from_str(&buffer).unwrap();
        let obj = data.as_object().unwrap();

        let access_token = obj.get("access_token").unwrap();

        logger::info(format!("got access token: {} ", access_token));

        self.success()

    }

}

