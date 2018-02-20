use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use tera::{Tera, Context};
use config::Config;
use router::Router;
use helpers;

use data_access::event::Event as EventData;


pub struct PostGame{
    tera: &'static Tera,
    hostname: String,
    event_data: EventData
}

impl PostGame{

    pub fn new(config: &Config, tera: &'static Tera, event_data: EventData) -> PostGame{
        
        let hostname = config.get("pd_host").unwrap();

        PostGame{
            tera,
            hostname,
            event_data
        }
    }

    pub fn display(&self, user_id: u64, game_id: u64) -> Response {
        let content_type = "text/html".parse::<Mime>().unwrap();

        let mut events_vec = self.event_data.get_game_events(game_id);
        let mut events = events_vec.as_mut_slice();
        events.reverse();

        let mut data = Context::new();
        data.add("logged_in", &true);
        data.add("events", &events);
        println!("events {:?}", events);
        let template = "post_game.html";
        let page = self.tera.render(template, data).expect("error rendering template");
        Response::with((content_type, status::Ok, page))

    }
}

impl Handler for PostGame {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.display(user_id, id.parse::<u64>().unwrap())
                    },
                    _ => redirect_to_homepage
                }
            },
            _ => redirect_to_homepage
        };

        Ok(resp)


    }
}
 
