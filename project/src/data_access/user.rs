
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

    fn insert_user(&self, user:PartUser) -> UserModel {
        let query_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.user
                ( name, provider_id, provider_type)
            VALUES
                (:name, :id, :type)",
            params!{
                "name" => user.name.clone(),
                "id" => user.provider_id.clone(),
                "type" => user.provider_type.clone()
            }).unwrap();

            UserModel{
                id: query_result.last_insert_id(),
                name: user.name.clone(),
                provider_type: user.provider_type.clone(),
                provider_id: user.provider_id.clone(),
                creation_date: String::from("000")
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
                                WHERE id = :id",
                            params!{
                                "id" => user.provider_id,
                                "type" => user.provider_type
                            }).unwrap();

        match user_record.next() {
            Some(result) => {

                let mut row = result.unwrap();
                Some(UserModel{
                    id: row.take("id").unwrap(),
                    name: row.take("name").unwrap(),
                    creation_date: row.take("creation_date").unwrap(),
                    provider_id: row.take("provider_id").unwrap(),
                    provider_type: row.take("provider_type").unwrap()
                })

            },

            _ => None
        }
    }
}
