use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use config::Config;
use helpers;
use data_access::round::Round as RoundData;

pub struct GameMove{
    round_data: RoundData,
    hostname: String
}

pub struct Pass{
    round_data: RoundData,
    hostname: String
}

impl GameMove{

    pub fn new(config:&Config, round_data: RoundData) -> GameMove {
        let hostname = config.get("hostname").unwrap();
        GameMove{ hostname: hostname, round_data: round_data }
    }

    fn execute(&self, user_id:u64, game_id:u64) -> Response {

        let play_url = format!("play/{}", game_id);
        helpers::redirect(&self.hostname, &play_url)
    }

}

impl Handler for GameMove {


    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.execute(user_id, id.parse::<u64>().unwrap())
                    },
                    _ => redirect_to_homepage
                }
            },
            _ => redirect_to_homepage
        };

        Ok(resp)
    }

}

impl Pass{

    pub fn new(config:&Config, round_data: RoundData) -> Pass {
        let hostname = config.get("hostname").unwrap();
        Pass{ hostname: hostname, round_data: round_data }
    }

    fn pass(&self, user_id:u64, game_id:u64) -> Response {

        let play_url = format!("play/{}", game_id);
        helpers::redirect(&self.hostname, &play_url)
    }


}

impl Handler for Pass {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.pass(user_id, id.parse::<u64>().unwrap())
                    },
                    _ => redirect_to_homepage
                }
            },
            _ => redirect_to_homepage
        };

        Ok(resp)
    }

}
