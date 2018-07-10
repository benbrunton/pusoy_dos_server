use std::panic::RefUnwindSafe;
use controller::{Controller, ResponseType};
use tokio_core::reactor::Core;
use hyper::client::Client;
use hyper::{Method, Request};
use config::Config;
use std::io::Read;
use std::collections::BTreeMap;
use serde_json;
use futures::{Future, Stream};

use helpers::{PathExtractor, QueryStringExtractor};
use data_access::user::User as UserData;
use model::user::PartUser;
use model::Session;

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
        let hostname = config.get("pd_host").expect("no pd_host found");

        FacebookAuthController {
            fb_secret: fb_secret,
            fb_app_id: fb_app_id,
            hostname: hostname,
            user_data: user_data,
        }
    }

    fn fetch_json(&self, url: String) -> Result<BTreeMap<String, serde_json::Value>, String> {

        let core = Core::new().expect("unable to unwrap core");
        let client = Client::new(&core.handle());

        info!("requesting json from : {:?}", url);
        let parsed_url = url.parse().expect("unable to parse fb url");

        let res = client.get(parsed_url)
            .and_then(|res| {
				res.body().fold(Vec::new(), |mut v, chunk| {
					v.extend(&chunk[..]);
					future::ok::<_, ()>(v)
				}).and_then(|chunks| {
					let s = String::from_utf8(chunks).unwrap();
					future::ok::<_, ()>(s)
				})
            });


        let mut r = core.run(res).unwrap();
        let mut buffer = String::new();
        let _ = r.read_to_string(&mut buffer);

        let data = serde_json::from_str(&buffer).unwrap();

        Ok(data.as_object().unwrap().clone())

    }

    fn get_access_token(&self, qs: &str) -> Result<String, ()> {

        let fb_secret = self.fb_secret.clone();
        let client_id = self.fb_app_id.clone();
        let hostname = self.hostname.clone();
        let redirect = format!("{}/fb-auth", hostname);
        let code = qs;

        let fb_token_url = format!("https://graph.facebook.com/v2.7/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}",
                                   client_id,
                                   redirect,
                                   fb_secret,
                                   code);

        info!("requesting token from Facebook");

        let fb_token = self.fetch_json(fb_token_url);

        match fb_token {
            Err(_) => {
                return Err(());
            }
            _ => (),
        }

        let fb_t = fb_token.unwrap();
        let access_token = fb_t.get("access_token")
            .unwrap()
            .as_str()
            .unwrap();

        info!("got access token");

        Ok(String::from(access_token))
    }

    fn get_profile(&self,
                   access_token: String)
					-> Result<BTreeMap<String, serde_json::Value>, ()> {

        let profile_url = format!("https://graph.facebook.com/v2.7/me?access_token={}&fields=id,\
                                   name,email",
                                  access_token);

        let profile_response = self.fetch_json(profile_url);

        match profile_response {
            Err(_) => {
                return Err(());
            }
            _ => (),
        }

        Ok(profile_response.unwrap())

    }

    fn success(&self) -> ResponseType {

        let full_url = format!("{}/games", self.hostname);
        let url = Url::parse(&full_url).unwrap();

        ResponseType::Redirect(url)
    }

    fn update_session(&self, user_id: u64, session: &mut Option<Session>) {
        *session = Some(Session {
            user_id: Some(user_id as usize),
            csrf_token: None
        });
    }
}

impl Controller for FacebookAuthController {
    fn get_response(
        &self, 
        session: &mut Option<Session>,
        _body: Option<String>,
        _path: Option<PathExtractor>,
        qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        // todo - pass query param, not request
        let token = qs.unwrap().code;
        let access_token_response = self.get_access_token(&token);
        info!("{:?}", access_token_response);

        match access_token_response {
            Err(x) => return ResponseType::ServerError,
            _ => (),
        }

        let access_token = access_token_response.unwrap();

        info!("loading profile");

        let profile_response = self.get_profile(access_token);

        match profile_response {
            Err(x) => return ResponseType::ServerError,
            _ => (),    
        }

        let profile = profile_response.unwrap();

        let id = {
			let i = profile.get("id");
			serde_json::to_string(&i).unwrap()
		};

        let name = {
			let n = profile.get("name");
			serde_json::to_string(&n).unwrap()
		};

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
 
        self.update_session(new_user.id, session);
        self.success()
    }
}

impl RefUnwindSafe for FacebookAuthController {}
