use model::Session;
use gotham::state::State;

pub enum ResponseType {
    PageResponse(String),
    Redirect(String)
}

pub trait Controller {
    fn get_response(&self, session:&mut Option<Session>, body: Option<String>) -> ResponseType;
}
