use mime;
use hyper::header::Location;
use hyper::{StatusCode, Body, Response};
use gotham::state::{FromState, State};
use gotham::middleware::session::SessionData;
use gotham::handler::{NewHandler, Handler, HandlerFuture};
use gotham::http::response::create_response;
use futures::{Stream, future, Future};

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

    fn get_response(&self, session: &mut Option<Session>, body: Option<String>) -> ResponseType {
        self.controller.get_response(session, body, None)
    }

    pub fn create_handler_future(state: &mut State, full_response: ResponseType) -> Response {
        use controller::ResponseType::*;
        match full_response {
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
            },
            Json(body) => {
                create_response(
                    &state,
                    StatusCode::Ok,
                    Some((body.as_bytes()
                            .to_vec(),
                        mime::APPLICATION_JSON)),
                )
            },
            ServerError => {
                create_response(
                    &state,
                    StatusCode::InternalServerError,
                    None
                )
            }
        }
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
        let bod = {
            Body::take_from(&mut state)
        };

        let f = bod.concat2()
            .then(move |full_body| {

                let response_type = {
                    let session: &mut Option<Session> 
                        = SessionData::<Option<Session>>::borrow_mut_from(&mut state);
                    
                    let req_body = match full_body {
                        Ok(valid_body) => {
                            let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                            Some(body_content)
                        }
                        Err(_) => None
                    };

                    self.get_response(session, req_body)
                };

                let res = {
                    Self::create_handler_future(&mut state, response_type)
                };
                future::ok((state, res))
            });

        Box::new(f)
    }
}
