use iron::prelude::*;
use iron::status;
use hyper::client::Client;
use config::Config;
use iron::middleware::Handler;

use query;
use logger;

pub struct AuthController{
    config: Config
}

impl AuthController {
    
    pub fn new(config: Config) -> AuthController {
        AuthController{
            config: config   
        }
    }

    fn success(&self) -> IronResult<Response> {

        Ok(Response::with((status::Ok, "ok")))
    }

    fn facebook_error(&self) -> IronResult<Response>{

        Ok(Response::with((status::Ok, "ok")))
    }

    fn invalid_query_param(&self) -> IronResult<Response>{
        Ok(Response::with((status::Ok, "ok")))
    }

}

impl Handler for AuthController {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        
        logger::info("AuthController handler");

        let fb_secret = self.config.get("fb_secret").unwrap();
        let client_id = self.config.get("fb_app_id").unwrap();
        let redirect = "http://localhost:3000/auth";

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

        let response = match res {
            Ok(r) => {
                logger::info(
                    format!("{:?}", r)
                );
                self.success()
            },
            Err(e) => {
                logger::warn(
                    format!("{:?}", e)
                );
                self.facebook_error()
            }
        };

        response
    }

}

