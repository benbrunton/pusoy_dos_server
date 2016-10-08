use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use config::Config;

use logger;
use data_access::user::User as UserData;
use model::user::PartUser;



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

}

impl Handler for TestAuthController {

    fn handle(&self, _: &mut Request) -> IronResult<Response> { 

        logger::info("TestAuthController handler");

        let name = "Testy McTestface";
        let id = "12345678";

        let user = PartUser{
            name: String::from(name),
            provider_id: String::from(id),
            provider_type: String::from("test") 
        };

        self.user_data.create_if_new(user);

        self.success()

    }

}
