use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::middleware::session::NewSessionMiddleware;
use handlers::{GenericHandler, PathHandler};
use model::Session;
use helpers::{QueryStringExtractor, PathExtractor};
use middleware::MiddlewareAddingResponseHeader;

pub fn get_router(
    dev_mode: bool,
    home_page_handler: GenericHandler,
    test_auth_handler: GenericHandler,
    game_list_handler: GenericHandler,
    logout_handler: GenericHandler,
    new_game_handler: GenericHandler,
    game_create_handler: GenericHandler,
    game_handler: PathHandler,
    game_join_handler: PathHandler,
    begin_game_handler: PathHandler,
    inplay_handler: PathHandler,
    players_handler: PathHandler,
    last_move_handler: PathHandler,
    my_cards_handler: PathHandler,
    submit_move_handler: PathHandler,
    time_limit_handler: PathHandler,
    update_notifications_handler: GenericHandler,
    fb_auth_handler: GenericHandler,
) -> Router {

    // Install middleware which handles session creation before, and updating after, our handler is
    // called.
    // The default NewSessionMiddleware stores session data in an in-memory map, which means that
    // server restarts will throw the data away, but it can be customized as needed.
    let middleware = NewSessionMiddleware::default()
        // Configure the type of data which we want to store in the session.
        // See the custom_data_type example for storing more complex data.
        .with_session_type::<Option<Session>>()
        .insecure(); // TODO: remove

    let (chain, pipelines) = single_pipeline(new_pipeline()
        .add(middleware)
        .add(MiddlewareAddingResponseHeader)
        .build());

    build_router(chain, pipelines, |route| {
        route.get("/").to_new_handler(home_page_handler);
        route.get("/games").to_new_handler(game_list_handler);
        route.get("/logout").to_new_handler(logout_handler);
        route.get("/new-game").to_new_handler(new_game_handler);
        route.get("/fb-auth")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to_new_handler(fb_auth_handler);
        route.get("/game/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(game_handler);
        route.get("/play/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(inplay_handler);

        route.post("/new-game").to_new_handler(game_create_handler);
        route.post("/game/:id:[0-9]+/join")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(game_join_handler);
        route.post("/game/:id:[0-9]+/begin")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(begin_game_handler);

        // json endpoints
        route.get("/api/v1/players/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(players_handler);
        route.get("/api/v1/last-move/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(last_move_handler);
        route.get("/api/v1/my-cards/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(my_cards_handler);
        route.post("/api/v1/submit-move/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(submit_move_handler);
        route.get("/api/v1/time-limit/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to_new_handler(time_limit_handler);
        route.post("/api/v1/update-notifications")
            .to_new_handler(update_notifications_handler);

        if dev_mode {
            route.get("/test_auth").to_new_handler(test_auth_handler);
        }
    })
}
