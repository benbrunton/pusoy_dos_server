use chrono::prelude::*;
use model::user::User;
use std::collections::BTreeMap;

pub struct Event{
    id: Option<u64>,
    user: Option<User>,
    game: Option<u64>,
    body: String,
    time: DateTime<UTC>
}

impl Event{
    pub fn new(id: Option<u64>, 
                user: Option<User>, 
                game: Option<u64>, 
                message: String, 
                datetime: DateTime<UTC>) ->  Event {

        Event{
            id: None,
            user: user,
            game: game,
            body: message,
            time: datetime
        }
    }

    pub fn match_user_id(&self, id:u64) -> bool {
        match self.user {
            Some(ref user)  => id == user.id,
            _           => false 
        }
    }

    pub fn get_message(&self) -> String {
        self.body.to_owned()
    }

    pub fn get_time(&self) -> String {
        format!("{}", self.time.format("%Y-%m-%d %H:%M:%S"))
    }

    pub fn display(&self) -> BTreeMap<String, String> {
        let user_name = match &self.user {
            &Some(ref user)  => user.name.clone(),
            _               => String::from("Unknown Player")
        };

        let user_id = match &self.user {
            &Some(ref user)  => user.id,
            _               => 0
        };

        let mut output = BTreeMap::new();
        output.insert(String::from("body"), self.body.to_owned());
        output.insert(String::from("user_name"), user_name);
        output.insert(String::from("user_id"), format!("{}", user_id));
        output.insert(String::from("time"), format!("{}", self.time.format("%Y-%m-%d %H:%M:%S")));

        output
    }
}
