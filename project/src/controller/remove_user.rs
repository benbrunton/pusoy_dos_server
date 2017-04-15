use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use config::Config;
use data_access::game::Game as GameData;
use helpers;

pub struct RemoveUser {
    hostname: String,
    game_data: GameData
}

impl RemoveUser {

    pub fn new(config: &Config, game_data: GameData) -> RemoveUser {
        let hostname = config.get("hostname").unwrap();
        RemoveUser{ hostname: hostname, game_data: game_data }
    }

    fn remove_user(&self, user: u64, id:u64) -> Response {

        self.game_data.remove_user(user, id);

        info!("user {} removed from game {}", user, id);

        let full_url = format!("{}/game/{}", self.hostname, id);
        let url =  Url::parse(&full_url).unwrap();

        Response::with((status::Found, modifiers::Redirect(url)))
    }


}

impl Handler for RemoveUser {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let router = req.extensions.get::<Router>().unwrap();
        let ref query = router.find("id");
        let ref user_query = router.find("user");
        info!("rendering game page for id: {:?}", query);

        let remove_user = user_query.unwrap();

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "games");
        
        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => self.remove_user(
                        remove_user.parse::<u64>().unwrap_or(0), 
                        id.parse::<u64>().unwrap_or(0)),
                    _ => redirect_to_homepage
                }

            },
            _ => redirect_to_homepage
        };

        Ok(resp)

    }

}
