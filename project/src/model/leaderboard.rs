#[derive(Debug, Serialize, Deserialize)]
pub struct Leaderboard{
    pub name: String,
	pub position: u64,
    pub wins: u64,
    pub played: u64,
    pub losses: u64,
    pub rating: f64
}

/*
impl Serialize for Leaderboard {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {

        let rating = format!("{:.2}", self.rating);

        let mut state = try!(serializer.serialize_map(Some(2)));
        try!(serializer.serialize_map_key(&mut state, "name"));
		try!(serializer.serialize_map_value(&mut state, &self.name));
        try!(serializer.serialize_map_key(&mut state, "position"));
		try!(serializer.serialize_map_value(&mut state, self.position));
        try!(serializer.serialize_map_key(&mut state, "wins"));
		try!(serializer.serialize_map_value(&mut state, self.wins));
        try!(serializer.serialize_map_key(&mut state, "played"));
		try!(serializer.serialize_map_value(&mut state, self.played));
        try!(serializer.serialize_map_key(&mut state, "losses"));
		try!(serializer.serialize_map_value(&mut state, self.losses));

        try!(serializer.serialize_map_key(&mut state, "rating"));
		try!(serializer.serialize_map_value(&mut state, rating));

        serializer.serialize_map_end(state)
    }
}
*/
