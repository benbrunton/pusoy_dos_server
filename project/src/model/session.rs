#[derive(Clone, Deserialize, Serialize, StateData)]
pub struct Session {
    pub user_id: Option<usize>
}
