use std::str;
use std::collections::HashMap;
use model::game::Game as GameModel;
use hyper::client::Client;
use hyper::{Method, Request};
use hyper::header::ContentType;
use tokio_core::reactor::Core;
use chrono::prelude::*;
use chrono::Utc;
use mysql;
use serde_json;

#[derive(Serialize, Deserialize)]
struct Player{
    id: u64,
    name: String
}

#[derive(Serialize, Deserialize)]
struct GameWinners{
    id: u64,
    players: Vec<Player>
}

#[derive(Clone)]
pub struct Game{
    pool: mysql::Pool,
    stat_endpoint: String
}

impl Game {

    pub fn new(pool: mysql::Pool, stat_endpoint: String) -> Game {
        
        Game {
            pool,
            stat_endpoint
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
                                                c num_players,
                                                game.max_move_duration,
                                                game.decks
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

                        let max_move_duration = match game_data.take("max_move_duration") {
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
                            num_players: game_data.get("num_players").unwrap(),
                            max_move_duration: self.get_max_move_duration(max_move_duration),
                            max_move_duration_mins: self.get_max_move_duration_mins(max_move_duration),
                            decks: game_data.get("decks").unwrap()
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

    pub fn create_game(&self, user:u64, max_move_duration:u64, max_players:u64, decks:u64) -> GameModel {
        info!("User {} created new game", user);

        let utc: DateTime<Utc> = Utc::now();
        let creation_date = format!("{}", utc.format("%Y-%m-%d %H:%M:%S"));

        let query_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.game
                ( creator, creation_date, max_move_duration, max_players, decks)
            VALUES
                (:user, :creation_date, :max_move_duration, :max_players, :decks)",
            params!{
                "user" => user,
                "creation_date" => creation_date,
                "max_move_duration" => max_move_duration,
                "max_players" => max_players,
                "decks" => decks
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
            num_players: 0,
            max_move_duration: self.get_max_move_duration(max_move_duration),
            max_move_duration_mins: self.get_max_move_duration_mins(max_move_duration),
            decks: decks
         }
    }

    pub fn join_game(&self, user:u64, new_game:u64) -> Result<(), String>{

        // TODO - if game matches max_players then it should start
        // if game exceeds max players then this should fail

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

    pub fn remove_user(&self, user:u64, game:u64) -> Result<(), String>{

        self.pool.prep_exec(r"DELETE FROM pusoy_dos.user_game
                                WHERE game = :game AND user = :user",
                                params!{
                                    "game" => game,
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
                                    decks,
									complete,
                                    c num_players,
                                    game.max_move_duration
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
                            decks,
							'unknown' as current_name,
							0 as current_id,
                            c num_players,
                            game.max_move_duration
                FROM pusoy_dos.game 
                JOIN pusoy_dos.user ON creator = user.id
                LEFT JOIN (SELECT game, COUNT(*) c FROM pusoy_dos.user_game GROUP BY game) a ON a.game = game.id
                LEFT JOIN pusoy_dos.user_game ON user_game.game = game.id AND user = :user
                    WHERE user_game.user IS NULL AND started = 0", user)

    }
    
    pub fn get_closed_games(&self, user:u64) -> Vec<GameModel> {
        info!("Getting closed games for user {}", user);

        self.get_game_list(r"SELECT game.id, 
                                    creator, 
                                    u1.name name, 
                                    started, 
                                    current_player, 
                                    u2.name current_name,
									user_game.user user,
                                    decks,
									complete,
                                    c num_players,
                                    game.max_move_duration
							FROM pusoy_dos.user_game
							JOIN pusoy_dos.game ON pusoy_dos.game.id = game
							JOIN pusoy_dos.user u1 ON creator = u1.id
							LEFT JOIN pusoy_dos.round r ON game.id = r.game
							LEFT JOIN pusoy_dos.user u2 ON r.current_player = u2.id
                            LEFT JOIN (SELECT game, COUNT(*) c FROM pusoy_dos.user_game GROUP BY game) a ON a.game = game.id
                    WHERE user = :user AND complete = 1", user)
    }

    // this is knacked
    pub fn get_all_closed_games(&self) -> Vec<GameModel> {
        info!("Getting closed games");

        self.get_general_game_list(r"SELECT game.id, 
                                    creator, 
                                    u1.name name, 
                                    started, 
                                    current_player, 
                                    u2.name current_name,
									user_game.user user,
                                    decks,
									complete,
                                    c num_players,
                                    game.max_move_duration
							FROM pusoy_dos.user_game
							JOIN pusoy_dos.game ON pusoy_dos.game.id = game
							JOIN pusoy_dos.user u1 ON creator = u1.id
							LEFT JOIN pusoy_dos.round r ON game.id = r.game
                            LEFT JOIN pusoy_dos.user u2 ON pusoy_dos.user_game.user = u2.id
                            LEFT JOIN (SELECT game, COUNT(*) c FROM pusoy_dos.user_game GROUP BY game) a ON a.game = game.id
                    WHERE complete = 1")
    }


    pub fn get_players(&self, id:u64) -> Vec<u64>{
        info!("Getting users for game {}", id);

        let result = self.pool.prep_exec(r"SELECT user
                            FROM pusoy_dos.user_game
                            WHERE game = :game
                            ORDER BY user_game.id",
                            params!{
                                "game" => id
                            }).unwrap();

        result.map(|player|{
            player.unwrap().take("user").unwrap()
        }).collect()

    }

    pub fn update_decks(&self, id:u64, decks:u64) {
        let _ = self.pool.prep_exec(r"UPDATE pusoy_dos.game 
                                    SET decks = :decks
                                    WHERE id = :id",
                                    params!{
                                        "id" => id,
                                        "decks" => decks
                                    });
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
        let body = serde_json::to_string(&winners).unwrap();
        // to POST /stats/leaderboard
        
        info!("sending : {}", {&body});
        let stat_endpoint1 = "http://localhost:8080/stats/leaderboard".parse()
            .expect("unable to parse stats endpoint 1");
        let stat_endpoint2 = format!("http://{}:3080/relay/{}", self.stat_endpoint, id).parse()
            .expect("unable to parse stats endpoint 2");

        let core = Core::new().expect("unable to unwrap core");
        let client = Client::new(&core.handle());
        let mut req = Request::new(Method::Post, stat_endpoint1);
        req.headers_mut().set(ContentType::json());
        req.set_body(body.clone());

        let _ = client.request(req);
        let mut req2 = Request::new(Method::Post, stat_endpoint2);
        req2.headers_mut().set(ContentType::json());
        req2.set_body(body.clone());

        let _ = client.request(req2);
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
                    let mut winners:Vec<u16> = serde_json::from_str(&winners_serialised)
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
        self.create_game_list(query, Some(user))
    }

    fn get_general_game_list(&self, query: &str) -> Vec<GameModel> {
        self.create_game_list(query, None)
    }

    fn create_game_list(&self, query:&str, user:Option<u64>) -> Vec<GameModel> {

        let games = match user {
            Some(u) => self.pool.prep_exec(query, params!{"user" => u}).unwrap(),
            None    => self.pool.prep_exec(query, ()).unwrap()
        };

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

                    let max_move_duration = match row.take("max_move_duration") {
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
                        num_players: row.take("num_players").unwrap_or(0),
                        max_move_duration: self.get_max_move_duration(max_move_duration),
                        max_move_duration_mins: self.get_max_move_duration_mins(max_move_duration),
                        decks: row.take("decks").unwrap_or(0)
                    }
                },
                _ => GameModel{ 
                        id: 0, 
                        creator_id:0, 
                        creator_name:String::from(""), 
                        started: false, 
                        next_player_name: None, 
                        next_player_id: None, 
                        num_players: 0, 
                        max_move_duration: String::from(""), 
                        max_move_duration_mins: 0,
                        decks: 0
                    }
            }
        }).collect()

    }

    pub fn get_started_games_with_move_limit(&self) -> Vec<(u64, u64)> {
        /*
            1 = Unlimited
            2 = 10 minutes
            3 = 1 hour
            4 = 4 hours
            5 = 8 hours
            6 = 1 day
            7 = 3 days
        */
        self.pool.prep_exec(r"SELECT id, max_move_duration
                                        FROM pusoy_dos.game
                                        WHERE started = 1
                                        AND complete = 0
                                        AND max_move_duration IS NOT NULL
                                        AND max_move_duration != 1", ())
            .map(|result|{
                
                result.map(|x| x.unwrap() ).map(|mut row| {
                    let move_duration = row.take("max_move_duration").unwrap_or(0);
                    let duration = self.get_max_move_duration_mins(move_duration);
                    (row.take("id").unwrap_or(0), duration)
                }).collect()
            }).unwrap()
                
    }

    fn get_max_move_duration(&self, code: u64) -> String {
        match code{
           2 => "10 minutes".to_string(),
           3 => "1 hour".to_string(),
           4 => "4 hours".to_string(),
           5 => "8 hours".to_string(),
           6 => "1 day".to_string(),
           7 => "3 days".to_string(),
           8 => "2 minutes".to_string(),
           _ => "No limit".to_string() 
        }
    }

    fn get_max_move_duration_mins(&self, code: u64) -> u64 {
        match code{
            2 => 10,
            3 => 60,
            4 => 60 * 4,
            5 => 60 * 8,
            6 => 60 * 24,
            7 => 60 * 24 * 3,
            8 => 2,
            _ => 0
        }
    }

}

