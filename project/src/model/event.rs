use chrono::prelude::*;
use model::user::User;
use std::collections::BTreeMap;

use serde::ser::{Serialize, Serializer, SerializeMap};

#[derive(Debug)]
pub struct Event{
    id: Option<u64>,
    user: Option<User>,
    game: Option<u64>,
    body: String,
    time: DateTime<Utc>
}

impl Event{
    pub fn new(id: Option<u64>, 
                user: Option<User>, 
                game: Option<u64>, 
                message: String, 
                datetime: DateTime<Utc>) ->  Event {

        Event{
            id,
            user,
            game,
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
        format!("{}", self.time.format("%H:%M %d/%m/%Y"))
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

impl Serialize for Event {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {

        let mut map = try!(serializer.serialize_map(Some(2)));
        
		try!(map.serialize_entry("id", &self.id));

		try!(map.serialize_entry("user_name", &self.user.to_owned().unwrap().name.clone()));
        
		try!(map.serialize_entry("body", &self.get_message()));

        let time = format!("{}", self.time.format("%Y-%m-%d %H:%M:%S"));
		try!(map.serialize_entry("time", &time));

        map.end()

    }
}


