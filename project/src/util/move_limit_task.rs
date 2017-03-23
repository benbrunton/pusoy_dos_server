use data_access::game::Game as GameData;

pub fn execute(game_data: GameData) {

    let games = game_data.get_started_games_with_move_limit();

    info!("games: {:?}", games);

    // TODO: iterate through games
    // TODO: get latest move from event table
    // TODO: IF move > game limit : load game and pass current user
    // TODO: ensure to log the event to prevent cycle of passing
}
