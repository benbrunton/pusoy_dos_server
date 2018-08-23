use std::panic::RefUnwindSafe;
use data_encoding::BASE64;
use model::Session;
use controller::{Controller, ResponseType};
use helpers::{PathExtractor, QueryStringExtractor};
use url::form_urlencoded::parse;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use data_access::game::Game as GameData;
use helpers;

pub struct RemoveUserController {
    game_data: GameData
}

impl RemoveUserController {
    pub fn new(game_data: GameData) -> RemoveUserController {
        RemoveUserController{ game_data }
    }

    fn remove_user(&self, user: u64, id:u64) {
        // TODO - validate that you have permission
        // and that the game hasn't started
        self.game_data.remove_user(user, id);

        info!("user {} removed from game {}", user, id);
    }
}

impl Controller for RemoveUserController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        body: Option<String>,
        path: Option<PathExtractor>,
        _qs: Option<QueryStringExtractor>
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session)
                .expect("unable to unwrap user id") as u64;


            let (game_id, user_id) = {
                let path = path.expect("unable to extract path");
                (path.id as u64, path.user.expect("unable to extract user") as u64)
            };
            self.remove_user(user_id, game_id);
            return ResponseType::Redirect(format!("/game/{}", game_id))
/*
 *          TODO - csrf token (audit other endpoints too)
 *

            let protect = AesGcmCsrfProtection::from_key(*b"01234567012345670123456701234567");
            let session_token = self.get_session_token(session)
                .as_bytes()
                .to_owned();
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
                    
                    let decoded_token = BASE64.decode(&token_bytes).expect("token not base64");
                    let decoded_cookie = BASE64.decode(&session_token).expect("token not base64");
                    let parsed_token = protect.parse_token(&decoded_token)
                        .expect("token not parsed");
                    let parsed_cookie = protect.parse_cookie(&decoded_cookie)
                        .expect("cookie not parsed");
                    if protect.verify_token_pair(&parsed_token, &parsed_cookie) {
                        self.insert_new_game(id, move_duration, decks);
                    }
                },
                _ => ()
            }
            
*/
        }

        ResponseType::Redirect("/".to_string())
    }
}

impl RefUnwindSafe for RemoveUserController {}
