use iron::prelude::*;
use iron::middleware::Handler;
use router::Router;
use urlencoded::UrlEncodedBody;
use std::collections::HashMap;

use config::Config;
use helpers;
use data_access::round::Round as RoundData;
use data_access::game::Game as GameData;
use pusoy_dos::game::game::Game;
use pusoy_dos::cards::types::*;
use pusoy_dos::cards::card::{ Card, PlayerCard };

pub struct GameMove{
    round_data: RoundData,
    game_data: GameData,
    hostname: String
}

impl GameMove{

    pub fn new(config:&Config, round_data: RoundData, game_data: GameData) -> GameMove {
        let hostname = config.get("hostname").unwrap();
        GameMove{ hostname: hostname, round_data: round_data, game_data: game_data }
    }

    fn execute(&self, user_id:u64, game_id:u64, hand: Vec<PlayerCard>) -> Response {
        let round_result = self.round_data.get(game_id);
        match round_result {
            None => {
                info!("redirecting as no round found for game {}", game_id);
                return helpers::redirect(&self.hostname, "games");  // think about an error page here?
            },
            _ => ()
        }

        info!("loading game: {}", game_id);

        let round = round_result.expect("error with round result");
        let game = Game::load(round.clone()).expect("error loading game");
        info!("game loaded");

        let valid_move = game.player_move(user_id, hand);
        let updated_game = valid_move.expect("error with move");
        self.round_data.update_round(game_id, updated_game.clone());

        let updated_round = updated_game.round.export();
        if updated_round.players.len() < 2 {
            let _ = self.game_data.complete_game(game_id);
        }

        let play_url = format!("play/{}", game_id);
        helpers::redirect(&self.hostname, &play_url)
    }

    fn get_hashmap(&self, req: &mut Request) -> Option<HashMap<String, Vec<String>>> {

        match req.get_ref::<UrlEncodedBody>(){
            Ok(hashmap) => Some(hashmap.to_owned().to_owned()),
            _ => None
        }
    }

    fn get_move(&self, hashmap: Option<HashMap<String, Vec<String>>>) -> Vec<PlayerCard>{
        let mut cards = vec!();

        match hashmap {
            Some(h) => {
                for(card, _) in h {
                    cards.push(self.get_card(card));
                }
            },
            _ => ()
        }

        cards
    }

    fn get_card(&self, card:String) -> PlayerCard {
        let words = card.split(" ").collect::<Vec<&str>>();
        let rank = self.get_rank(words[1]);
        let suit = self.get_suit(words[0]);
            
        let card = Card::new(rank, suit);

        PlayerCard::Card(card)
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
            _    => panic!("invalid rank supplied in move")
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

impl Handler for GameMove {


    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref hashmap = self.get_hashmap(req);

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        // TODO - take jokers into account
        let hand = self.get_move(hashmap.to_owned());

        
        info!("{:?}", hashmap);
        info!("{:?}", hand);

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.execute(user_id, id.parse::<u64>().unwrap(), hand)
                    },
                    _ => redirect_to_homepage
                }
            },
            _ => redirect_to_homepage
        };

        Ok(resp)
    }

}


