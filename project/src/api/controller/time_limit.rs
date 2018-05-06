use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;
use std::collections::BTreeMap;
use chrono::prelude::*;
use time::Duration;

use data_access::event::Event as EventData;
use data_access::game::Game as GameData;

use helpers;
use serde_json;
use model::time_limit::TimeLimit as TimeLimitModel;

#[derive(Clone)]
pub struct TimeLimit{
    event_data: EventData,
    game_data: GameData,
    content_type: Mime
}

impl TimeLimit {
    pub fn new(event_data: EventData, game_data: GameData) -> TimeLimit {
        let content_type = "application/json".parse::<Mime>().unwrap();
        TimeLimit {
            event_data: event_data,
            game_data: game_data,
            content_type: content_type
        }
    }

    fn output(&self, _:u64, game_id: u64) -> Response {
        let last_event = self.event_data.get_last_game_event(game_id);
        let game = self.game_data.get_game(game_id);

        match game {
            Some(_) => (),
            None => return self.output_error()
        }

        match last_event {
            None => return self.empty_limit(game_id),
            Some(_) => ()
        }

        let now: DateTime<UTC> = UTC::now();
        let g = game.unwrap();
        let last_ev = last_event.unwrap();

        let actual_duration = now.signed_duration_since(last_ev);
        let max_move_mins = g.max_move_duration_mins;
        let max_move_secs = (max_move_mins * 60) as i64;
        let move_duration = match max_move_secs {
            0 => None,
            n => Some(Duration::seconds(n) - actual_duration)
        };


        let time_limit = TimeLimitModel{
            game: game_id,
            status: move_duration
        };



        Response::with((self.content_type.to_owned(), status::Ok, 
            serde_json::to_string(&time_limit).unwrap()))
    }

    fn empty_limit(&self, game_id: u64) -> Response {
        let time_limit = TimeLimitModel{
            game: game_id,
            status: None
        };

        Response::with((self.content_type.to_owned(), status::Ok, 
            serde_json::to_string(&time_limit).unwrap()))

    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        Response::with((self.content_type.to_owned(), status::InternalServerError, json_error))
    }
}


impl Handler for TimeLimit {

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
