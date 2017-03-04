
pub enum EventBody{
    Generic(String),
    Hand
}

pub struct Event{
    id: Option<u64>,
    game: Option<u64>,
    type: u64,
    body: EventBody
}
