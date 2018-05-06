use serde::{Serialize, Serializer};

#[derive(Clone)]
pub struct PartUser{
    pub name: String,
    pub provider_id: String,
    pub provider_type: String
}

#[derive(Clone, Debug)]
pub struct User{
    pub id: u64,
    pub name: String,
    pub provider_id: String,
    pub provider_type: String,
    pub creation_date: String
}

impl Serialize for User {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_map(Some(2)));
		try!(serializer.serialize_map_key(&mut state, "id"));
		try!(serializer.serialize_map_value(&mut state, self.id));
		try!(serializer.serialize_map_key(&mut state, "name"));
		try!(serializer.serialize_map_value(&mut state, &self.name));
		try!(serializer.serialize_map_key(&mut state, "provider_id"));
		try!(serializer.serialize_map_value(&mut state, &self.provider_id));
        try!(serializer.serialize_map_key(&mut state, "provider_type"));
        try!(serializer.serialize_map_value(&mut state, &self.provider_type));
        try!(serializer.serialize_map_key(&mut state, "creation_date"));
        try!(serializer.serialize_map_value(&mut state, &self.creation_date));


        serializer.serialize_map_end(state)
    }
}
