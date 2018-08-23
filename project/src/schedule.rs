use schedule_recv::periodic_ms;
use std::thread;
use data_access::game::Game;
use data_access::round::Round;
use data_access::event::Event;
use util::move_limit_task;

pub fn run(game_data: Game, event_data: Event, round_data:Round) {
    info!("setting up scheduled jobs..");
    let tick = periodic_ms(10000);

    let handle = thread::spawn(move || loop {
        tick.recv().expect("failed to receive tick period");

        move_limit_task::execute(game_data.clone(), event_data.clone(), round_data.clone());
    });
}

