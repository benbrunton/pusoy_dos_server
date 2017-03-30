use time::Timespec;
use chrono::prelude::*;
use mysql;
use model::event::Event as EventModel;
use model::user::User as UserModel;

#[derive(Clone)]
pub struct Event{
    pool: mysql::Pool
}

impl Event {
    pub fn new(pool: mysql::Pool) -> Event {
        
        Event {
            pool: pool
        }
    }

    // put an event in
    pub fn insert_game_event(&self, user: u64, game: u64, event_body: String){
        let utc: DateTime<UTC> = UTC::now();
        let a = self.pool.prep_exec(r"INSERT INTO pusoy_dos.event
                ( game, user, body, creation_date )
            VALUES
                (:game, :user, :body, :creation_date)",
            params!{
                "game" => game,
                "user" => user,
                "body" => event_body,
                "creation_date" =>  format!("{}", utc.format("%Y-%m-%d][%H:%M:%S"))
            }).unwrap();

    }

    pub fn get_game_events(&self, game: u64) -> Vec<EventModel> {
        info!("getting events for game: {}", game);
        let event_query_result = self.pool.prep_exec(r"
                SELECT event.id, 
                        user.name user_name, 
                        user.id user_id, 
                        user.provider_type user_type,
                        user.provider_id user_prov_id,
                        user.creation_date user_date,
                        body, 
                        event.creation_date
                FROM pusoy_dos.event
                LEFT JOIN pusoy_dos.user on user.id = event.user
                WHERE game = :game
                ORDER BY event.creation_date DESC",
            params!{
                "game" => game
            }).expect("error accessing event table");

        info!("unwrapping query result");
        event_query_result.map(|result| {

            match result {
                Ok(mut row) => {

                    let user_name = row.take("user_name").unwrap_or(String::from("Unknown user"));
                    debug!("getting user id");
                    let user_id = row.take("user_id").unwrap_or(0);
                    debug!("getting provider type");
                    let provider_type = row.take("user_type")
                                            .unwrap_or(String::from("unknown"));
                    debug!("getting provider id");
                    let provider_id:String = row.take("user_prov_id").expect("provider_id not retrieved from db");

                    debug!("getting timespec");
                    let user_ts: Timespec = row.take("user_date").unwrap();
                    debug!("creating datetime");
                    let user_creation_date: DateTime<UTC> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp(user_ts.sec, user_ts.nsec as u32),
                            UTC);

                    debug!("getting message");
				    let body = row.take("body").unwrap_or(String::from(""));

                    debug!("creating timespec");
                    let ts: Timespec = row.take("creation_date").unwrap();
                    
                    debug!("creating event datetime");
                    let stored_date: DateTime<UTC> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp(ts.sec, ts.nsec as u32),
                            UTC);

                    debug!("creating user model");
                    let user = UserModel{
                        id: user_id,
                        name: user_name,
                        provider_id: format!("{}", provider_id),
                        provider_type: provider_type,
                        creation_date: format!("{}", user_creation_date.format("%Y-%m-%d][%H:%M:%S"))
                    };


                    EventModel::new(
                        row.take("id"),
                        Some(user),
                        Some(game),
                        body,
                        stored_date
                    )
                },
                _ => {
                    let time: DateTime<UTC> = UTC::now();
                    EventModel::new( 
                        None, 
                        None,
                        None, 
                        String::from(""), 
                        time 
                    )
                }
            }

        }).collect()
                                                    
    }

    pub fn get_last_game_event(&self, game_id: u64) -> Option<DateTime<UTC>> {
        info!("getting last event for game: {}", game_id);
        let mut result = self.pool.prep_exec(r"
                SELECT event.creation_date
                FROM pusoy_dos.event
                LEFT JOIN pusoy_dos.user on user.id = event.user
                WHERE game = :game
                ORDER BY event.creation_date DESC
                LIMIT 1",
            params!{
                "game" => game_id
            }).expect("error accessing event table");

        let row = result.next();
        match row {
            None => None,
            Some(r) => {
                let ts: Timespec = r.unwrap().get("creation_date").unwrap();
                let stored_date: DateTime<UTC> = DateTime::from_utc(
                        NaiveDateTime::from_timestamp(ts.sec, ts.nsec as u32),
                        UTC);

                Some(stored_date)
            }
        }


    }

}
