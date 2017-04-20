use serde::{Serialize, Serializer};

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

	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {

        let started = if self.started { 1 } else { 0 };
        let mut state = try!(serializer.serialize_map(Some(2)));

        let next_name = match &self.next_player_name {
            &Some(ref n) => n.clone(),
            _       => String::from("none")
        };
		try!(serializer.serialize_map_key(&mut state, "id"));
		try!(serializer.serialize_map_value(&mut state, self.id));
		try!(serializer.serialize_map_key(&mut state, "creator_id"));
		try!(serializer.serialize_map_value(&mut state, self.creator_id));
		try!(serializer.serialize_map_key(&mut state, "creator_name"));
		try!(serializer.serialize_map_value(&mut state, &self.creator_name));
        try!(serializer.serialize_map_key(&mut state, "started"));
        try!(serializer.serialize_map_value(&mut state, started));
        try!(serializer.serialize_map_key(&mut state, "next_player_name"));
        try!(serializer.serialize_map_value(&mut state, &next_name));
        try!(serializer.serialize_map_key(&mut state, "next_player_id"));
        try!(serializer.serialize_map_value(&mut state, 
                self.next_player_id.unwrap_or(0)));
        try!(serializer.serialize_map_key(&mut state, "num_players"));
        try!(serializer.serialize_map_value(&mut state, self.num_players));
        try!(serializer.serialize_map_key(&mut state, "max_move_duration"));
        try!(serializer.serialize_map_value(&mut state, &self.max_move_duration));
        try!(serializer.serialize_map_key(&mut state, "max_move_duration_mins"));
        try!(serializer.serialize_map_value(&mut state, &self.max_move_duration_mins));

        try!(serializer.serialize_map_key(&mut state, "decks"));
        try!(serializer.serialize_map_value(&mut state, self.decks));

        serializer.serialize_map_end(state)
    }
}
