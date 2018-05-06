#[derive(Clone, Deserialize, Serialize, StateData)]
pub struct Session {
    user_id: Option<usize>
}
