use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use hyper::client::Client;
use hyper::header::Headers;

use std::collections::BTreeMap;
use serde_json;

use helpers;
use bodyparser;

use data_access::round::Round as RoundData;
use data_access::game::Game as GameData;
use data_access::event::Event as EventData;
use data_access::user::User as UserData;
use data_access::notification::Notification as NotificationData;

use pusoy_dos::game::game::Game;
use pusoy_dos::cards::types::*;
use pusoy_dos::cards::card::{ Card, PlayerCard };
use pusoy_dos::game::round::RoundDefinition;



#[derive(Clone)]
pub struct SubmitMove{
    round_data: RoundData,
    game_data: GameData,
    event_data: EventData,
    user_data: UserData,
    notification_data: NotificationData
}

impl SubmitMove {

    pub fn new(round_data: RoundData, game_data: GameData, event_data: EventData, user_data: UserData, notification_data: NotificationData) -> SubmitMove{
        SubmitMove{
            round_data: round_data,
            game_data: game_data,
            event_data: event_data,
            user_data: user_data,
            notification_data: notification_data
        }
    }

    pub fn execute(&self, user_id: u64,
                        id: u64,
                        json:Option<serde_json::Value>) -> Response {


        let round_result = self.round_data.get(id);
        match round_result {
            None => {
                info!("no round found for game {}", id);
                return self.output_error();
            },
            _ => ()
        }

        info!("loading game: {}", id);

        let round = round_result.expect("error with round result");
        let reversed = round.reversed;
        info!("game reversed: {:?}", reversed);
        let game = Game::load(round.clone()).expect("error loading game");
        info!("game loaded");


        let player_move = json.unwrap();
        let cards = self.get_cards(player_move, reversed);
        info!("{:?}", cards);

        let valid_move = game.player_move(user_id, cards.clone());

        match valid_move {
            Ok(updated_game) => {
                info!("valid move - updating game");
                self.round_data.update_round(id, updated_game.clone());
                let played_cards = helpers::convert_vec_to_display_cards(cards.clone());
                let event_descr = serde_json::to_string(&played_cards).unwrap();
                self.event_data.insert_game_event(user_id, id, event_descr);

                let updated_round = updated_game.round.export();

                if updated_round.players.len() < 2 {
                    let _ = self.game_data.complete_game(id);
                    //TODO Send game ended notification
                } else {
                    self.notify_next_player(updated_round, user_id, cards, id);
                }
            },
            _ => {
                info!("invalid_move!");
                return self.output_error();
            }
        }

        self.output_success()
    }

    fn notify_next_player(
        &self,
        updated_round: RoundDefinition,
        user_id: u64,
        cards: Vec<PlayerCard>,
        id: u64
    ) {
        let user_sub = self.notification_data.get_user_subscription(updated_round.current_player);
        match user_sub {
            Some(subscription) => {
                let client = Client::new();
                let mut body = BTreeMap::new();
                let player = match self.user_data.get_username_from_id(user_id) {
                    Some(username) => username,
                    _ => "".to_owned()
                };

                let cards_played = helpers::cards_played_summary(cards.clone());

                body.insert("subscription", subscription);
                body.insert("title", format!("Your move in game #{}", id));
                body.insert("body", format!("{} played {}", player, cards_played));
                body.insert("data", format!("{{ \"game\": {} }}", id));
                body.insert("tag", "moves".to_owned());

                let mut headers = Headers::new();
                headers.set_raw("content-type", vec!(b"application/json".to_vec()));
                let res = client.post("http://localhost:8888")
                    .body(&json::encode(&body).unwrap())
                    .headers(headers)
                    .send();
            },
            _ => {
                //No sub do nothing
            }
        }

    }

    fn output_success(&self) -> Response {

        let mut payload = BTreeMap::new();
        payload.insert("success", true);

        let success = json::encode(&payload).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::Ok, success))

    }

    fn output_error(&self) -> Response {
        let mut error = BTreeMap::new();
        error.insert("error", true);

        let json_error = json::encode(&error).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Response::with((content_type, status::InternalServerError, json_error))

    }

    fn get_body(&self, req: &mut Request) -> Option<serde_json::Value> {

        match req.get::<bodyparser::Json>(){
            Ok(json) => Some(json.expect("unable to unwrap json")),
            _ => None
        }
    }

    fn get_cards(&self, player_move:serde_json::Value, reversed: bool) -> Vec<PlayerCard> {
            player_move
                .as_array()
                .unwrap()
                .iter()
                .map(|ref obj| {

            let obj = obj.as_object().unwrap();
            let suit = obj.get("suit").unwrap().as_str().unwrap();
            let rank = obj.get("rank").unwrap().as_str().unwrap();
            let joker = obj.get("joker").unwrap().as_bool().unwrap();
            self.get_card(rank, suit, joker, reversed)
        }).collect::<Vec<PlayerCard>>()

    }

   fn get_card(&self, rank:&str, suit:&str, joker: bool, reversed:bool) -> PlayerCard {

        let r = self.get_rank(&rank);
        let s = self.get_suit(&suit);
        let card = Card::new(r, s, reversed);

        if joker {
            PlayerCard::Wildcard(card)
        } else {
            PlayerCard::Card(card)
        }
    }


    fn get_rank(&self, rank:&str) -> Rank {
        match rank {
            "2"  => Rank::Two,
            "3"  => Rank::Three,
            "4"  => Rank::Four,
            "5"  => Rank::Five,
            "6"  => Rank::Six,
            "7"  => Rank::Seven,
            "8"  => Rank::Eight,
            "9"  => Rank::Nine,
            "10" => Rank::Ten,
            "J"  => Rank::Jack,
            "Q"  => Rank::Queen,
            "K"  => Rank::King,
            "A"  => Rank::Ace,
            _    => panic!("invalid rank supplied in move : {}", rank)
        }

    }

    fn get_suit(&self, suit:&str) -> Suit {
        match suit {
            "Clubs"    => Suit::Clubs,
            "Hearts"   => Suit::Hearts,
            "Diamonds" => Suit::Diamonds,
            "Spades"   => Suit::Spades,
            _          => panic!("invalid suit supplied in move")
        }
    }

}

impl Handler for SubmitMove{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref body = self.get_body(req);


        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);

        let resp = match session_user_id {
            Some(user_id) => {
                info!("valid user - checking game id");
                match *query {
                    Some(id) => {
                        self.execute(user_id, id.parse::<u64>().unwrap(), body.to_owned())
                    },
                    _ => {
                        info!("invalid id: {:?}", query);
                        self.output_error()
                    }
                }
            },
            _ => self.output_error()
        };

        Ok(resp)
    }

}
