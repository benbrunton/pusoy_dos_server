use serde::{Serialize, Serializer};
use chrono::prelude::*;
use time::Duration;

#[derive(Debug)]
pub struct TimeLimit{
    pub game: u64,
    pub status: Option<Duration>
}

impl Serialize for TimeLimit {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {

        let mut state = try!(serializer.serialize_map(Some(2)));

        try!(serializer.serialize_map_key(&mut state, "game"));
        try!(serializer.serialize_map_value(&mut state, &self.game));
        let status = match self.status{
            None => String::from("null"),
            Some(dt) => duration_to_string(dt)
        };

        let mins = match self.status{
           None => 0,
           Some(dt) => dt.num_minutes()
        };
        try!(serializer.serialize_map_key(&mut state, "status"));
        try!(serializer.serialize_map_value(&mut state, status));
        try!(serializer.serialize_map_key(&mut state, "minutes"));
        try!(serializer.serialize_map_value(&mut state, mins));

        serializer.serialize_map_end(state)
    }
}

fn duration_to_string(dt: Duration) -> String {
    let hours = dt.num_hours();
    let mut mins = dt.num_minutes();

    while mins > 60 {
        mins -= 60;
    }

    let hours_txt = match hours {
        0 => String::from(""),
        _ => format!("{} hours", hours)
    };

    let connect = if hours > 0 && mins > 0 {
        String::from(", ")
    } else {
        String::from("")
    };

    let mins_txt = match mins {
        0 => String::from(""),
        _ => format!("{} mins", mins)
    };

    format!("{}{}{}",
        hours_txt, connect, mins_txt)

}
