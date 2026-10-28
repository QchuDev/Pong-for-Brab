// player.rs
use avian2d::prelude::*;
use bevy::prelude::*;

// Atlas constants
const MOVE_SPEED: f32 = 200.0; // pixels per second

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_player);
    }
}

#[derive(Component)]
pub struct Player; // Moved from main.rs to here

// Control de players pad
pub fn move_player(
    player: Single<&mut LinearVelocity, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut y_dir = 0.0;

    if input.pressed(KeyCode::ArrowUp) {
        y_dir += 1.0;
    }

    if input.pressed(KeyCode::ArrowDown) {
        y_dir -= 1.0;
    }

    let mut player_vel = player.into_inner();
    player_vel.y = MOVE_SPEED * y_dir;
}
