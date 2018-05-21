mod home_page;
mod game_list;
mod logout;
mod new_game;
mod game_create;
mod game;
mod game_join;
mod begin_game;
mod inplay;
/*
mod fb_auth;
mod google_auth;
mod game_move;
mod post_game;
mod leaderboard;
mod about;
mod remove_user;
mod update_game;
mod complete_games;
mod privacy;
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

pub use self::controller::{Controller, ResponseType};
