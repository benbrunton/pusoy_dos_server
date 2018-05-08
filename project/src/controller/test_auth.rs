use hyper::{Response, StatusCode};
use hyper::header::Location;
use tera::{Tera, Context, TeraResult};
use mime;
use gotham::http::response::create_response;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham::middleware::session::{NewSessionMiddleware, SessionData};
use gotham::handler::{NewHandler, Handler, HandlerFuture};
use futures::{future, Future};

use config::Config;

use data_access::user::User as UserData;
use model::user::PartUser;
use model::Session;
use rand;

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

/*
impl Handler for TestAuthController {

    fn handle(&self, req: &mut Request) -> IronResult<Response> { 

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
        let session = self.get_new_session(req, new_user.id);
        req.extensions.insert::<Session>(session);

        self.success()

    }
}
*/

impl NewHandler for TestAuthController {
    type Instance = Self;

    fn new_handler(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl Handler for TestAuthController {

    fn handle(self, mut state: State) -> Box<HandlerFuture> {
		let maybe_session = {
			let session_data: &Option<Session> = SessionData::<Option<Session>>::borrow_from(&state);
			session_data.clone()
		};

        let (status, body, redirect) = self.get_response(maybe_session);

        let mut res = {
            create_response(
                &state,
                status,
                body
            )
        };

        match redirect {
            Some(uri) => {
                let mut headers = res.headers_mut();
                headers.set(Location::new(uri));
            },
            _ => ()
        }

        Box::new(future::ok((state, res)))
    }
}
