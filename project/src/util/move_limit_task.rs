use data_access::game::Game as GameData;

pub fn execute(game_data: GameData) {

    let games = game_data.get_started_games_with_move_limit();

    info!("games: {:?}", games);
}
