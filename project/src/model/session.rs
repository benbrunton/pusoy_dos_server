#[derive(Clone, Deserialize, Serialize, StateData)]
pub struct Session {
    pub user_id: Option<usize>,
    pub csrf_token: Option<String>
}
