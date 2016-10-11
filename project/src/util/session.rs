// before routing - create or retrieve a session using session cookie
// all controllers should have access to the session to retrieve login status
// and user info

use iron::{BeforeMiddleware, AfterMiddleware, IronResult, IronError, Request, Response};
use iron::typemap::Key;
use hyper::header::{Cookie, Headers, SetCookie};
use cookie::Cookie as CookiePair;
use uuid::Uuid;

use logger;

pub struct SessionKey{ val: Uuid }
impl Key for SessionKey { type Value = Uuid; }

#[derive(Debug)]
pub struct Session{
    key: Uuid,
    userId: Option<u64>
}

impl Session {

    pub fn new(key: Uuid, userId:Option<u64>) -> Session{
        Session{
            key: key,
            user_id: None 
        }
    }

    pub fn set_user(&mut self, id: u64){
        self.user_id = Some(id);
    }
}

impl Key for Session {type Value = Session;}

#[derive(Clone)]
pub struct SessionMiddleware;
impl <'a> SessionMiddleware{

    // creates a session from key
    // returning from db or inserting new
    fn build_session(&self, key:Uuid) -> Session{
        Session::new(key, None)
    }

    fn get_cookie(req: &'a mut Request) -> Option<&'a Cookie>{
        req.headers.get::<Cookie>()
    }

    fn get_session_key(req: &mut Request) -> SessionKey {
        match SessionMiddleware::get_cookie(req) {
            None => SessionMiddleware::generate_session_key(),
            Some(&Cookie(ref cookies)) => {
                SessionMiddleware::find_session_cookie(cookies)
            }
        }
    }

    fn generate_session_key() -> SessionKey{
        SessionKey{val: Uuid::new_v4() }
    }

    fn find_session_cookie(cookies: &Vec<CookiePair>) -> SessionKey {
		for pair in cookies {
			if pair.name == "pd_session" {
                return SessionKey{ val: Uuid::parse_str(&pair.value).unwrap() };
            }
		}

		SessionMiddleware::generate_session_key()
    }

}

impl BeforeMiddleware for SessionMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let key = SessionMiddleware::get_session_key(req);
        let session = self.build_session(key.val);
        req.extensions.insert::<SessionKey>(key.val);
        req.extensions.insert::<Session>(session);
        Ok(())
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<()> {
        logger::info(format!("{:?}", req.headers)); 
        Err(err)
    }
}

impl AfterMiddleware for SessionMiddleware {

	fn after(&self, req: &mut Request, r: Response) -> IronResult<Response> {
		let key = req.extensions.get::<SessionKey>().unwrap();
		let str_key = format!("{}", key);
		let cookie = CookiePair::new("pd_session".to_owned(), str_key);
		let mut res = Response::new();
		res.status = r.status;
		res.body = r.body;
		res.headers = r.headers;
		res.headers.set(
			SetCookie(vec![
				cookie
			])
		);

        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        //try!(self.log(req, &err.response));
        Err(err)
    }

}
