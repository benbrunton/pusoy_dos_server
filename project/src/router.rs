use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::middleware::session::NewSessionMiddleware;
use generic_handler::GenericHandler;
use model::Session;

pub fn get_router(
    dev_mode: bool,
    home_page_handler: GenericHandler,
    test_auth_handler: GenericHandler,
    game_list_handler: GenericHandler,
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

    let (chain, pipelines) = single_pipeline(new_pipeline().add(middleware).build());

    build_router(chain, pipelines, |route| {
        route.get("/").to_new_handler(home_page_handler);
        route.get("/games").to_new_handler(game_list_handler);
        
        if dev_mode {
            route.get("/test_auth").to_new_handler(test_auth_handler);
        }
    })
}