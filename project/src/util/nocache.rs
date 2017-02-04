use iron::{AfterMiddleware, IronResult, Request, Response};
use hyper::header::{CacheControl, CacheDirective, Expires, HttpDate, Pragma};
use time;
 
pub struct NoCacheMiddleware;

impl AfterMiddleware for NoCacheMiddleware {

	fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut res = Response::new();
		res.status = r.status;
		res.body = r.body;
		res.headers = r.headers;
		res.headers.set(
			CacheControl(vec![
				CacheDirective::NoCache,
				CacheDirective::NoStore,
				CacheDirective::MustRevalidate
			])
		);

 //    headers.set(Expires(HttpDate(time::now() + Duration::days(1))));
        res.headers.set(Expires(HttpDate(time::now())));
        res.headers.set(Pragma::NoCache);

        Ok(res)
    }

}
