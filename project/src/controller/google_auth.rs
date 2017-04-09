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

pub struct GoogleAuthController {
    google_secret: String,
    google_app_id: String,
    hostname: String,
    user_data: UserData,
}

impl GoogleAuthController {
    pub fn new(config: &Config, user_data: UserData) -> GoogleAuthController {
        let google_secret = config.get("google_secret").unwrap();
        let google_app_id = config.get("google_app_id").unwrap();
        let hostname = config.get("hostname").unwrap();

        GoogleAuthController {
            google_secret: google_secret,
            google_app_id: google_app_id,
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

    fn get_access_token(&self, req: &mut Request) -> Result<bool, Result<Response, IronError>> {

        let google_secret = self.google_secret.clone();
        let client_id = self.google_app_id.clone();
        let hostname = self.hostname.clone();
        let redirect = format!("{}/auth", hostname);
        let auth_token = query::get(req.url.to_string(), "auth_token");

        if auth_token == None {
            warn!("invalid_query_param - no auth_token");
            return Err(self.invalid_query_param());
        }

        let auth_token = auth_token.unwrap();
        let google_token_url = format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}", auth_token);

        info!("requesting token from Google API /tokeninfo endpoint");

        let google_token_response = self.fetch_json(google_token_url);

        match google_token_response {
            Err(_) => {
                return Err(self.google_error());
            }
            _ => (),
        }

        info!("got access token");

        Ok(true)
    }

    fn extract_profile_info(&self, req: &Request) -> BTreeMap<String, String> {
        let mut profile = BTreeMap::new();
        profile.insert(String::from("id"), query::get(req.url.to_string(), "id").unwrap());
        profile.insert(String::from("name"), query::get(req.url.to_string(), "username").unwrap());

        return profile
    }

    fn success(&self) -> IronResult<Response> {

        let full_url = format!("{}/games", self.hostname);
        let url = Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))
    }

    fn google_error(&self) -> IronResult<Response> {

        Ok(Response::with((status::Ok, "there was an error with google auth")))
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

impl Handler for GoogleAuthController {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        info!("GoogleAuthController handler");

        let access_token_response = self.get_access_token(req);
        info!("{:?}", access_token_response);

        match access_token_response {
            Err(x) => return x,
            _ => (),
        }

        let access_token = access_token_response.unwrap();

        if !access_token {
            return self.google_error();
        }

        info!("loading profile");

        let profile = self.extract_profile_info(req);

        let id = profile.get("id").unwrap();
        let name = profile.get("name").unwrap();

        debug!("GOOGLE RESPONSE");
        debug!("{:?}", profile);

        info!("{}", id);
        info!("{}", name);

        let user = PartUser {
            name: name.clone(),
            provider_id: id.clone(),
            provider_type: String::from("google"),
        };

        let new_user = self.user_data.create_if_new(user);
        let session = self.get_new_session(req, new_user.id);
        req.extensions.insert::<Session>(session);

        self.success()
    }
}
