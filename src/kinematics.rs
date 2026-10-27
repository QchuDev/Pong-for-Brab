use bevy::prelude::*;

pub struct KinematicsPlugin;
impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_positions);
    }
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// Its a system, it needs to run for ALL queried results
pub fn update_positions(mut kinematic: Query<(&mut Velocity, &mut Transform)>, timer: Res<Time>) {
    for (vel, mut transform) in &mut kinematic {
        transform.translation.x += timer.delta_secs() * vel.x;
        transform.translation.y += timer.delta_secs() * vel.y;
    }
}
