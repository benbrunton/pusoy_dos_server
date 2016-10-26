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

        let result = self.pool.prep_exec(r"SELECT game.id, creator, name
                                        FROM pusoy_dos.game
                                        JOIN pusoy_dos.user ON creator = user.id
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
                        Some(GameModel{
                            id: game_data.get("id").unwrap(),
                            creator_id: game_data.get("creator").unwrap(),
                            creator_name: game_data.get("name").unwrap()
                        })
                    },
                    _ => {
                        info!("No game found with id: {}", id);
                        None
                    }

                }
            },
            _ => {
                error!("Error from getting game : {}", id);
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

         self.pool.prep_exec(r"INSERT INTO pusoy_dos.user_game
                                    (game, user)
                                VALUES
                                    (:game, :user)",
                                params!{
                                    "game" => new_game,
                                    "user" => user    
                                }).unwrap();

         GameModel{
            id: new_game,
            creator_id: user, 
            creator_name: String::from("current user")
         }
    }

    pub fn get_valid_games(&self, user:u64) -> Vec<GameModel> {
        info!("Getting games for user {}", user);

        self.get_game_list(r"SELECT pusoy_dos.game.id, 
                            creator,
                            name
                        FROM pusoy_dos.user_game
                        JOIN pusoy_dos.game ON pusoy_dos.game.id = game
                        JOIN pusoy_dos.user ON creator = user.id
                    WHERE user = :user", user)
    }

    pub fn get_open_games(&self, user:u64) -> Vec<GameModel> {
        info!("Getting open games for user {}", user);
        
        self.get_game_list(r"SELECT pusoy_dos.game.id, 
                            creator,
                            name
                FROM pusoy_dos.game
                JOIN pusoy_dos.user ON creator = user.id
                WHERE creator != :user", user)

    }

    fn get_game_list(&self, query:&str, user:u64) -> Vec<GameModel> {

        let games = self.pool.prep_exec(query,
                            params!{
                                "user" => user
                            }).unwrap();

        games.map(|result|{

            match result {

                Ok(mut row) => {
                    GameModel{
                        id: row.take("id").unwrap(),
                        creator_id: row.take("creator").unwrap(),
                        creator_name: row.take("name").unwrap()
                    }
                },
                _ => GameModel{ id: 0, creator_id:0, creator_name:String::from("")}
            }
        }).collect()

    }

}

