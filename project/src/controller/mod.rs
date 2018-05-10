mod home_page;
mod game_list;
/*
mod fb_auth;
mod google_auth;
mod game;
mod game_list;
mod game_create;
mod game_join;
mod new_game;
mod logout;
mod begin_game;
mod inplay;
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

pub use self::controller::{Controller, ResponseType};
