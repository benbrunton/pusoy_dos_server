use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use config::Config;

use logger;
use data_access::user::User as UserData;
use model::user::PartUser;
use util::session::Session;

pub struct TestAuthController {
    hostname: String,
    user_data: UserData
}

impl TestAuthController {
    
    pub fn new(config: &Config, user_data: UserData) -> TestAuthController {

        let hostname = config.get("hostname").unwrap();

        TestAuthController{
            hostname: hostname,
            user_data: user_data
        }
    }

    fn success(&self) -> IronResult<Response> {

        let full_url = format!("{}/games", self.hostname);
        let url =  Url::parse(&full_url).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))
    }

    fn get_new_session(&self, req: &mut Request, user_id:u64) -> Session{
        let session = req.extensions.get::<Session>().unwrap();
        let new_session = session.set_user(user_id);
        new_session
    }


}

impl Handler for TestAuthController {

    fn handle(&self, req: &mut Request) -> IronResult<Response> { 

        logger::info("TestAuthController handler");

        let name = "Testy McTestface";
        let id = "12345678";

        let user = PartUser{
            name: String::from(name),
            provider_id: String::from(id),
            provider_type: String::from("test") 
        };

        let new_user = self.user_data.create_if_new(user);
        let session = self.get_new_session(req, new_user.id);
        req.extensions.insert::<Session>(session);

        self.success()

    }

}
