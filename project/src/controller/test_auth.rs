use std::panic::RefUnwindSafe;

use controller::{Controller, ResponseType};

use config::Config;

use data_access::user::User as UserData;
use model::user::PartUser;
use model::Session;
use rand;

#[derive(Clone)]
pub struct TestAuthController {
    hostname: String,
    user_data: UserData
}

impl TestAuthController {
    
    pub fn new(config: &Config, user_data: UserData) -> TestAuthController {

        let hostname = config.get("pd_host").unwrap();

        TestAuthController{
            hostname: hostname,
            user_data: user_data
        }
    }

    fn success(&self) -> ResponseType {
        ResponseType::Redirect("/games".to_string())
    }

    fn create_user(&self) -> u64 {
        info!("TestAuthController handler");
        
        let unique_num = rand::random::<u8>();
        let name = format!("Testy McTestface_{}", unique_num);;
        let id = format!("1660{}", unique_num); // just rammed some random nums in here to prevent collisions

        let user = PartUser{
            name: String::from(name),
            provider_id: String::from(id),
            provider_type: String::from("test") 
        };

        let new_user = self.user_data.create_if_new(user);
        new_user.id
    }

    fn update_session(&self, user_id: u64, session: &mut Option<Session>) {
        *session = Some(Session {
            user_id: Some(user_id as usize)
        });
    }


}

impl Controller for TestAuthController {
    fn get_response(&self, session: &mut Option<Session>) -> ResponseType {
        let user_id = self.create_user();
        self.update_session(user_id, session);
        self.success()
    }
}

impl RefUnwindSafe for TestAuthController {}
