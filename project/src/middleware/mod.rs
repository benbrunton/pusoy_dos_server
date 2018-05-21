use gotham::state::State;
use gotham::middleware::Middleware;
use gotham::handler::HandlerFuture;
use hyper::header::{CacheControl, CacheDirective, Expires, Pragma};
use std::time::SystemTime;
use futures::Future;

#[derive(NewMiddleware, Copy, Clone)]
struct MiddlewareAddingResponseHeader;

impl Middleware for MiddlewareAddingResponseHeader {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
        where Chain: FnOnce(State) -> Box<HandlerFuture> + 'static
    {
        let f = chain(state)
            .map(|(state, mut response)| {
                {
                    let resp = response.headers_mut();
                    resp.set(
                        CacheControl(vec![
                            CacheDirective::NoCache,
                            CacheDirective::NoStore,
                            CacheDirective::MustRevalidate
                        ])
                    );
                    resp.set(Expires(SystemTime::now().into()));
                    resp.set(Pragma::NoCache);
                }

                (state, response)
            });

        Box::new(f)
    }
}
