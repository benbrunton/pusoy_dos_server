use gotham;
use router;

use tera::Tera;
use config::Config;
use controller::HomePageController;

pub fn run(port: u16, config: &Config, tera: &'static Tera) {
    let home_page_controller = HomePageController::new(&config, &tera);
    let router = router::get_router(home_page_controller);
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router);
}
