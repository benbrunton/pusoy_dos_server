use router::Router;

use data_access::game::Game as GameData;

use external::controller::game_history;

pub fn new(game_data:GameData) -> Router {

    let mut router = Router::new();

    let game_history_controller = game_history::GameHistory::new(game_data.clone());

    router.get("/game-history", game_history_controller, "api_game_history");

    router
}
