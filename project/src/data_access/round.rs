use mysql;
use rustc_serialize::json;

use pusoy_dos::game::game::{ GameDefinition, Game };
use pusoy_dos::game::player::Player;
use pusoy_dos::game::round::Round as GameRound;

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
        
        let round_def = game_def.round.export();
        let game = Game::load(game_def.clone()).unwrap();

        let _ = self.pool.prep_exec(r"INSERT INTO pusoy_dos.round
              (  game,
                 hands,
                 current_player,
                 last_move,
                 pass_count,
                 first_round,
                 winners,
                 reversed
            )
            VALUES
                ( :game, 
                  :hands, 
                  :current_player, 
                  :last_move,
                  :pass_count,
                  :first_round,
                  :winners,
                  :reversed
                )",
            params!{
                "game" => id,
                "hands" => json::encode(&game_def.players).unwrap(),
                "current_player" => game.get_next_player().unwrap().get_id(),
                "last_move" => json::encode(&round_def.last_move).unwrap(),
                "pass_count" => round_def.pass_count,
                "first_round" => round_def.first_round,
                "winners" => json::encode(&game_def.winners).expect("unable to encode winners"),
                "reversed" => if game_def.reversed { 1 } else { 0 }
            }).unwrap();

            // todo - return result

    }

    pub fn update_round(&self, id: u64, game_def: GameDefinition) {

        let round_def = game_def.round.export();
        let game = Game::load(game_def.clone()).unwrap();

        let _ = self.pool.prep_exec(r"UPDATE pusoy_dos.round
            SET hands = :hands,
                current_player = :current_player,
                last_move = :last_move,
                pass_count = :pass_count,
                first_round = :first_round,
                winners = :winners,
                reversed = :reversed
            WHERE game = :game",
            params!{
                "game" => id,
                "hands" => json::encode(&game_def.players)
                    .expect("unable to encode player hands"),
                "current_player" => game.get_next_player()
                    .expect("unable to find current player").get_id(),
                "last_move" => json::encode(&round_def.last_move)
                    .expect("unable to encode last move"),
                "pass_count" => round_def.pass_count,
                "first_round" => round_def.first_round,
                "winners" => json::encode(&game_def.winners)
                    .expect("unable to encode winners"),
                "reversed" => if game_def.reversed { 1 } else { 0 }
            }).expect("update round failed");
    }

    pub fn get(&self, id: u64) -> Option<GameDefinition> {

        let query_result = self.pool.prep_exec(r"SELECT
                id, 
                 game,
                 hands,
                 current_player,
                 last_move,
                 pass_count,
                 first_round,
                 winners,
                 reversed
             FROM pusoy_dos.round
            WHERE game = :id",
            params!{
                "id" => id
            });

        match query_result {

            Ok(mut r) => {
                info!("game found with id: {}", id);
                let row = r.next();
                match row {
                    Some(game) => {
                        let mut game_data = game.unwrap();

                        let last_move_serialised = game_data.get::<String, &str>("last_move")
                                .expect("unable to get last move");
                        let last_move = json::decode(&last_move_serialised)
                                .expect("unable to decode last move");

                        let first_round = game_data.get::<u8, &str>("first_round")
                                .expect("unable to get `is first round`") == 1;

                        let players_serialised = game_data.get::<String, &str>("hands")
                                .expect("unable to get player hands");

                        let players:Vec<Player> = json::decode(&players_serialised)
                                .expect("unable to decode player hands");

                        let player_ids = players.iter()
                                .filter(|ref player| {player.get_hand().len() > 0 })
                                .map(|ref player| { player.get_id() }).collect();

                        let current_player = game_data.get("current_player")
                                .expect("unable to get current player");

                        let pass_count = game_data.get("pass_count")
                                .expect("unable to get pass count");
                                
                        let reversed = game_data.get::<u8, &str>("reversed")
                                .expect("unable to get reversed") == 1;
                        
                        let round = GameRound::new(
                                            player_ids, 
                                            current_player, 
                                            last_move, 
                                            pass_count, 
                                            first_round);

                        let winners_serialised = game_data.get::<String, &str>("winners")
                                .expect("unable to get winners from db");
                        let winners = json::decode(&winners_serialised)
                                .expect("unable to decode winners");

                        Some(GameDefinition{
                            players: players,
                            round: round,
                            winners: winners,
                            reversed: reversed
                        })
                    },
                    _ => {
                        info!("No round found matching game : {}", id);
                        None
                    }

                }
            },
            Err(e) => {
                error!("Error while getting round for game : {}", id);
                error!("{}", e);
                None
            }
        }

    }


}

