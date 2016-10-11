use mysql;
use logger;

#[derive(Clone)]
pub struct Session{
    pool: mysql::Pool 
}

impl Session {
    pub fn new(pool: mysql::Pool) -> Session {
        
        Session {
            pool: pool
        }
    }

    pub fn store_session(session: SessionModel) {
        self.pool.prep_exec(r"INSERT INTO pusoy_dos.session
                ( id, user_id)
            VALUES
                (:id, :user_id)",
            params!{
                "id" => user.provider_id.clone(),
                "user_id" => session.user_id.clone()
            }).unwrap();

    }
