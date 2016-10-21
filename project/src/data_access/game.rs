use model::game::Game as GameModel;
use mysql;
use logger;

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

    pub fn create_game(&self, user:u64) -> GameModel {
        logger::info(format!("User {} created new game", user));

        let query_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.game
                ( creator )
            VALUES
                (:user)",
            params!{
                "user" => user
            }).unwrap();

            GameModel{
                id: query_result.last_insert_id(),
                creator_id: user, 
                creator_name: String::from("current user")
            }
    }

    // todo: should be a linking table - not just games created by user
    pub fn get_valid_games(&self, user:u64) -> Vec<GameModel> {
        logger::info(format!("Getting games for user {}", user));

        let mut games = self.pool.prep_exec(r"SELECT pusoy_dos.game.id, 
                                                    creator,
                                                    name
                                    FROM pusoy_dos.game
                                    JOIN pusoy_dos.user ON creator = user.id
                                WHERE creator = :user",
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

