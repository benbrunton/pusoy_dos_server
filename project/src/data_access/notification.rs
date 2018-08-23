use mysql;

#[derive(Clone)]
pub struct Notification {
    pool: mysql::Pool
}

impl Notification {
    pub fn new(pool: mysql::Pool) -> Notification {
        Notification {
            pool: pool
        }
    }

    pub fn update_push_subscription(&self, user: u64, subscription: String) {
        let a = self.pool.prep_exec(r"INSERT INTO pusoy_dos.notifications
                ( user, subscription )
            VALUES
                (:user, :subscription)
            ON DUPLICATE KEY UPDATE
                subscription = VALUES(subscription)",
            params!{
                "user" => user,
                "subscription" => subscription
            }).unwrap();

        //info!("{:?}", a);
    }

    pub fn get_user_subscription(&self, user: u64) -> Option<String> {
        let mut user_sub = self.pool.prep_exec(r"SELECT subscription
            FROM pusoy_dos.notifications
            WHERE user = :user",
            params!{
                "user" => user
            }).unwrap();

        let retrieved_sub = match user_sub.next() {
            Some(result) => {
                let mut row = result.unwrap();
                Some(row.take("subscription").unwrap())
            },
            _ => {
                info!("No subscription found for {}", user);
                None
            }
        };

        retrieved_sub
    }

}
