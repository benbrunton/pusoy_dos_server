use mysql;
use rustc_serialize::json;

use pusoy_dos::game::game::{GameDefinition, Game};

#[derive(Clone)]
pub struct Round{
    pool: mysql::Pool 
}

impl Round {

    pub fn new(pool: mysql::Pool) -> Round {
        Round {
            pool: pool
        }
    }

    pub fn create_round(&self, id: u64, game_def:GameDefinition) {
        /*
         info!("{:?}", new_game.round);

        // deal cards to peeps
        for player in new_game.players.iter() {
            info!("{:?}", player.get_id());
            // todo - data adapter for storing hands
            info!("{:?}", player.get_hand());
        }

        info!("{:?}", json::encode(&new_game.players)) */
        let round_def = game_def.round.export();
        let game = Game::load(game_def.clone()).unwrap();

        let query_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.round
              (  game,
                 hands,
                 current_player,
                 last_move,
                 pass_count,
                 first_round
            )
            VALUES
                ( :game, 
                  :hands, 
                  :current_player, 
                  :last_move,
                  :pass_count,
                  :first_round
                )",
            params!{
                "game" => id,
                "hands" => json::encode(&game_def.players).unwrap(),
                "current_player" => game.get_next_player().unwrap().get_id(),
                "last_move" => json::encode(&round_def.last_move).unwrap(),
                "pass_count" => round_def.pass_count,
                "first_round" => round_def.first_round
            }).unwrap();

    }


}

