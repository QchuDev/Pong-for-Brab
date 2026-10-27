// player.rs
use crate::kinematics::*;
use bevy::prelude::*;

//
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_pad);
    }
}

// Atlas constants
#[derive(Component)]
pub struct Player; // Moved from main.rs to here

// Control de players pad
pub fn move_pad(
    input: Res<ButtonInput<KeyCode>>,
    player_kinematic: Single<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (_transform, mut vel) = player_kinematic.into_inner();

    let mut dir = Vec2::ZERO;

    if input.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }

    if input.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }

    if dir != Vec2::ZERO {
        let speed = 100.0;

        let delta = speed * dir.normalize();

        vel.x = delta.x;
        vel.y = delta.y;
    } else {
        vel.x = 0.0;
        vel.y = 0.0;
    }
}
