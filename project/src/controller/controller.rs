use model::Session;
use helpers::{PathExtractor, QueryStringExtractor};

pub enum ResponseType {
    PageResponse(String),
    Redirect(String),
    Json(String),
    ServerError,
}

pub trait Controller {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        body: Option<String>,
        path: Option<PathExtractor>,
        qs: Option<QueryStringExtractor>,
    ) -> ResponseType;
}
