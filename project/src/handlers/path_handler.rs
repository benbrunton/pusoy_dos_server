use hyper::Body;
use gotham::state::{FromState, State};
use gotham::middleware::session::SessionData;
use gotham::handler::{NewHandler, Handler, HandlerFuture};
use futures::{Stream, future, Future};

use std::io;
use model::Session;
use controller::{Controller, ResponseType};
use std::panic::RefUnwindSafe;
use std::sync::Arc;
use handlers::GenericHandler;
use helpers::{PathExtractor};

#[derive(Clone)]
pub struct PathHandler {
    controller: Arc<(Controller + Sync + Send + RefUnwindSafe)>
}

impl PathHandler {
    pub fn new(controller: Arc<(Controller + Sync + Send + RefUnwindSafe)>) -> PathHandler {
        PathHandler {
            controller
        }
    }

    fn get_response(
        &self,
        session: &mut Option<Session>,
        body: Option<String>,
        path: Option<PathExtractor>,
    ) -> ResponseType {
        self.controller.get_response(session, body, path, None)
    }
}

impl NewHandler for PathHandler {
    type Instance = Self;

    fn new_handler(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl Handler for PathHandler {

    fn handle(self, mut state: State) -> Box<HandlerFuture> {
        let bod = {
            Body::take_from(&mut state)
        };

        let f = bod.concat2()
            .then(move |full_body| {

                let response_type = {
                    
                    let path = {
                        PathExtractor::take_from(&mut state)
                    };

                    let session: &mut Option<Session> = {
                        SessionData::<Option<Session>>::borrow_mut_from(&mut state)
                    };

                    let req_body = match full_body {
                        Ok(valid_body) => {
                            let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                            Some(body_content)
                        }
                        Err(_) => None
                    };

                    self.get_response(session, req_body, Some(path))
                };

                let res = {
                    GenericHandler::create_handler_future(&mut state, response_type)
                };
                future::ok((state, res))
            });

        Box::new(f)
    }
}
