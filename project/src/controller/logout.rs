use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;

use util::session::SessionInstruction;
use config::Config;

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

}
impl Handler for LogoutController {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let instruction = SessionInstruction::DELETE;
        req.extensions.insert::<SessionInstruction>(instruction);
        
        let url =  Url::parse(&self.hostname).unwrap();

        Ok(Response::with((status::Found, modifiers::Redirect(url))))

    }

}
 
