use std::panic::RefUnwindSafe;
use std::collections::BTreeMap;
use chrono::prelude::*;
use time::Duration;
use tokio_core::reactor::Core;
use controller::{Controller, ResponseType};
use helpers::{PathExtractor, QueryStringExtractor};
use model::Session;

use data_access::event::Event as EventData;
use data_access::game::Game as GameData;

use helpers;
use serde_json;
use model::time_limit::TimeLimit as TimeLimitModel;

#[derive(Clone)]
pub struct TimeLimitController{
    event_data: EventData,
    game_data: GameData,
}

impl TimeLimitController {
    pub fn new(event_data: EventData, game_data: GameData) -> TimeLimitController {
        TimeLimitController {
            event_data: event_data,
            game_data: game_data,
        }
    }

    fn execute(&self, _:u64, game_id: u64) -> String {
        let last_event = self.event_data.get_last_game_event(game_id);
        let game = self.game_data.get_game(game_id);

        /*
        match game {
            Some(_) => (),
            None => return self.output_error()
        }
        */

        match last_event {
            None => return self.empty_limit(game_id),
            Some(_) => ()
        }

        let now: DateTime<Utc> = Utc::now();
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

        serde_json::to_string(&time_limit).unwrap()
    }

    fn empty_limit(&self, game_id: u64) -> String {
        let time_limit = TimeLimitModel{
            game: game_id,
            status: None
        };

        serde_json::to_string(&time_limit).unwrap()

    }

/*
    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        Response::with((self.content_type.to_owned(), status::InternalServerError, json_error))
    }
    */
}

impl Controller for TimeLimitController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _: Option<String>,
        path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            let path_id = path.expect("no_path").id as u64;
            let json = self.execute(id, path_id);
            ResponseType::Json(json)
        } else {
           ResponseType::Json("{}".to_string())
        }
    }
}


impl RefUnwindSafe for TimeLimitController {}
