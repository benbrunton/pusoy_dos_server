use std::str;
use model::game::Game as GameModel;
use mysql;

#[derive(Clone)]
pub struct Game{
    pool: mysql::Pool 
}

impl Game {

    pub fn new(pool: mysql::Pool) -> Game {
        
        Game {
            pool: pool
        }
    }

    pub fn get_game(&self, id:u64) -> Option<GameModel> {
        info!("Retrieving game {}", id);

        let result = self.pool.prep_exec(r"SELECT game.id, 
                                                creator, 
                                                u1.name name, 
                                                started, 
                                                current_player, 
                                                u2.name current_name
                                        FROM pusoy_dos.game
                                            INNER JOIN pusoy_dos.user u1 ON creator = u1.id
                                            LEFT JOIN pusoy_dos.round r ON game.id = r.game
                                            LEFT JOIN pusoy_dos.user u2 ON r.current_player = u2.id
                                        WHERE game.id = :id", 
                                        params!{
                                            "id" => id
                                        });

        match result {

            Ok(mut r) => {
                info!("game found with id: {}", id);
                let row = r.next();
                match row {
                    Some(game) => {
                        let mut game_data = game.unwrap();
                        let started:u8 = game_data.get("started").unwrap();

						let current_name:Option<String> = match game_data.take("current_name") {
							Some(mysql::Value::Bytes(a)) => Some(String::from(str::from_utf8(&a).unwrap())),
							_				  => None
							
						};

						let current_player_value = game_data.take("current_player");
						let current_id:u64 = match current_player_value {
							Some(mysql::Value::UInt(n)) => n,
							Some(mysql::Value::Int(n)) => n as u64,
							Some(mysql::Value::Float(n)) => n as u64,
							_				         => 0
						};

                        Some(GameModel{
                            id: game_data.get("id").unwrap(),
                            creator_id: game_data.get("creator").unwrap(),
                            creator_name: game_data.get("name").unwrap(),
                            started: started == 1,
                            next_player_name: current_name,
                            next_player_id: Some(current_id)
                        })
                    },
                    _ => {
                        info!("No game found with id: {}", id);
                        None
                    }

                }
            },
            _ => {
                error!("Error while getting game : {}", id);
                None
            }
        }
        
    }

    pub fn create_game(&self, user:u64) -> GameModel {
        info!("User {} created new game", user);

        let query_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.game
                ( creator )
            VALUES
                (:user)",
            params!{
                "user" => user
            }).unwrap();

         let new_game = query_result.last_insert_id();

         let _ = self.join_game(user, new_game);

         GameModel{
            id: new_game,
            creator_id: user, 
            creator_name: String::from("current user"),
            started: false,
            next_player_name: None,
            next_player_id: None
         }
    }

    pub fn join_game(&self, user:u64, new_game:u64) -> Result<(), String>{

         self.pool.prep_exec(r"INSERT INTO pusoy_dos.user_game
                                    (game, user)
                                VALUES
                                    (:game, :user)",
                                params!{
                                    "game" => new_game,
                                    "user" => user    
                                }).unwrap();

        Ok(())
    }

    pub fn get_valid_games(&self, user:u64) -> Vec<GameModel> {
        info!("Getting games for user {}", user);

        self.get_game_list(r"SELECT game.id, 
                                    creator, 
                                    u1.name name, 
                                    started, 
                                    current_player, 
                                    u2.name current_name,
									user_game.user user,
									complete
							FROM pusoy_dos.user_game
							JOIN pusoy_dos.game ON pusoy_dos.game.id = game
							JOIN pusoy_dos.user u1 ON creator = u1.id
							LEFT JOIN pusoy_dos.round r ON game.id = r.game
							LEFT JOIN pusoy_dos.user u2 ON r.current_player = u2.id
                    WHERE user = :user AND complete = 0", user)
    }

    pub fn get_open_games(&self, user:u64) -> Vec<GameModel> {
        info!("Getting open games for user {}", user);
        
        self.get_game_list(r"SELECT pusoy_dos.game.id, 
                            creator,
                            name,
							'unknown' as current_name,
							0 as current_id
                FROM pusoy_dos.game 
                JOIN pusoy_dos.user ON creator = user.id
                LEFT JOIN pusoy_dos.user_game ON user_game.game = game.id AND user = :user
                    WHERE user_game.user IS NULL AND started = 0", user)

    }

    pub fn get_players(&self, id:u64) -> Vec<u64>{
        info!("Getting users for game {}", id);

        let result = self.pool.prep_exec(r"SELECT user
                            FROM pusoy_dos.user_game
                            WHERE game = :game",
                            params!{
                                "game" => id
                            }).unwrap();

        result.map(|player|{
            player.unwrap().take("user").unwrap()
        }).collect()

    }

    pub fn start_game(&self, id:u64) -> Result<(), &str>{
        let _ = self.pool.prep_exec(r"UPDATE pusoy_dos.game SET started = 1 WHERE id = :id",
            params!{ "id" => id });
        Ok(())
    }

    pub fn complete_game(&self, id:u64) -> Result<(), &str>{
        let _ = self.pool.prep_exec(r"UPDATE pusoy_dos.game SET complete = 1 WHERE id = :id",
                params!{ "id" => id });
        Ok(())
    }

    fn get_game_list(&self, query:&str, user:u64) -> Vec<GameModel> {

        let games = self.pool.prep_exec(query,
                            params!{
                                "user" => user
                            }).unwrap();

        games.map(|result|{

            match result {

                Ok(mut row) => {
                    let started:u8 = row.take("started").unwrap_or(0);

					let current_name:Option<String> = match row.take("current_name") {
						Some(mysql::Value::Bytes(a)) => Some(String::from(str::from_utf8(&a).unwrap())),
						_				  => None
						
					};

					let current_player_value = row.take("current_player");
					let current_id:u64 = match current_player_value {
						Some(mysql::Value::UInt(n)) => n,
						Some(mysql::Value::Int(n)) => n as u64,
						Some(mysql::Value::Float(n)) => n as u64,
						_				         => 0
					};

                    GameModel{
                        id: row.take("id").unwrap(),
                        creator_id: row.take("creator").unwrap(),
                        creator_name: row.take("name").unwrap(),
                        started: started == 1,
                        next_player_name: current_name,
                        next_player_id: Some(current_id)
                    }
                },
                _ => GameModel{ id: 0, creator_id:0, creator_name:String::from(""), started: false, next_player_name: None, next_player_id: None }
            }
        }).collect()

    }

}

