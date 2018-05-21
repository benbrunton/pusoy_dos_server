use tera::{Tera, Context};
use std::panic::RefUnwindSafe;
use data_access::round::Round as RoundData;
use data_access::user::User as UserData;
use config::Config;
use helpers;
use helpers::PathExtractor;
use pusoy_dos::game::game::Game;
use pusoy_dos::game::player_move::{Move, Trick};
use pusoy_dos::cards::card::PlayerCard;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use controller::{ResponseType, Controller};
use model::Session;

pub struct InPlayController{
    tera: &'static Tera,
    round_data: RoundData,
    hostname: String,
    user_data: UserData
}

impl InPlayController {
    
    pub fn new(config: &Config, 
                tera:&'static Tera, 
                round_data: RoundData, 
                user_data: UserData) -> InPlayController {

        let hostname = config.get("pd_host").unwrap();

        InPlayController{
            tera: tera,
            round_data: round_data,
            hostname: hostname,
            user_data: user_data
        }
    }

    pub fn get_page_response(&self, user_id:u64, game_id:u64) -> ResponseType {

        let template = "inplay.html";
        let mut data = Context::new();
        let round_result = self.round_data.get(game_id);

        match round_result {
            None => {
                info!("redirecting as no round found for game {}", game_id);
                return ResponseType::Redirect("/games".to_string());  // think about an error page here?
            },
            _ => ()
        }

        info!("loading game : {}", game_id);
        let round = round_result.expect("failed to load round");
        let game = Game::load(round.clone()).expect("game failed to load");

        let round_def = round.round.export();
        info!("round_def: {:?}", round_def);
        if round_def.players.len() < 2 {
            info!("GAME OVER FOR GAME: {}", game_id);
            return ResponseType::Redirect(format!("game-complete/{}", game_id));
        }

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
        let mut user_name = "unknown";

        for player in players.iter(){
            if player.id == next_player_id {
                next_player_name = &player.name;
            }
            if player.id == user_id {
                user_name = &player.name;
            }
        }

        let reversed = round.reversed;

        data.add("user_id", &user_id);
        data.add("user_name", &user_name);
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
        data.add("round_reversed", &reversed);

        let page = self.tera.render(template, &data).expect("unable to unwrap inplay page");
        ResponseType::PageResponse(page)
    }

    fn convert_move_to_cards(&self, last_move:Move) -> Vec<DCard> {
        match last_move {
            Move::Pass                  => vec!(),
            Move::Single(card)          => vec!(DCard(PlayerCard::Card(card))),
            Move::Pair(c1, c2)          => vec!(DCard(PlayerCard::Card(c1)), 
                                                DCard(PlayerCard::Card(c2))),
            Move::Prial(c1, c2, c3)     => vec!(DCard(PlayerCard::Card(c1)), 
                                                DCard(PlayerCard::Card(c2)), 
                                                DCard(PlayerCard::Card(c3))),
            Move::FiveCardTrick(trick)  => self.trick_to_cards(trick)
        }
        
    }

    fn trick_to_cards(&self, trick:Trick) -> Vec<DCard> {
        let c = trick.cards;
        vec!(DCard(PlayerCard::Card(c[0])),
            DCard(PlayerCard::Card(c[1])),
            DCard(PlayerCard::Card(c[2])),
            DCard(PlayerCard::Card(c[3])),
            DCard(PlayerCard::Card(c[4])))
    }
}

impl Controller for InPlayController {
    fn get_response(
        &self,
        session:&mut Option<Session>,
        _body: Option<String>,
        path: Option<PathExtractor>
    ) -> ResponseType {
        if helpers::is_logged_in(session) {
            let id = helpers::get_user_id(session).expect("no user id") as u64;
            let path_id = path.expect("no_path").id as u64;
            self.get_page_response(id, path_id)
        } else {
           ResponseType::Redirect("/".to_string())
        }
    }
}


// todo - move
//
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct DCard(PlayerCard);

impl Serialize for DCard {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {

        let card = self.0;
        let (rank, suit, suit_display, joker) = match card {
            PlayerCard::Card(c) |
            PlayerCard::Wildcard(c)  => (format!("{}", c.rank), 
                                    format!("{:?}", c.suit),
                                    format!("{}", c.suit),
                                    false),
            PlayerCard::Joker(n)    => (String::from(""), 
                                    format!("joker {}", n),
                                    String::from("joker"),
                                    true)
        };

        let mut state = try!(serializer.serialize_map(Some(4)));
		try!(state.serialize_entry("suit_display", &suit_display));
		try!(state.serialize_entry("suit", &suit));
		try!(state.serialize_entry("rank", &rank));
        try!(state.serialize_entry("joker", &joker));

        state.end()
    }
}

impl RefUnwindSafe for InPlayController {}
