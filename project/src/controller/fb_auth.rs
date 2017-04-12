use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use hyper::client::Client;
use config::Config;
use std::io::Read;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

use query;
use data_access::user::User as UserData;
use model::user::PartUser;
use util::session::Session;

pub struct FacebookAuthController {
    fb_secret: String,
    fb_app_id: String,
    hostname: String,
    user_data: UserData,
}

impl FacebookAuthController {
    pub fn new(config: &Config, user_data: UserData) -> FacebookAuthController {
        let fb_secret = config.get("fb_secret").unwrap();
        let fb_app_id = config.get("fb_app_id").unwrap();
        let hostname = config.get("hostname").unwrap();

        FacebookAuthController {
            fb_secret: fb_secret,
            fb_app_id: fb_app_id,
            hostname: hostname,
            user_data: user_data,
        }
    }

    fn fetch_json(&self, url: String) -> Result<BTreeMap<String, Json>, String> {

        let client = Client::new();

        info!("requesting json from : {:?}", url);

        let res = client.get(&url).send();

        match res {
            Err(e) => {
                let err = format!("Error requesting json: {:?}", e);
                warn!("{}", &err);
                return Err(err.clone());
            }
            _ => (),
        }

        let mut r = res.unwrap();
        let mut buffer = String::new();
        let _ = r.read_to_string(&mut buffer);

        let data = Json::from_str(&buffer).unwrap();

        Ok(data.as_object().unwrap().clone())

    }

    fn get_access_token(&self, req: &mut Request) -> Result<String, Result<Response, IronError>> {

        let fb_secret = self.fb_secret.clone();
        let client_id = self.fb_app_id.clone();
        let hostname = self.hostname.clone();
        let redirect = format!("{}/auth", hostname);
        let code = query::get(req.url.to_string(), "code");

        if code == None {
            warn!("invalid_query_param - no code");
            return Err(self.invalid_query_param());
        }

        let code = code.unwrap();

        let fb_token_url = format!("https://graph.facebook.com/v2.7/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}",
                                   client_id,
                                   redirect,
                                   fb_secret,
                                   code);

        info!("requesting token from Facebook");

        let fb_token = self.fetch_json(fb_token_url);

        match fb_token {
            Err(_) => {
                return Err(self.facebook_error());
            }
            _ => (),
        }

        let fb_t = fb_token.unwrap();
        let access_token = fb_t.get("access_token")
            .unwrap()
            .as_string()
            .unwrap();

        info!("got access token");

        Ok(String::from(access_token))
    }

    fn get_profile(&self,
                   access_token: String)
                   -> Result<BTreeMap<String, Json>, Result<Response, IronError>> {


        let profile_url = format!("https://graph.facebook.com/v2.7/me?access_token={}&fields=id,\
                                   name,email",
                                  access_token);

        let profile_response = self.fetch_json(profile_url);

        match profile_response {
            Err(_) => {
                return Err(self.facebook_error());
            }
            _ => (),
        }

        Ok(profile_response.unwrap())

    }

    fn success(&self) -> IronResult<Response> {

        let full_url = format!("{}/games", self.hostname);
        let url = Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))
    }

    fn facebook_error(&self) -> IronResult<Response> {

        Ok(Response::with((status::Ok, "there was an error with facebook")))
    }

    fn invalid_query_param(&self) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "not ok")))
    }

    fn get_new_session(&self, req: &mut Request, user_id: u64) -> Session {
        let session = req.extensions.get::<Session>().unwrap();
        let new_session = session.set_user(user_id);
        new_session
    }
}

impl Handler for FacebookAuthController {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        info!("FacebookAuthController handler");

        let access_token_response = self.get_access_token(req);
        info!("{:?}", access_token_response);

        match access_token_response {
            Err(x) => return x,
            _ => (),
        }

        let access_token = access_token_response.unwrap();

        info!("loading profile");

        let profile_response = self.get_profile(access_token);

        match profile_response {
            Err(x) => return x,
            _ => (),    
        }

        let profile = profile_response.unwrap();

        let id = profile.get("id").unwrap().as_string().unwrap();
        let name = profile.get("name").unwrap().as_string().unwrap();

        debug!("FACEBOOK RESPONSE");
        debug!("{:?}", profile);

        info!("{}", id);
        info!("{}", name);

        let user = PartUser {
            name: String::from(name),
            provider_id: String::from(id),
            provider_type: String::from("facebook"),
        };

        let new_user = self.user_data.create_if_new(user);
        let session = self.get_new_session(req, new_user.id);
        req.extensions.insert::<Session>(session);

        self.success()
    }
}
