
use router::Router;

use data_access::round::Round as RoundData;
use data_access::game::Game as GameData;
use data_access::user::User as UserData;
use data_access::event::Event as EventData;

use data_access::notification::Notification as NotificationData;

use api::controller::{ 
    players,
    last_move,
    my_cards,
    submit_move,
    game_events,
    update_notifications,
    time_limit
};

pub fn new(round_data:RoundData, user_data:UserData, game_data:GameData, event_data:EventData, notification_data:NotificationData) -> Router {

    let mut router = Router::new();

    let players_controller = players::Players::new(round_data.clone(), user_data.clone(), event_data.clone());
    let last_move_controller = last_move::LastMove::new(round_data.clone());
    let my_cards_controller = my_cards::MyCards::new(round_data.clone());
    let submit_move_controller = submit_move::SubmitMove::new(
                                        round_data.clone(),
                                        game_data.clone(),
                                        event_data.clone(),
                                        user_data.clone(),
                                        notification_data.clone());
    let game_events_controller = game_events::GameEvents::new(event_data.clone());

    let update_notifications_controller = update_notifications::UpdateNotifications::new(notification_data.clone());
    let time_limit_controller = time_limit::TimeLimit::new(event_data.clone(), game_data.clone());

    router.get("/players/:id", players_controller, "api_players");
    router.get("/last-move/:id", last_move_controller, "api_last_move");
    router.get("/my-cards/:id", my_cards_controller, "api_my_cards");
    router.post("/submit-move/:id", submit_move_controller, "api_submit_move");
    router.get("/game-events/:id", game_events_controller, "api_game_events");
    router.get("/time-limit/:id", time_limit_controller, "api_time_limit");

    router.post("/update-notifications", update_notifications_controller, "api_update_notifications");
    router
}
