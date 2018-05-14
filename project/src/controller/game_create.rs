use std::collections::HashMap;
use std::panic::RefUnwindSafe;
use data_encoding::BASE64;
use model::Session;
use data_access::game::Game as GameData;
use controller::{Controller, ResponseType};
use helpers;
use gotham::state::State;
use url::form_urlencoded::parse;
use csrf::{AesGcmCsrfProtection, CsrfProtection};

pub struct GameCreateController {
    game_data: GameData
}

impl GameCreateController {
    pub fn new(game_data: GameData) -> GameCreateController {
        GameCreateController{ game_data }
    }

    fn insert_new_game(
        &self,
        id: u64, 
        move_duration: u64,
        decks: u64,
    ) -> Result<(), String>{
        self.game_data.create_game(id, move_duration, 4, decks);
        Ok(())
    }

    fn get_session_token(&self, session: &mut Option<Session>) -> String {
        let sess = session.clone().unwrap();
        sess.csrf_token.expect("unable to find csrf token")
    }
}

impl Controller for GameCreateController {

    fn get_response(&self, session:&mut Option<Session>, body: Option<String>) -> ResponseType {
        
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session)
                .expect("unable to unwrap user id") as u64;
            let protect = AesGcmCsrfProtection::from_key(*b"01234567012345670123456701234567");
            let session_token = self.get_session_token(session)
                .as_bytes()
                .to_owned();
            let mut token_bytes = "".as_bytes().to_owned();
            let mut move_duration = 0;
            let mut decks = 0;
            info!("{:?}", body);
            match body {
                Some(b) => {
                    let parsed_body = parse(b.as_bytes());
                    let pairs = parsed_body.into_owned();
                    for (key, val) in pairs {
                        match key.as_ref() {
                            "_csrf_token" => {
                                token_bytes = val.as_bytes().to_owned();
                            },
                            "max-move-duration" => {
                                move_duration = val.parse::<u64>().unwrap().to_owned();
                            },
                            "decks" => {
                                decks = val.parse::<u64>().unwrap().to_owned();
                            },
                            _ => ()
                        }
                    }
                    
                    info!("{:?}", token_bytes);
                    let decoded_token = BASE64.decode(&token_bytes).expect("token not base64");
                    let decoded_cookie = BASE64.decode(&session_token).expect("token not base64");
                    let parsed_token = protect.parse_token(&decoded_token)
                        .expect("token not parsed");
                    let parsed_cookie = protect.parse_cookie(&decoded_cookie)
                        .expect("cookie not parsed");
                    if protect.verify_token_pair(&parsed_token, &parsed_cookie) {
                        self.insert_new_game(id, move_duration, decks);
                        return ResponseType::Redirect("/games".to_string())
                    }
                },
                _ => ()
            }
            
        }

        ResponseType::Redirect("/".to_string())
    }
}

impl RefUnwindSafe for GameCreateController {}
