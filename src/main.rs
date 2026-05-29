use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    my_current_score: u32,
    highest_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            my_current_score: Default::default(),
            highest_score: Default::default(),
            enemy_labels: Default::default(),
            spawn_timer: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }
}

fn main() {
    let mut game = Game::default();
    let mut game_state = GameState::new();
    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    game_state.current_score += 1;
    let _ = engine.add_sprite("my_player", SpritePreset::RacingCarBlue);
    println!("Current score: {}", game_state.current_score);
}
