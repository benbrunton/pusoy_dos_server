
use router::Router;

use data_access::round::Round as RoundData;
use data_access::user::User as UserData;

use api::controller::{ players };

pub fn new(round_data:RoundData, user_data:UserData) -> Router {

    let mut router = Router::new();

    let players_controller = players::Players::new(round_data, user_data);

    router.get("/players/:game_id", players_controller, "api_players");

    router
}
