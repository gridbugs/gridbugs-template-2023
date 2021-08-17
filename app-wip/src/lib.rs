use chargrid::{control_flow::*, core::*};
use orbital_decay_game::Config;
use rand::SeedableRng;
use rand_isaac::Isaac64Rng;

mod colours;
mod controls;
mod game;
mod game_loop;
mod menu;
mod stars;
mod tile_3x3;

struct AppState {
    game_loop_state: game_loop::GameLoopState,
}

pub fn app() -> impl Component<Output = app::Output, State = ()> {
    let config = Config {
        omniscient: None,
        demo: false,
        debug: true,
    };
    let mut rng = Isaac64Rng::from_entropy();
    let (game_loop_state, running) = game_loop::GameLoopState::new(config, &mut rng);
    let state = AppState { game_loop_state };
    game_loop::game_loop_component(running)
        .lens_state(lens!(AppState[game_loop_state]: game_loop::GameLoopState))
        .map(|_| app::Exit)
        .with_state(state)
        .clear_each_frame()
        .exit_on_close()
}