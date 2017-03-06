use std::str;
use std::collections::HashMap;
use model::game::Game as GameModel;
use rustc_serialize::json;
use hyper::client::Client;
use hyper::header::Headers;
use mysql;

#[derive(RustcDecodable, RustcEncodable)]
struct Player{
    id: u64,
    name: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct GameWinners{
    id: u64,
    players: Vec<Player>
}

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
                                                u2.name current_name,
                                                c num_players
                                        FROM pusoy_dos.game
                                            INNER JOIN pusoy_dos.user u1 ON creator = u1.id
                                            LEFT JOIN (SELECT game, COUNT(*) c FROM pusoy_dos.user_game GROUP BY game) a ON a.game = game.id
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
                            next_player_id: Some(current_id),
                            num_players: game_data.get("num_players").unwrap()
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
            next_player_id: None,
            num_players: 0
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
									complete,
                                    c num_players
							FROM pusoy_dos.user_game
							JOIN pusoy_dos.game ON pusoy_dos.game.id = game
							JOIN pusoy_dos.user u1 ON creator = u1.id
							LEFT JOIN pusoy_dos.round r ON game.id = r.game
							LEFT JOIN pusoy_dos.user u2 ON r.current_player = u2.id
                            LEFT JOIN (SELECT game, COUNT(*) c FROM pusoy_dos.user_game GROUP BY game) a ON a.game = game.id
                    WHERE user = :user AND complete = 0", user)
    }

    pub fn get_open_games(&self, user:u64) -> Vec<GameModel> {
        info!("Getting open games for user {}", user);
        
        self.get_game_list(r"SELECT pusoy_dos.game.id, 
                            creator,
                            name,
							'unknown' as current_name,
							0 as current_id,
                            c num_players
                FROM pusoy_dos.game 
                JOIN pusoy_dos.user ON creator = user.id
                LEFT JOIN (SELECT game, COUNT(*) c FROM pusoy_dos.user_game GROUP BY game) a ON a.game = game.id
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

        self.send_stat_update(id);
        Ok(())
    }

    // TODO - put me in a dedicated stats helper
    // and do some validation
    fn send_stat_update(&self, id:u64){
        let winners = self.get_winners(id);
        let body = json::encode(&winners).unwrap();
        // to POST /stats/leaderboard
        
        info!("sending : {}", {&body});

        let mut headers = Headers::new();
        headers.set_raw("content-type", vec!(b"application/json".to_vec()));
        let client = Client::new();
        let _ = client.post("http://localhost:8080/stats/leaderboard")
            .body(&body)
            .headers(headers)
            .send();
    }

    fn get_winners(&self, id: u64) -> GameWinners {
        let query = r"SELECT user, name from pusoy_dos.user_game
                        JOIN pusoy_dos.user ON user.id = user_game.user
                        WHERE user_game.game = :id";
        let all_players = self.pool.prep_exec(query,
                                    params!{
                                        "id" => id
                                    }).unwrap();

        let mut map = HashMap::new();

        for row in all_players {
            match row {
                Ok(mut player) => {
                    let id: u64 = player.take("user").unwrap();
                    let name: String = player.take("name").unwrap();
                    map.insert(id, String::from(name));
                },
                Err(_) => ()
            }
        }
        

        let winner_query = r"SELECT current_player, winners
                            FROM pusoy_dos.round WHERE game = :id";
        let mut winner_order = self.pool.prep_exec(winner_query,
                                params!{
                                    "id" => id
                                }).unwrap();

        let row = winner_order.next();
        let winners = match row {
            Some(row) => {
                    let mut r = row.unwrap();
					let winners_serialised = r.take::<String, &str>("winners")
                            .expect("unable to get winners from db");
                    let mut winners:Vec<u16> = json::decode(&winners_serialised)
                            .expect("unable to decode winners");

					let loser =  r.take("current_player").expect("gotta be a current_player");

					winners.push(loser);

					winners
            },
            _ => vec!()
        };

        let players = winners.iter().map(|player_id| {
            let id = *player_id as u64;
            let ref name = *map.get(&id).unwrap();

            Player {
                id: id,
                name:  name.to_owned()
           }
        }).collect();

        GameWinners{
            id: id,
            players: players
        }

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
                        next_player_id: Some(current_id),
                        num_players: row.take("num_players").unwrap_or(0)
                    }
                },
                _ => GameModel{ id: 0, creator_id:0, creator_name:String::from(""), started: false, next_player_name: None, next_player_id: None, num_players: 0 }
            }
        }).collect()

    }

}

