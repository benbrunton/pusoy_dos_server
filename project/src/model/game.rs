use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug, Deserialize)]
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
        let mut state = try!(serializer.serialize_struct("Game", 10));

        let next_name = match &self.next_player_name {
            &Some(ref n) => n.clone(),
            _       => String::from("none")
        };
		try!(state.serialize_field("id", &self.id));
		try!(state.serialize_field("creator_id", &self.creator_id));
		try!(state.serialize_field("creator_name", &self.creator_name));
        try!(state.serialize_field("started", &started));
        try!(state.serialize_field("next_player_name", &next_name));
        try!(state.serialize_field("next_player_id", 
            &self.next_player_id.unwrap_or(0)));
        try!(state.serialize_field("num_players", &self.num_players));
        try!(state.serialize_field("max_move_duration", &self.max_move_duration));
        try!(state.serialize_field("max_move_duration_mins", &self.max_move_duration_mins));
        try!(state.serialize_field("decks", &self.decks));

        state.end()
    }
}
