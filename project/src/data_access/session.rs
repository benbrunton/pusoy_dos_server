use mysql;
use util::session::Session as SessionModel;
use logger;
use uuid::Uuid;

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
        let session_key = format!("{}", session.key.clone());
        let user_id = session.user_id.clone().unwrap_or(0);
    
        self.remove_session(session);

        let session_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.session
                ( id, user_id)
            VALUES
                (:id, :user_id)",
            params!{
                "id" => &session_key,
                "user_id" => &user_id
            });

        match session_result {
            Err(e) => logger::warn(format!("{}", e)),
            Ok(_) => logger::info(format!("session {} stored in db with user {}", 
                &session_key, &user_id))
        }

    }

    pub fn get_session(&self, session_key: &str) -> Option<SessionModel> {
        let result = self.pool.prep_exec(r"SELECT id, user_id FROM pusoy_dos.session
                                WHERE id = :id",
                params!{
                    "id" => session_key
                });
        let session_key = Uuid::parse_str(session_key).unwrap();
        match result {
            Ok(mut r) => {
                let row = r.next();
                Some(
                    SessionModel{
                        key: session_key,
                        user_id: row.unwrap().unwrap().get("user_id")
                    }
                )
            },
            Err(e) => {
                logger::warn(e);    
                None
            }
        }
    }

    fn remove_session(&self, session: &SessionModel) {
        let session_key = format!("{}", session.key.clone());
        let _ = self.pool.prep_exec(r"DELETE FROM pusoy_dos.session WHERE id = :id;",
            params!{
                "id" => &session_key
            });

    }
}
