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
                first_round = :first_round
            WHERE game = :game",
            params!{
                "game" => id,
                "hands" => json::encode(&game_def.players).unwrap(),
                "current_player" => game.get_next_player().unwrap().get_id(),
                "last_move" => json::encode(&round_def.last_move).unwrap(),
                "pass_count" => round_def.pass_count,
                "first_round" => round_def.first_round
            }).unwrap();
    }

    pub fn get(&self, id: u64) -> Option<GameDefinition> {

        let query_result = self.pool.prep_exec(r"SELECT
                id, 
                 game,
                 hands,
                 current_player,
                 last_move,
                 pass_count,
                 first_round
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

                        /*
                        TODO: builder for game definition

                        Card -> card!(rank, suit)
                        Move -> build_move(Vec<Card>)

                        Player::new(id)
                        player.set_hand(hand);

                        Round::new        players: Vec<u64>,
                                          current_player: u64,
                                          last_move: Move,
                                          passes: u64,
                                          first_round: bool

                        GameDefinition 
                        /// pub players: Vec<Player>,
                        /// pub round: Round,
                        /// pub winner: Option<u64> 

                         */

                        let last_move_serialised = game_data.get::<String, &str>("last_move").unwrap();
                        let last_move = json::decode(&last_move_serialised).unwrap();
                        info!("last move: {:?}", last_move);

                        let first_round = game_data.get::<u8, &str>("first_round").unwrap() == 1;
                        info!("is first round?: {}", first_round);

                        let players_serialised = game_data.get::<String, &str>("hands").unwrap();
                        let players:Vec<Player> = json::decode(&players_serialised).unwrap();
                        //info!("players: {:?}", players);

                        let player_ids = players.iter().map(|ref player| { player.get_id() }).collect();
                        info!("player ids: {:?}", player_ids);

                        let current_player = game_data.get("current_player").unwrap();
                        info!("current player {:?}", current_player);

                        let pass_count = game_data.get("pass_count").unwrap();
                        info!("pass count: {}", pass_count);
                        
                        let round = GameRound::new(player_ids, current_player, last_move, pass_count, first_round);
                        info!("round: {:?}", round);

                        Some(GameDefinition{
                            players: players,
                            round: round,
                            winner: None
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

