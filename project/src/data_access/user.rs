use time::Timespec;
use chrono::prelude::*;

use model::user::{PartUser, User as UserModel};
use mysql;

#[derive(Clone)]
pub struct User{
    pool: mysql::Pool
}

impl User {
    pub fn new(pool: mysql::Pool) -> User {

        User {
            pool: pool
        }
    }


    // insert into db if new user
    // return full user
    pub fn create_if_new(&self, user:PartUser) -> UserModel {
        let existing = self.retrieve_user(user.clone());

        match existing {
            Some(result) => result,
            _ => self.insert_user(user.clone())
        }
    }

    pub fn get_users_by_game(&self, id:u64) -> Vec<UserModel> {
        info!("getting users from game: {}", id);
        let result = self.pool.prep_exec(r"SELECT user.id, user.name, user.provider_type, user.provider_id, user.creation_date
                                            FROM pusoy_dos.user_game
                                            JOIN pusoy_dos.user on pusoy_dos.user_game.user = user.id
                                            WHERE pusoy_dos.user_game.game = :id
                                            ORDER BY user_game.id", params!{
                                                "id" => id
                                            }).unwrap();

        result.map(|result|{

            match result {

                Ok(mut row) => {
                    UserModel{
                        id: row.take("id").unwrap(),
                        provider_id: row.take("provider_id").unwrap(),
                        provider_type: row.take("provider_type").unwrap(),
                        name: row.take("name").unwrap(),
                        creation_date: String::from("")
                    }
                },
                _ => UserModel{ id: 0, provider_id:String::from(""), provider_type: String::from(""), name:String::from(""), creation_date: String::from("")}
            }
        }).collect()

    }

    fn insert_user(&self, user:PartUser) -> UserModel {
        info!("Creating new user : {}", user.name);
        let utc: DateTime<UTC> = UTC::now();
        let query_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.user
                ( name, provider_id, provider_type, creation_date)
            VALUES
                (:name, :id, :type, :creation_date)",
            params!{
                "name" => user.name.clone(),
                "id" => user.provider_id.clone(),
                "type" => user.provider_type.clone(),
                "creation_date" =>  format!("{}", utc.format("%Y-%m-%d][%H:%M:%S"))
            }).unwrap();

            UserModel{
                id: query_result.last_insert_id(),
                name: user.name.clone(),
                provider_type: user.provider_type.clone(),
                provider_id: user.provider_id.clone(),
                creation_date: String::from(format!("{}", utc.format("%Y-%m-%d][%H:%M:%S")))
            }
    }

    fn retrieve_user(&self, user:PartUser) -> Option<UserModel> {

        // must be mutable because Row::take (used below) is a mutable action
        let mut user_record = self.pool.prep_exec(r"SELECT id,
                                                    name,
                                                    creation_date,
                                                    provider_id,
                                                    provider_type
                                FROM pusoy_dos.user
                                WHERE provider_id = :id
                                AND provider_type = :type",
                            params!{
                                "id" => user.provider_id,
                                "type" => user.provider_type
                            }).unwrap();

        let retrieved_user = match user_record.next() {
            Some(result) => {

                let mut row = result.unwrap();
                let id = row.take("id").unwrap();
                info!("User found with id : {}", id);

                Some(UserModel{
                    id: id,
                    name: row.take("name").unwrap(),
                    creation_date: String::from(""), //row.take("creation_date").unwrap(),
                    provider_id: row.take("provider_id").unwrap(),
                    provider_type: row.take("provider_type").unwrap()
                })

            },

            _ => {
                info!("No user found for {}", user.name);
                None
            }
        };

        retrieved_user
    }

    pub fn get_username_from_id(&self, id: u64) -> Option<String> {
        let mut user_record = self.pool.prep_exec(r"SELECT name
                                                  FROM pusoy_dos.user
                                                  WHERE id = :id",
                                                  params!{
                                                       "id" => id
                                                  }).unwrap();
        let retrieved_user = match user_record.next() {
            Some(result) => {
                let mut row = result.unwrap();
                row.take("name").unwrap()
            },
            _ => {
                None
            }
        };

        retrieved_user
    }
}
