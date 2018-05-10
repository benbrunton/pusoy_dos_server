use pusoy_dos::game::player_move::{Move, Trick, TrickType, build_move};
use pusoy_dos::cards::card::PlayerCard;
use serde::ser::{Serialize, Serializer, SerializeMap};

use model::Session;

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

pub fn is_logged_in(session: &mut Option<Session>) -> bool {
    let sess_clone = session.clone();
    match sess_clone {
        Some(sess) => sess.user_id != None,
        None       => false
    }
}

pub fn get_user_id(session: &mut Option<Session>) -> Option<usize> {
    let sess_clone = session.clone();
    match sess_clone {
        Some(sess) => sess.user_id,
        None       => None
    }
}

