
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
        self.pool.prep_exec(r"INSERT INTO pusoy_dos.user
                ( name, provider_id, provider_type)
            VALUES
                (:name, :id, :type)",
            params!{
                "name" => user.name.clone(),
                "id" => user.provider_id.clone(),
                "type" => user.provider_type.clone()
            }).unwrap();

            UserModel{
                id: 0,
                name: user.name.clone(),
                provider_type: user.provider_type.clone(),
                provider_id: user.provider_id.clone()
            }

    }
}
