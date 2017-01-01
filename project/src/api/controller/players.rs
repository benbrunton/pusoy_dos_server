use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;

use data_access::round::Round as RoundData;
use data_access::user::User as UserData;

pub struct Players{
    round_data: RoundData,
    user_data: UserData
}

impl Players {
    pub fn new(round_data: RoundData, user_data: UserData) -> Players {
        Players {
            round_data: round_data,
            user_data: user_data
        }
    }

    fn output_players(&self, round_id: u64) -> IronResult<Response> {
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, "here's your players!")))
   
    }
}

impl Handler for Players {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.output_players(4)
    }
}
