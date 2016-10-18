// before routing - create or retrieve a session using session cookie
// all controllers should have access to the session to retrieve login status
// and user info

use iron::{BeforeMiddleware, AfterMiddleware, IronResult, IronError, Request, Response};
use iron::typemap::Key;
use hyper::header::{Cookie, SetCookie};
use cookie::Cookie as CookiePair;
use uuid::Uuid;

use logger;
use data_access::session::Session as SessionStore;

pub struct SessionKey{ val: Uuid }
impl Key for SessionKey { type Value = Uuid; }

pub enum SessionInstruction {
    STORE,
    DELETE,
    NONE
}

impl Key for SessionInstruction { type Value = SessionInstruction; }

#[derive(Debug)]
pub struct Session{
    pub key: Uuid,
    pub user_id: Option<u64>
}

impl Session {

    pub fn new(key: Uuid, user_id:Option<u64>) -> Session{

        Session{
            key: key,
            user_id: user_id 
        }
    }

    pub fn set_user(&self, id: u64) -> Session{
        Session{
            key: self.key,
            user_id: Some(id)
        }
    }
}

impl Key for Session {type Value = Session;}

#[derive(Clone)]
pub struct SessionMiddleware{
    store: SessionStore    
}
impl <'a> SessionMiddleware{

    pub fn new(store: SessionStore) -> SessionMiddleware {
        SessionMiddleware{
            store: store
        }
    }

    // creates a session from key
    // returning from db or inserting new
    fn build_session(&self, key:Uuid) -> Session{
        logger::info(format!("building session from key: {}", key));
        let session_key = format!("{}", key);
        let result = self.store.get_session(&session_key);

        match result {
            Some(s) => s,
            None    => Session::new(key, None)
        }
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
                return SessionKey{ val: Uuid::parse_str(&pair.value).unwrap_or(Uuid::new_v4()) };
            }
		}

		SessionMiddleware::generate_session_key()
    }

    fn store_session(&self, session: &Session, key: &Uuid, r: Response) -> Response {
        self.store.store_session(session);
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

        res
    }

    fn delete_session(&self, r:Response) -> Response {
        let cookie = CookiePair::new("pd_session".to_owned(), String::from(""));
		let mut res = Response::new();
		res.status = r.status;
		res.body = r.body;
		res.headers = r.headers;
		res.headers.set(
			SetCookie(vec![
				cookie
			])
		);

        res

    }

}

impl BeforeMiddleware for SessionMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let key = SessionMiddleware::get_session_key(req);
        let session = self.build_session(key.val);
        let instruction = SessionInstruction::STORE;

        logger::info("inserting session data into request");
        req.extensions.insert::<SessionKey>(key.val);
        req.extensions.insert::<Session>(session);
        req.extensions.insert::<SessionInstruction>(instruction);
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
        let session = req.extensions.get::<Session>().unwrap();
        let instruction = req.extensions.get::<SessionInstruction>().unwrap();

        let res = match *instruction {
            SessionInstruction::STORE => self.store_session(session, key, r),
            SessionInstruction::DELETE => self.delete_session(r),
            _   => r
        };

        Ok(res)
    }

    fn catch(&self, /* req */ _: &mut Request, err: IronError) -> IronResult<Response> {
        //try!(self.log(req, &err.response));
        Err(err)
    }

}
