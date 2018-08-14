use hyper::{Body};
use gotham::state::{FromState, State};
use gotham::middleware::session::SessionData;
use gotham::handler::{NewHandler, Handler, HandlerFuture};
use futures::{Stream, future, Future};

use std::io;
use model::Session;
use controller::{Controller, ResponseType};
use std::panic::RefUnwindSafe;
use std::sync::Arc;
use helpers::QueryStringExtractor;
use handlers::GenericHandler;

#[derive(Clone)]
pub struct QueryStringHandler {
    controller: Arc<(Controller + Sync + Send + RefUnwindSafe)>
}

impl QueryStringHandler {
    pub fn new(controller: Arc<(Controller + Sync + Send + RefUnwindSafe)>) -> QueryStringHandler {
        QueryStringHandler {
            controller
        }
    }

    fn get_response(
        &self,
        session: &mut Option<Session>,
        body: Option<String>,
        qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        self.controller.get_response(session, body, None, qs)
    }

}

impl NewHandler for QueryStringHandler {
    type Instance = Self;

    fn new_handler(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl Handler for QueryStringHandler {

    fn handle(self, mut state: State) -> Box<HandlerFuture> {
        let bod = {
            Body::take_from(&mut state)
        };

        let f = bod.concat2()
            .then(move |full_body| {

                let response_type = {
                    let qs = {
                        QueryStringExtractor::take_from(&mut state)
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

                    self.get_response(session, req_body, Some(qs))
                };

                let res = {
                    GenericHandler::create_handler_future(&mut state, response_type)
                };
                future::ok((state, res))
            });

        Box::new(f)
    }
}
