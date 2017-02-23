use model::leaderboard::Leaderboard as LeaderboardModel;
use mysql;
use rustc_serialize::json;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Leaderboard{
    pool: mysql::Pool 
}

impl Leaderboard {

    pub fn new(pool: mysql::Pool) -> Leaderboard {
        
        Leaderboard {
            pool: pool
        }
    }

    pub fn get_leaderboard(&self) -> Option<Vec<LeaderboardModel>> {

        let result = self.pool.prep_exec(r"SELECT game.id, winners
                                        FROM pusoy_dos.game
                                        JOIN pusoy_dos.round ON round.id = game.id
                                        WHERE game.complete = 1", ()).unwrap(); 

        let mut map = HashMap::new();
        for row in result {
            match row{
                Ok(mut game_data) => {
                    let winners_serialised = game_data.take::<String, &str>("winners")
                            .expect("unable to get winners from db");
                    let winners:Vec<u16> = json::decode(&winners_serialised)
                            .expect("unable to decode winners");

                    let winner = winners[0];

                    let count = map.entry(winner).or_insert(0);
                     *count += 1;

                },
                _ => ()
            }
        }

        info!("{:?}", map);

        let mut leaderboard = vec!();
        for (uid, count) in map.iter() {
            let name = self.get_username(*uid).unwrap();

            leaderboard.push((uid, count, name));
        }

        leaderboard.sort_by(|&(_, b1, _), &(_, b2, _)| b2.cmp(b1) );


        let mut sorted_lb = vec!();
        let mut count = 1;

        for (a,b,c) in leaderboard {

            sorted_lb.push(LeaderboardModel{
                id: *a as u64,
                name: c,
                position: count,
                wins: *b as u64
            });

            count += 1;

        }

        Some(sorted_lb)

 
    }

    fn get_username(&self, id:u16) -> Option<String> {
        let mut result = self.pool.prep_exec(r"SELECT name FROM pusoy_dos.user
                                            WHERE id = :id",
                                            params!{
                                                "id" => id
                                            }).unwrap();

        let row = result.next();
        match row {
            Some(row) => {
                let name = row.unwrap().get("name");
                name
            },
            _ => Some(String::from("Unknown name"))
        }

    }

}

