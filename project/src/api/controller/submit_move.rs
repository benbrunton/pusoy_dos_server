use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use rustc_serialize::json;
use std::collections::BTreeMap;
use serde_json;
use serde_json::{Value, Map};

use helpers;
use helpers::DCard;
use bodyparser;

use data_access::round::Round as RoundData;
use data_access::user::User as UserData;

use pusoy_dos::game::game::Game;
use pusoy_dos::cards::types::*;
use pusoy_dos::cards::card::{ Card, PlayerCard };



#[derive(Clone)]
pub struct SubmitMove{
    round_data: RoundData
}

impl SubmitMove {

    pub fn new(round_data: RoundData) -> SubmitMove{
        SubmitMove{
            round_data: round_data
        }
    }

    pub fn execute(&self, user_id: u64, 
                        id: u64, 
                        json:Option<serde_json::Value>) -> Response {

        let player_move = json.unwrap();
        info!("{:?}", player_move);

        let cards = self.get_cards(player_move);
        info!("{:?}", cards);
        self.output_error()
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

    fn get_cards(&self, player_move:serde_json::Value) -> Vec<PlayerCard> {
            player_move
                .as_array()
                .unwrap()
                .iter()
                .map(|ref obj| {
    
            let obj = obj.as_object().unwrap();
            let suit = obj.get("suit").unwrap().as_str().unwrap();
            let rank = obj.get("rank").unwrap().as_str().unwrap();
            let joker = obj.get("joker").unwrap().as_bool().unwrap();
            self.get_card(rank, suit, joker, false)
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
