use model::Session;
use gotham::state::State;
use gotham::router::response::extender::StaticResponseExtender;
use helpers::PathExtractor;

pub enum ResponseType {
    PageResponse(String),
    Redirect(String),
    ServerError,
}

pub trait Controller {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        body: Option<String>,
        path: Option<PathExtractor>,
    ) -> ResponseType;
}
