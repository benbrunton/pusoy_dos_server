
use router::Router;

use data_access::round::Round as RoundData;
use data_access::user::User as UserData;

use api::controller::{ players, last_move };

pub fn new(round_data:RoundData, user_data:UserData) -> Router {

    let mut router = Router::new();

    let players_controller = players::Players::new(round_data.clone(), user_data);
    let last_move_controller = last_move::LastMove::new(round_data.clone());

    router.get("/players/:id", players_controller, "api_players");
    router.get("/last-move/:id", last_move_controller, "api_last_move");

    router
}
