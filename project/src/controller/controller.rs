pub enum ResponseType {
    PageResponse(String),
    Redirect(String)
}

pub trait Controller {
    fn get_response(&self, session:Option<Session>) -> ResponseType
}
