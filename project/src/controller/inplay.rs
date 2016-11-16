use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::mime::Mime;
use router::Router;

use tera::{Tera, Context};
use data_access::round::Round as RoundData;
use data_access::user::User as UserData;
use config::Config;
use helpers;
use pusoy_dos::game::game::Game;
use pusoy_dos::game::player_move::{Move, Trick};
use pusoy_dos::cards::card::Card;
use serde::{Serialize, Serializer};

pub struct InPlay{
    tera: &'static Tera,
    round_data: RoundData,
    hostname: String,
    user_data: UserData
}

impl InPlay {
    
    pub fn new(config: &Config, 
                tera:&'static Tera, 
                round_data: RoundData, 
                user_data: UserData) -> InPlay {

        let hostname = config.get("hostname").unwrap();

        InPlay{
            tera: tera,
            round_data: round_data,
            hostname: hostname,
            user_data: user_data
        }
    }

    pub fn display(&self, user_id:u64, game_id:u64) -> Response {

        let template = "inplay.html";
        let mut data = Context::new();
        let round_result = self.round_data.get(game_id);

        match round_result {
            None => {
                info!("redirecting as no round found for game {}", game_id);
                return helpers::redirect(&self.hostname, "games");  // think about an error page here?
            },
            _ => ()
        }

        info!("loading game : {}", game_id);
        let round = round_result.expect("failed to load round");
        let game = Game::load(round.clone()).expect("game failed to load");
        let next_player = game.get_next_player().expect("unable to get next player");

        let next_player_id = next_player.get_id();

        let current_user_turn = user_id == next_player_id; 
        let current_user = game.get_player(user_id).unwrap();

        let mut cards:Vec<DCard> = current_user.get_hand().iter().map(|&c|{ DCard(c.clone()) }).collect();
        cards.sort();
        cards.reverse();

        let current_user_winner = match round.winners.clone().first() {
            Some(id) => {
                info!("current_winner: {}", id);
                *id == user_id
            },
            _        => {
                info!("no winners!");
                false
            }
        };

        let mut sorted_winners = round.winners.clone();
        sorted_winners.sort();

        let current_user_finished = match sorted_winners.binary_search(&user_id){
            Ok(_) => true,
            _     => false
        };

        let last_move = round.clone().round.get_last_move();
        let display_last_move = self.convert_move_to_cards(last_move);

        let players = self.user_data.get_users_by_game(game_id);
        let mut next_player_name = "unknown";

        for player in players.iter(){
            if player.id == next_player_id {
                next_player_name = &player.name;
            }
        }

        data.add("user_id", &user_id);
        data.add("logged_in", &true);
        data.add("current_user_winner", &current_user_winner);
        data.add("current_user_finished", &current_user_finished);
        data.add("your_turn", &current_user_turn);
        data.add("next_player", &next_player_id);
        data.add("next_player_name", &next_player_name);
        data.add("id", &game_id);
        data.add("cards", &cards);
        data.add("last_move", &display_last_move);
        data.add("players", &players);

        let content_type = "text/html".parse::<Mime>().unwrap();
        let page = self.tera.render(template, data).unwrap();
        Response::with((content_type, status::Ok, page))
    }

    fn convert_move_to_cards(&self, last_move:Move) -> Vec<DCard> {
        match last_move {
            Move::Pass                    => vec!(),
            Move::Single(card)            => vec!(DCard(card)),
            Move::Pair(c1, c2)            => vec!(DCard(c1), DCard(c2)),
            Move::Prial(c1, c2, c3)       => vec!(DCard(c1), DCard(c2), DCard(c3)),
            Move::FiveCardTrick(trick)    => self.trick_to_cards(trick)
        }
        
    }

    fn trick_to_cards(&self, trick:Trick) -> Vec<DCard> {
        let c = trick.cards;
        vec!(DCard(c[0]),
            DCard(c[1]),
            DCard(c[2]),
            DCard(c[3]),
            DCard(c[4]))
    }
}

impl Handler for InPlay {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.display(user_id, id.parse::<u64>().unwrap())
                    },
                    _ => redirect_to_homepage
                }
            },
            _ => redirect_to_homepage
        };

        Ok(resp)
    }

}

// todo - move
//
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct DCard(Card);

impl Serialize for DCard {

	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {

        let mut state = try!(serializer.serialize_map(Some(2)));
		try!(serializer.serialize_map_key(&mut state, "suit_display"));
		try!(serializer.serialize_map_value(&mut state, format!("{}", self.0.suit)));
		try!(serializer.serialize_map_key(&mut state, "suit"));
		try!(serializer.serialize_map_value(&mut state, format!("{:?}", self.0.suit)));
		try!(serializer.serialize_map_key(&mut state, "rank"));
		try!(serializer.serialize_map_value(&mut state, format!("{}", self.0.rank)));
        serializer.serialize_map_end(state)
    }
}
