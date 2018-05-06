/// Web controller which manages Google OAuth login requests and token authentication.
use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use hyper::client::Client;
use config::Config;
use std::io::Read;
use std::collections::BTreeMap;
use time;
use regex::Regex;
use serde_json;

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
    /// Creates a new `GoogleAuthController` using `&Config` to retrieve app configuration
    /// and `user_data` to fetch user information.
    pub fn new(config: &Config, user_data: UserData) -> GoogleAuthController {
        let google_secret = config.get("google_secret").expect("no google secret found");
        let google_app_id = config.get("google_app_id").expect("no google app id found");
        let hostname = config.get("pd_host").expect("no hostname found");

        GoogleAuthController {
            google_secret: google_secret,
            google_app_id: google_app_id,
            hostname: hostname,
            user_data: user_data,
        }
    }

    /// Fetch a JSON document from `url` and return it as a key-value map, where value will
    /// typically be just a string
    fn fetch_json(&self, url: String) -> Result<BTreeMap<String, serde_json::Value>, String> {

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

        let data = serde_json::from_str(&buffer).unwrap();

        Ok(data.as_object().unwrap().clone())

    }

    /// Handle a Google OAuth sign-in request and retrieve the OAuth access token from Google Auth API and return it
    fn get_access_token(&self, req: &mut Request) -> Result<BTreeMap<String, Json>, Result<Response, IronError>> {

        // First stage of Google OAuth is performed client-side via a JavaScript API.
        // All left to do is sent the user's ID to Google API to obtain the session token.
        let hostname = self.hostname.clone();
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

        let g_t = google_token_response.unwrap();
        info!("got access token");

        Ok(g_t)
    }

    /// Verify response obtained from Google Auth API is integral and correct.
    fn validate_access_token(&self, token: BTreeMap<String, Json>) -> bool {

        let now = time::get_time();
        let iss = token.get("iss").unwrap().as_string().unwrap();
        let aud = token.get("aud").unwrap().as_string().unwrap();
        let kid = token.get("kid").unwrap().as_string().unwrap(); // Google's public key
        let expiry_time = token.get("exp").unwrap().as_string().unwrap().parse::<i64>().unwrap(); // Token's expiry time

        // Check verification was performed by Google
        let iss_re = Regex::new(r"(https://)?accounts.google.com").unwrap();
        if !iss_re.is_match(iss) {
            return false
        }

        // Check token hasn't expired
        if expiry_time < now.sec {
            return false
        }

        // Check response was crafted exclusively for our app
        if aud != self.google_app_id {
            return false
        }

        // TODO -- Add Google OAuth pubkey verification too

        true
    }

    /// Extract user information from OAuth profile and return it as a map
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
    /// Handle HTTP request to authenticate using a Google OAuth provider
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        info!("GoogleAuthController handler");

        // Retrieve token to authenticate user
        let access_token_response = self.get_access_token(req);
        info!("{:?}", access_token_response);

        match access_token_response {
            Err(x) => return x,
            _ => (),
        }

        let access_token = access_token_response.unwrap();

        info!("verifying authentication token");
        // Check session token received is OK
        if !self.validate_access_token(access_token) {
            return self.google_error()
        }

        info!("loading profile");

        // Load user's basic profile info
        let profile = self.extract_profile_info(req);

        let id = profile.get("id").unwrap();
        let name = profile.get("name").unwrap();

        debug!("GOOGLE RESPONSE");
        debug!("{:?}", profile);

        info!("{}", id);
        info!("{}", name);

        // Create a user model out of the information obtained and commit their details to the
        // database if not already there.
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
