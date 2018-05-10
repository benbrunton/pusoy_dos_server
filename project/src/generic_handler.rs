use mime;
use hyper::header::Location;
use hyper::StatusCode;
use gotham::state::{FromState, State};
use gotham::middleware::session::SessionData;
use gotham::handler::{NewHandler, Handler, HandlerFuture};
use gotham::http::response::create_response;
use futures::future;

use std::io;
use model::Session;
use controller::{Controller, ResponseType};
use std::panic::RefUnwindSafe;
use std::sync::Arc;

#[derive(Clone)]
pub struct GenericHandler {
    controller: Arc<(Controller + Sync + Send + RefUnwindSafe)>
}

impl GenericHandler {
    pub fn new(controller: Arc<(Controller + Sync + Send + RefUnwindSafe)>) -> GenericHandler {
        GenericHandler {
            controller
        }
    }

    fn get_response(&self, session: &mut Option<Session>) -> ResponseType {
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
        use controller::ResponseType::*;
        let full_response = {
            let session: &mut Option<Session> 
                = SessionData::<Option<Session>>::borrow_mut_from(&mut state);
            self.get_response(session)
        };

        let res = match full_response {
            PageResponse(body) => {
                create_response(
                    &state,
                    StatusCode::Ok,
                    Some((body.as_bytes()
                            .to_vec(),
                        mime::TEXT_HTML)),
                )

            },
            Redirect(uri) => {
                let mut r = create_response(
                    &state,
                    StatusCode::Found,
                    None
                );
                {
                    let mut headers = r.headers_mut();
                    headers.set(Location::new(uri));
                }
                r
            }
        };

        Box::new(future::ok((state, res)))
    }
}
