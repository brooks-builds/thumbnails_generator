use grokking_algorithms::GameState;
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

fn main() {
    let mut game_state = GameState::new((WIDTH, HEIGHT));
    bbggez::run::run_dim(
        &mut game_state,
        WIDTH,
        HEIGHT,
        "Grokking Algorithms Thumbnail Generator",
        "Brooks Patton",
    );
}
