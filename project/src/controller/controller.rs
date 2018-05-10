use model::Session;

pub enum ResponseType {
    PageResponse(String),
    Redirect(String)
}

pub trait Controller {
    fn get_response(&self, session:&mut Option<Session>) -> ResponseType;
}
