use serde::ser::{Serialize, Serializer, SerializeMap};

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
        let mut map = try!(serializer.serialize_map(Some(2)));
		try!(map.serialize_entry("id", &self.id));
		try!(map.serialize_entry("name", &self.name));
		try!(map.serialize_entry("provider_id", &self.provider_id));
        try!(map.serialize_entry("provider_type", &self.provider_type));
        try!(map.serialize_entry("creation_date", &self.creation_date));

        map.end()
    }
}
