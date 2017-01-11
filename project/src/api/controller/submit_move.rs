use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use rustc_serialize::json;
use std::collections::BTreeMap;
use serde_json;
use serde_json::{Value, Map};

use helpers;
use helpers::DCard;
use bodyparser;

use data_access::round::Round as RoundData;
use data_access::user::User as UserData;

use pusoy_dos::game::game::Game;


#[derive(Clone)]
pub struct SubmitMove{
    round_data: RoundData
}

impl SubmitMove {

    pub fn new(round_data: RoundData) -> SubmitMove{
        SubmitMove{
            round_data: round_data
        }
    }

    pub fn execute(&self, user_id: u64, 
                        id: u64, 
                        json:Option<serde_json::Value>) -> Response {
        self.output_error()
    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))

    }
    
    fn get_body(&self, req: &mut Request) -> Option<serde_json::Value> {

        match req.get::<bodyparser::Json>(){
            Ok(json) => Some(json.expect("unable to unwrap json")),
            _ => None
        }
    }

}

impl Handler for SubmitMove{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref hashmap = self.get_body(req);


        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);

        let resp = match session_user_id {
            Some(user_id) => {
                info!("valid user - checking game id");
                match *query {
                    Some(id) => {
                        self.execute(user_id, id.parse::<u64>().unwrap(), hashmap.to_owned())
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
