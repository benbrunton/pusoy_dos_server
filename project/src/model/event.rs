
pub enum EventBody{
    Generic(String),
    Hand
}

pub struct Event{
    id: Option<u64>,
    game: Option<u64>,
    event_type: u64,
    body: EventBody
}
