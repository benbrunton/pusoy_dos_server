use tera::{Tera, Context};
use data_access::event::Event as EventData;
use helpers;
use helpers::{PathExtractor, QueryStringExtractor};
use controller::{Controller, ResponseType};
use model::Session;
use std::panic::RefUnwindSafe;


pub struct PostGameController {
    tera: &'static Tera,
    event_data: EventData
}

impl PostGameController {

    pub fn new(tera: &'static Tera, event_data: EventData) -> PostGameController {
        
        PostGameController {
            tera,
            event_data
        }
    }

    pub fn get_page(&self, user_id: u64, game_id: u64) -> String {
        let mut events_vec = self.event_data.get_game_events(game_id);
        let mut events = events_vec.as_mut_slice();
        events.reverse();

        let mut data = Context::new();
        data.add("logged_in", &true);
        data.add("events", &events);
        self.tera.render("post_game.html", &data).expect("error rendering template")
    }
}

impl Controller for PostGameController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            let path_id = path.expect("no_path").id as u64;
            ResponseType::PageResponse(self.get_page(id, path_id))
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }
}

impl RefUnwindSafe for PostGameController {}
