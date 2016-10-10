
#[derive(Clone)]
pub struct PartUser{
    pub name: String,
    pub provider_id: String,
    pub provider_type: String
}

#[derive(Clone)]
pub struct User{
    pub id: u64,
    pub name: String,
    pub provider_id: String,
    pub provider_type: String,
    pub creation_date: String
}
