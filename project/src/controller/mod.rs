mod home_page;
mod game_list;
mod logout;
mod new_game;
mod game_create;
mod game;
mod game_join;
mod begin_game;
mod inplay;
mod players;
mod last_move;
mod my_cards;
mod submit_move;
mod time_limit;
mod update_notifications;
mod fb_auth;
mod about;
mod privacy;
mod post_game;
mod remove_user;
mod leaderboard;

/*
mod google_auth;
mod update_game;
mod complete_games;
*/

mod test_auth;
mod controller;

pub use self::home_page::HomePageController;
pub use self::game_list::GameListController;
pub use self::test_auth::TestAuthController;
pub use self::logout::LogoutController;
pub use self::new_game::NewGameController;
pub use self::game_create::GameCreateController;
pub use self::game::GameController;
pub use self::game_join::GameJoinController;
pub use self::begin_game::BeginGameController;
pub use self::inplay::InPlayController;
pub use self::players::PlayersController;
pub use self::last_move::LastMoveController;
pub use self::my_cards::MyCardsController;
pub use self::submit_move::SubmitMoveController;
pub use self::time_limit::TimeLimitController;
pub use self::update_notifications::UpdateNotificationsController;
pub use self::fb_auth::FacebookAuthController;
pub use self::about::AboutController;
pub use self::privacy::PrivacyController;
pub use self::post_game::PostGameController;
pub use self::remove_user::RemoveUserController;
pub use self::leaderboard::LeaderboardController;

pub use self::controller::{Controller, ResponseType};
