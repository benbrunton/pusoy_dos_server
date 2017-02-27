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
                                        JOIN pusoy_dos.round ON round.game = game.id
                                        WHERE game.complete = 1", ()).unwrap(); 

        let mut map = HashMap::new();
        for row in result {
            match row{
                Ok(mut game_data) => {
                    let winners_serialised = game_data.take::<String, &str>("winners")
                            .expect("unable to get winners from db");
                    let winners:Vec<u16> = json::decode(&winners_serialised)
                            .expect("unable to decode winners");

                    if winners.len() > 0 {
                        let winner = winners[0];

                        let count = map.entry(winner).or_insert(0);
                         *count += 1;
                    }

                },
                _ => ()
            }
        }

        info!("{:?}", map);

        let mut leaderboard = vec!();
        for (uid, count) in map.iter() {
            let (name, played) = self.get_user_details(*uid).unwrap();
            let dec:f32 = *count as f32 / played as f32;
            let perc = dec * 100.0;

            leaderboard.push((uid, count, name, played, perc as u64));
        }

        leaderboard.sort_by(|&(_, _, _, _, e1), &(_, _, _, _, e2)| e2.cmp(&e1) );


        let mut sorted_lb = vec!();
        let mut count = 1;

        for (a,b,c,d,e) in leaderboard {

            sorted_lb.push(LeaderboardModel{
                id: *a as u64,
                name: c,
                position: count,
                wins: *b as u64,
                played: d as u64,
                win_percentage: e as u64
            });

            count += 1;

        }

        Some(sorted_lb)

 
    }

    fn get_user_details(&self, id:u16) -> Option<(String, u64)> {
        let mut result = self.pool.prep_exec(r"SELECT name, c FROM pusoy_dos.user
                                            JOIN (SELECT user, COUNT(*) c 
                                                FROM pusoy_dos.user_game GROUP BY user) a 
                                                ON a.user = user.id
                                            WHERE id = :id",
                                            params!{
                                                "id" => id
                                            }).unwrap();

        let row = result.next();
        match row {
            Some(row) => {
                let mut r = row.unwrap();
                let name = r.get("name").unwrap();
                let played = r.get("c").unwrap();
                Some((name, played))
            },
            _ => None
        }

    }

}

