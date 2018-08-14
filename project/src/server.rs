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
    BeginGameController,
    InPlayController,
    PlayersController,
    LastMoveController,
    MyCardsController,
    SubmitMoveController,
    TimeLimitController,
    UpdateNotificationsController,
    FacebookAuthController,
    AboutController,
    PrivacyController,
    PostGameController,
};
use data_access::user::User;
use data_access::game::Game;
use data_access::round::Round;
use data_access::event::Event;
use data_access::notification::Notification;
use handlers::{GenericHandler, PathHandler, QueryStringHandler};
use std::sync::Arc;

pub fn run(
    port: u16,
    config: &Config,
    tera: &'static Tera, 
    user_data: User,
    game_data: Game,
    round_data: Round,
    event_data: Event,
    notification_data: Notification,
    ) {

    let home_page_controller = HomePageController::new(&config, &tera);
    let test_auth_controller = TestAuthController::new(&config, user_data.clone());
    let game_list_controller = GameListController::new(&config, &tera, game_data.clone());
    let logout_controller = LogoutController::new(&config);
    let new_game_controller = NewGameController::new(&tera);
    let game_create_controller = GameCreateController::new(game_data.clone());
    let game_controller = GameController::new(&config, &tera, game_data.clone(), user_data.clone());
    let game_join_controller = GameJoinController::new(&config, game_data.clone());
    let begin_game_controller = BeginGameController::new(
        &config,
        game_data.clone(),
        round_data.clone()
    );
    let inplay_controller = InPlayController::new(
        &config,
        &tera,
        round_data.clone(),
        user_data.clone()
    );

    let players_controller = PlayersController::new(
        round_data.clone(),
        user_data.clone(),
        event_data.clone()
    );

    let last_move_controller = LastMoveController::new(
        round_data.clone(),
    );

    let my_cards_controller = MyCardsController::new(
        round_data.clone(),
    );

    let submit_move_controller = SubmitMoveController::new(
        round_data.clone(),
        game_data.clone(),
        event_data.clone(),
        user_data.clone(),
        notification_data.clone(),
    );

    let time_limit_controller = TimeLimitController::new(
        event_data.clone(),
        game_data.clone(),
    );

    let update_notifications_controller = UpdateNotificationsController::new(
        notification_data.clone(),
    );

    let fb_auth_controller = FacebookAuthController::new(
        &config,
        user_data.clone(),
    );

    let about_controller = AboutController::new(
        &tera
    );

    let privacy_controller = PrivacyController::new(
        &tera
    );

    let post_game_controller = PostGameController::new(
        &tera,
        event_data.clone(),
    );

    let home_page_handler = GenericHandler::new(Arc::new(home_page_controller));
    let test_auth_handler = GenericHandler::new(Arc::new(test_auth_controller));
    let game_list_handler = GenericHandler::new(Arc::new(game_list_controller));
    let logout_handler = GenericHandler::new(Arc::new(logout_controller));
    let new_game_handler = GenericHandler::new(Arc::new(new_game_controller));
    let game_create_handler = GenericHandler::new(Arc::new(game_create_controller));
    let game_handler = PathHandler::new(Arc::new(game_controller));
    let game_join_handler = PathHandler::new(Arc::new(game_join_controller));
    let begin_game_handler = PathHandler::new(Arc::new(begin_game_controller));
    let inplay_handler = PathHandler::new(Arc::new(inplay_controller));
    let players_handler = PathHandler::new(Arc::new(players_controller));
    let last_move_handler = PathHandler::new(Arc::new(last_move_controller));
    let my_cards_handler = PathHandler::new(Arc::new(my_cards_controller));
    let submit_move_handler = PathHandler::new(Arc::new(submit_move_controller));
    let time_limit_handler = PathHandler::new(Arc::new(time_limit_controller));
    let update_notifications_handler = GenericHandler::new(Arc::new(update_notifications_controller));
    let fb_auth_handler = QueryStringHandler::new(Arc::new(fb_auth_controller));
    let about_handler = GenericHandler::new(Arc::new(about_controller));
    let privacy_handler = GenericHandler::new(Arc::new(privacy_controller));
    let post_game_handler = PathHandler::new(Arc::new(post_game_controller));

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
        begin_game_handler,
        inplay_handler,
        players_handler,
        last_move_handler,
        my_cards_handler,
        submit_move_handler,
        time_limit_handler,
        update_notifications_handler,
        fb_auth_handler,
        about_handler,
        privacy_handler,
        post_game_handler,
    );

    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router);
}
