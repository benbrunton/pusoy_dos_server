use gotham;
use router;

use tera::Tera;
use config::Config;
use controller::{
    HomePageController,
    TestAuthController
};
use data_access::user::UserData;

pub fn run(
    port: u16,
    config: &Config,
    tera: &'static Tera, 
    user_data: &UserData,
    ) {
    let home_page_controller = HomePageController::new(&config, &tera);
    let test_auth_controller = TestAuthController::new(&config, user_data.clone());
    let router = router::get_router(home_page_controller);
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router);
}
