use hyper::{Response, StatusCode};
use hyper::header::Location;
use tera::{Tera, Context, TeraResult};
use mime;
use gotham::http::response::create_response;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham::middleware::session::{NewSessionMiddleware, SessionData};
use gotham::handler::{NewHandler, Handler, HandlerFuture};
use futures::{future, Future};

use std::io;
use model::Session;
use controller::{Controller, ResponseType};

#[derive(Clone)]
pub struct GenericHandler {
    controller: Controller
}

impl GenericHandler {
    pub fn new(controller: &Controller) -> GenericHandler {
        GenericHandler {
            controller
        }
    }

    fn get_response(&self, session:Option<Session>) -> ResponseType {
        self.controller.get_response(session)
    }

}

impl NewHandler for GenericHandler {
    type Instance = Self;

    fn new_handler(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl Handler for GenericHandler {

    fn handle(self, mut state: State) -> Box<HandlerFuture> {
		let maybe_session = {
			let session_data: &Option<Session> = SessionData::<Option<Session>>::borrow_from(&state);
			session_data.clone()
		};

        let full_response = self.get_response(maybe_session);

        let res = match full_response {
            PageResponse(body) => {
                create_response(
                    &state,
                    StatusCode::Ok,
                    Some((body.as_bytes()
                            .to_vec(),
                        mime::TEXT_HTML)),

            },
            Redirect(uri) => {
                let mut r = create_response(
                    &state,
                    StatusCode::Found,
                    None
                );
                let mut headers = r.headers_mut();
                headers.set(Location::new(uri));
                r
            }
        }

        Box::new(future::ok((state, res)))
    }
}
