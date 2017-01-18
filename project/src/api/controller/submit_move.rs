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
use data_access::game::Game as GameData;
use data_access::user::User as UserData;

use pusoy_dos::game::game::Game;
use pusoy_dos::cards::types::*;
use pusoy_dos::cards::card::{ Card, PlayerCard };



#[derive(Clone)]
pub struct SubmitMove{
    round_data: RoundData,
    game_data: GameData
}

impl SubmitMove {

    pub fn new(round_data: RoundData, game_data: GameData) -> SubmitMove{
        SubmitMove{
            round_data: round_data,
            game_data: game_data
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
        info!("{:?}", player_move);

        let cards = self.get_cards(player_move);
        info!("{:?}", cards);

        let valid_move = game.player_move(user_id, cards.clone());

        match valid_move {
            Ok(updated_game) => {
                self.round_data.update_round(id, updated_game.clone());

                let updated_round = updated_game.round.export();
                if updated_round.players.len() < 2 {
                    let _ = self.game_data.complete_game(id);
                }        
            },
            _ => {
                info!("invalid_move! {:?}", cards);
                return self.output_error();
            }
        }
 
        self.output_error()
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
