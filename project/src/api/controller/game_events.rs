use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;
use std::collections::BTreeMap;

use data_access::event::Event as EventData;

use helpers;
use serde_json;

#[derive(Clone)]
pub struct GameEvents{
    event_data: EventData
}

impl GameEvents {
    pub fn new(event_data: EventData) -> GameEvents {
        GameEvents {
            event_data: event_data
        }
    }

    fn output(&self, _:u64, game_id: u64) -> Response {
        let content_type = "application/json".parse::<Mime>().unwrap();
        let event_data = self.event_data.get_game_events(game_id);

        let displayed_events: Vec<BTreeMap<String, String>> = event_data.iter().map(|ev| {
            ev.display()
        }).collect();

        Response::with((content_type, status::Ok, 
            serde_json::to_string(&displayed_events).unwrap()))

    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))
    }
}


impl Handler for GameEvents {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);

        let resp = match session_user_id {
            Some(user_id) => {
                info!("valid user - checking game id");
                match *query {
                    Some(id) => {
                        self.output(user_id, id.parse::<u64>().unwrap())
                    },
                    _ => {
                        info!("invalid id: {:?}", query);
                        self.output_error()
                    }
                }
            },
            _ => self.output_error()
        };

        Ok(resp)
    }
}
