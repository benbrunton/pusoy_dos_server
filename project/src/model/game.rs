use serde::ser::{Serialize, Serializer, SerializeMap};

#[derive(Debug)]
pub struct Game{
    pub id: u64,
    pub creator_id: u64,
	pub creator_name: String,
    pub started: bool,
    pub next_player_name: Option<String>,
    pub next_player_id: Option<u64>,
    pub num_players: u64,
    pub max_move_duration: String,
    pub max_move_duration_mins: u64,
    pub decks: u64
}

impl Serialize for Game {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {

        let started = if self.started { 1 } else { 0 };
        let mut map = try!(serializer.serialize_map(Some(2)));

        let next_name = match &self.next_player_name {
            &Some(ref n) => n.clone(),
            _       => String::from("none")
        };
		try!(map.serialize_entry("id", &self.id));
		try!(map.serialize_entry("creator_id", &self.creator_id));
		try!(map.serialize_entry("creator_name", &self.creator_name));
        try!(map.serialize_entry("started", &started));
        try!(map.serialize_entry("next_player_name", &next_name));
        try!(map.serialize_entry("next_player_id", 
            &self.next_player_id.unwrap_or(0)));
        try!(map.serialize_entry("num_players", &self.num_players));
        try!(map.serialize_entry("max_move_duration", &self.max_move_duration));
        try!(map.serialize_entry("max_move_duration_mins", &self.max_move_duration_mins));
        try!(map.serialize_entry("decks", &self.decks));

        map.end()
    }
}
