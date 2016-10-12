use mysql;
use util::session::Session as SessionModel;

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

    pub fn store_session(&self, session: &SessionModel) {
        self.pool.prep_exec(r"INSERT INTO pusoy_dos.session
                ( id, user_id)
            VALUES
                (:id, :user_id)",
            params!{
                "id" => format!("{}", session.key.clone()),
                "user_id" => session.user_id.clone()
            }).unwrap();

    }
}
