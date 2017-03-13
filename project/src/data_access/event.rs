use time::Timespec;
use chrono::prelude::*;
use mysql;
use model::event::Event as EventModel;

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
    pub fn insert_game_event(&self, game: u64, event_body: String){
        let utc: DateTime<UTC> = UTC::now();
        let a = self.pool.prep_exec(r"INSERT INTO pusoy_dos.event
                ( game, body, creation_date )
            VALUES
                (:game, :body, :creation_date)",
            params!{
                "game" => game,
                "body" => event_body,
                "creation_date" =>  format!("{}", utc.format("%Y-%m-%d][%H:%M:%S"))
            }).unwrap();

        info!("{:?}", a);
    }

    pub fn get_game_events(&self, game: u64) -> Vec<EventModel> {
        let event_query_result = self.pool.prep_exec(r"
                SELECT id, body, creation_date
                FROM pusoy_dos.event
                WHERE game = :game",
            params!{
                "game" => game
            }).expect("error accessing event table");

        event_query_result.map(|result| {

            match result {
                Ok(mut row) => {
				    let body = row.take("body").unwrap_or(String::from(""));	
                    let ts: Timespec = row.take("creation_date").unwrap();
                    let stored_date: DateTime<UTC> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp(ts.sec, ts.nsec as u32),
                            UTC);

                    EventModel::new(
                        row.take("id"),
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
                        String::from(""), 
                        time 
                    )
                }
            }

        }).collect()
                                                    
    }

}
