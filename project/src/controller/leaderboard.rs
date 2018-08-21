use tera::{Tera, Context, Result as TeraResult};
use model::Session;
use std::panic::RefUnwindSafe;

use helpers;
use helpers::{PathExtractor, QueryStringExtractor};
use controller::{Controller, ResponseType};

use data_access::leaderboard::Leaderboard as LeaderboardData;

pub struct LeaderboardController {
    tera: &'static Tera,
    leaderboard: LeaderboardData
}

impl LeaderboardController {
    pub fn new(tera:&'static Tera, leaderboard_data: LeaderboardData) -> LeaderboardController {

        LeaderboardController{ 
            tera,
            leaderboard: leaderboard_data
        }
    }

    fn get_page(&self) -> TeraResult<String> {

        let lb_result = self.leaderboard.get_leaderboard();
        let mut data = Context::new(); 
        data.add("logged_in", &true);

        match lb_result {
            Some(lb) => {
                data.add("leaderboard", &lb);
            },
            _ => {
            }

        }

        self.tera.render("leaderboard.html", &data)
    }
}

impl Controller for LeaderboardController {

    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        _path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        
        if helpers::is_logged_in(session) {
            ResponseType::PageResponse(self.get_page().expect("unable to unwrap leaderboard page"))
        } else {
            ResponseType::Redirect("/".to_string())
        }
    }
}

impl RefUnwindSafe for LeaderboardController {}
