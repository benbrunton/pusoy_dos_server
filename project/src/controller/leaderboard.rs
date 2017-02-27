use iron::prelude::*;
use iron::middleware::Handler;
use tera::{Tera, Context, TeraResult};

use data_access::leaderboard::Leaderboard as LeaderboardData;
use config::Config;
use helpers;

pub struct Leaderboard {
    tera: &'static Tera,
    hostname: String,
    leaderboard: LeaderboardData
}

impl Leaderboard {
    pub fn new(config: &Config, tera:&'static Tera, leaderboard_data: LeaderboardData) -> Leaderboard {

        let hostname = config.get("hostname").unwrap();

        Leaderboard{ 
            tera: tera,
            hostname: hostname,
            leaderboard: leaderboard_data
        }
    }

    fn get_page(&self, _:u64) -> TeraResult<String> {

        let lb = self.leaderboard.get_leaderboard().unwrap();
        let mut data = Context::new(); 
        data.add("logged_in", &true);
        data.add("leaderboard", &lb);

        self.tera.render("leaderboard.html", data)
    }
}

impl Handler for Leaderboard {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(id) => helpers::render(self.get_page(id)),
            _        => redirect_to_homepage
        };

        Ok(resp)

    }

}
