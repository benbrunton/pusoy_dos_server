use model::leaderboard::Leaderboard as LeaderboardModel;
use mysql;
use hyper::client::Client;
use std::io::Read;

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

        let leaderboard_req = self.fetch_json(
                            String::from("http://localhost:8080/stats/leaderboard/"));

        match leaderboard_req {
            Ok(leaderboard) => {
                let mut pos = 0;
                let model_list = leaderboard.iter().map(|ref x| {
                    let y = x.as_object().unwrap();
                    pos = pos + 1;
                    LeaderboardModel{
                        name: String::from(y.get("name").unwrap()
                                .as_string().unwrap_or("unknown")),
                        position: pos,
                        wins: y.get("wins").unwrap()
                                .as_u64().unwrap_or(0),
                        played: y.get("played").unwrap()
                                .as_u64().unwrap_or(0),
                        losses: y.get("losses").unwrap()
                                .as_u64().unwrap_or(0),
                        rating: y.get("rating").unwrap()
                                .as_f64().unwrap_or(0.0)
                    }
                }).collect();


                Some(model_list)
            },
            _ => None

        }

    }

    fn fetch_json(&self, url:String) -> Result<Vec<Json>, String>{

        let client = Client::new();

        info!("requesting json from : {:?}", url);

        let res = client.get(&url).send();

        match res {
            Err(e) => {
                let err = format!("Error requesting json: {:?}", e);
                warn!("{}", &err);
                return Err(err.clone())
            },
            _ => ()
        }

        let mut r = res.unwrap();
        let mut buffer = String::new();
        let _ = r.read_to_string(&mut buffer);

        let data = Json::from_str(&buffer).unwrap();

        Ok(data.as_array().unwrap().clone())

    }

}

