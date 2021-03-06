use config::Config;
use std::panic::RefUnwindSafe;
use controller::{Controller, ResponseType};
use model::Session;
use helpers::{PathExtractor, QueryStringExtractor};

pub struct LogoutController{
    hostname: String
}

impl LogoutController{

    pub fn new(config: &Config) -> LogoutController {

        let hostname = config.get("pd_host").unwrap();

        LogoutController{
            hostname: hostname
        }
    }

    fn update_session(&self, session: &mut Option<Session>) {
        *session = None;
    }
}

impl Controller for LogoutController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        _path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        self.update_session(session);
        ResponseType::Redirect("/".to_string())
    }
}

impl RefUnwindSafe for LogoutController {}
