use mysql;

#[derive(Clone)]
pub struct Round{
    pool: mysql::Pool 
}

impl Round {

    pub fn new(pool: mysql::Pool) -> Round {
        Round {
            pool: pool
        }
    }

    pub fn create_round(&self, user:u64) {

        let query_result = self.pool.prep_exec(r"INSERT INTO pusoy_dos.round
              (  game,
                 hands,
                 current_player,
                 last_move,
                 pass_count,
                 first_round,
            )
            VALUES
                ( :game, 
                  :hands, 
                  :current_player, 
                  :last_move,
                  :pass_count,
                  :first_round
                )",
            params!{
                "user" => user
            }).unwrap();

         let new_game = query_result.last_insert_id();

    }


}

