use gotham;
use router;

use tera::Tera;
use config::Config;
use controller::{
    HomePageController,
    TestAuthController,
    GameListController,
    LogoutController,
    NewGameController,
    GameCreateController,
    GameController,
    GameJoinController,
};
use data_access::user::User;
use data_access::game::Game;
use handlers::{GenericHandler, PathHandler};
use std::sync::Arc;

pub fn run(
    port: u16,
    config: &Config,
    tera: &'static Tera, 
    user_data: User,
    game_data: Game,
    ) {

    let home_page_controller = HomePageController::new(&config, &tera);
    let test_auth_controller = TestAuthController::new(&config, user_data.clone());
    let game_list_controller = GameListController::new(&config, &tera, game_data.clone());
    let logout_controller = LogoutController::new(&config);
    let new_game_controller = NewGameController::new(&tera);
    let game_create_controller = GameCreateController::new(game_data.clone());
    let game_controller = GameController::new(&config, &tera, game_data.clone(), user_data.clone());
    let game_join_controller = GameJoinController::new(&config, game_data.clone());

    let home_page_handler = GenericHandler::new(Arc::new(home_page_controller));
    let test_auth_handler = GenericHandler::new(Arc::new(test_auth_controller));
    let game_list_handler = GenericHandler::new(Arc::new(game_list_controller));
    let logout_handler = GenericHandler::new(Arc::new(logout_controller));
    let new_game_handler = GenericHandler::new(Arc::new(new_game_controller));
    let game_create_handler = GenericHandler::new(Arc::new(game_create_controller));
    let game_handler = PathHandler::new(Arc::new(game_controller));
    let game_join_handler = PathHandler::new(Arc::new(game_join_controller));

    let dev_mode = match config.get("mode") {
        Some(mode) => mode == "dev",
        _          => false
    };

    let router = router::get_router(
        dev_mode,
        home_page_handler,
        test_auth_handler,
        game_list_handler,
        logout_handler,
        new_game_handler,
        game_create_handler,
        game_handler,
        game_join_handler,
    );
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router);
}
