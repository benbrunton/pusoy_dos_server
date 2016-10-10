// before routing - create or retrieve a session using session cookie
// all controllers should have access to the session to retrieve login status
// and user info

use iron::{BeforeMiddleware, IronResult, IronError, Request};
use iron::typemap::Key;
use hyper::header::Cookie;
use cookie::Cookie as CookiePair;
use uuid::Uuid;

use logger;

pub struct SessionKey{ val: Uuid }
impl Key for SessionKey { type Value = Uuid; }

#[derive(Debug)]
pub struct Session{

}

impl <'a> Session {

    pub fn new() -> Session{
        Session{}
    }

    fn get_cookie(req: &'a mut Request) -> Option<&'a Cookie>{
        req.headers.get::<Cookie>()
    }

    fn get_session_key(req: &mut Request) -> SessionKey {
        match Session::get_cookie(req) {
            None => Session::generate_session_key(),
            Some(&Cookie(ref cookies)) => {
                Session::find_session_cookie(cookies)
            }
        }
    }

    fn generate_session_key() -> SessionKey{
        SessionKey{val: Uuid::new_v4() }
    }

    fn find_session_cookie(cookies: &Vec<CookiePair>) -> SessionKey {
        Session::generate_session_key()
    }
}


impl Key for Session {type Value = Session;}

impl BeforeMiddleware for Session {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let key = Session::get_session_key(req);
        let session = Session::new();
        logger::info(format!("{:?}", key.val));
        req.extensions.insert::<SessionKey>(key.val);
        req.extensions.insert::<Session>(session);
        Ok(())
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<()> {
        logger::info(format!("{:?}", req.headers)); 
        Err(err)
    }
}

