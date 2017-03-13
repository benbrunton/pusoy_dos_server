use chrono::prelude::*;
use std::collections::BTreeMap;

pub struct Event{
    id: Option<u64>,
    game: Option<u64>,
    body: String,
    time: DateTime<UTC>
}

impl Event{
    pub fn new(id: Option<u64>, game: Option<u64>, message: String, datetime: DateTime<UTC>) ->  Event {
        Event{
            id: None,
            game: game,
            body: message,
            time: datetime
        }
    }

    pub fn display(&self) -> BTreeMap<String, String> {
        let mut output = BTreeMap::new();
        output.insert(String::from("body"), self.body.to_owned());
        output.insert(String::from("time"), format!("{}", self.time.format("%Y-%m-%d %H:%M:%S")));

        output
    }
}
