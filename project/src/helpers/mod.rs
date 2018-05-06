use std::fmt::Display;
/*
use iron::prelude::*;
use iron::{status, modifiers, Url};
use iron::mime::Mime;
*/
use tera::TeraResult;
use pusoy_dos::game::player_move::{Move, Trick, TrickType, build_move};
use pusoy_dos::cards::card::PlayerCard;
use serde::ser::{Serialize, Serializer, SerializeMap};

use hyper::{Response, StatusCode};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{Handler, HandlerFuture};
use futures::{future, Future};
use mime;

//use util::session::Session;

/*
pub fn get_user_id(req: &Request) -> Option<u64> {

    match req.extensions.get::<Session>() {
        Some(session) => session.user_id,
        _             => None
    }

}
*/

pub fn redirect<S: Display>(mut state: State, hostname:&str, path:S) -> Box<HandlerFuture>{

/*
    let full_url = format!("{}/{}", hostname, path);
    let url =  Url::parse(&full_url).unwrap();

    Response::with((status::Found, modifiers::Redirect(url)))
*/
        let res = {
            create_response(
                &state,
                StatusCode::Ok,
                Some((
					"redirect".as_bytes()
                        .to_vec(),
                    mime::TEXT_PLAIN,
                )),
            )
        };


        Box::new(future::ok((state, res)))

}

pub fn render(mut state: State, result: TeraResult<String>) -> Box<HandlerFuture>{
/*
    let content_type = "text/html".parse::<Mime>().unwrap();
    Response::with((content_type, status::Ok, result.unwrap()))
*/
    let res = {
        create_response(
            &state,
            StatusCode::Ok,
            Some((
                "render".as_bytes()
                    .to_vec(),
                mime::TEXT_PLAIN,
            )),
        )
    };

    Box::new(future::ok((state, res)))

}

pub fn cards_played_summary(last_move: Vec<PlayerCard>) -> String {
    match build_move(last_move).unwrap() {
        Move::Pass                  => "Pass".to_owned(),
        Move::Single(card)          => format!("a {}{}", card.rank, card.suit),
        Move::Pair(card, _)         => format!("a pair of {}s", card.rank),
        Move::Prial(card, _, _)     => format!("a prail of {}s", card.rank),
        Move::FiveCardTrick(trick)  => match trick.trick_type {
            TrickType::Straight => "a straight".to_owned(),
            TrickType::Flush => "a flush".to_owned(),
            TrickType::FullHouse => "a full house".to_owned(),
            TrickType::FourOfAKind => "four of a kind".to_owned(),
            TrickType::StraightFlush => "a straight flush".to_owned(),
            TrickType::FiveOfAKind => "five of a kind".to_owned()
        }
    }
}

pub fn convert_move_to_display_cards(last_move:Move) -> Vec<DCard> {
    match last_move {
        Move::Pass                  => vec!(),
        Move::Single(card)          => vec!(DCard(PlayerCard::Card(card))),
        Move::Pair(c1, c2)          => vec!(DCard(PlayerCard::Card(c1)),
                                            DCard(PlayerCard::Card(c2))),
        Move::Prial(c1, c2, c3)     => vec!(DCard(PlayerCard::Card(c1)),
                                            DCard(PlayerCard::Card(c2)),
                                            DCard(PlayerCard::Card(c3))),
        Move::FiveCardTrick(trick)  => trick_to_cards(trick)
    }

}

pub fn convert_vec_to_display_cards(card_vec:Vec<PlayerCard>) -> Vec<DCard> {
    card_vec.iter().map(|ref c|{
        let card = c.to_owned();
        DCard(*card)
    }).collect()
}

fn trick_to_cards(trick:Trick) -> Vec<DCard> {
    let c = trick.cards;
    vec!(DCard(PlayerCard::Card(c[0])),
        DCard(PlayerCard::Card(c[1])),
        DCard(PlayerCard::Card(c[2])),
        DCard(PlayerCard::Card(c[3])),
        DCard(PlayerCard::Card(c[4])))
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DCard(PlayerCard);

impl DCard {
    pub fn new(c: PlayerCard) -> DCard {
        DCard(c)
    }
}

impl Serialize for DCard {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {

        let card = self.0;
        let (rank, suit, suit_display, joker) = match card {
            PlayerCard::Card(c) => (format!("{}", c.rank),
                                    format!("{:?}", c.suit),
                                    format!("{}", c.suit),
                                    false),
            PlayerCard::Wildcard(c)  => (format!("{}", c.rank),
                                    format!("{:?}", c.suit),
                                    format!("{}", c.suit),
                                    true),
            PlayerCard::Joker(n)    => (String::from(""),
                                    format!("joker {}", n),
                                    String::from("🃏"),
                                    true)
        };

        let mut map = try!(serializer.serialize_map(Some(2)));
		try!(map.serialize_entry("suit_display", &suit_display));
        try!(map.serialize_entry("suitDisplay", &suit_display));
		try!(map.serialize_entry("suit", &suit));
		try!(map.serialize_entry("rank", &rank));
        try!(map.serialize_entry("joker", &joker));

        map.end()
    }
}
